use std::any::Any;
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock, mpsc};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum ComModel {
    STA,
    MTA,
}

/// Helper for `with_com` macro to ensure COM cleanup
pub struct ComGuard;

impl Drop for ComGuard {
    fn drop(&mut self) {
        unsafe {
            windows::Win32::System::Com::CoUninitialize();
        }
    }
}

/// Initialize COM and return a guard that will uninitialize on drop.
///
/// # Safety
/// This function calls CoInitializeEx internally.
pub unsafe fn init_com(model: ComModel) -> ComGuard {
    use windows::Win32::System::Com::{
        COINIT_APARTMENTTHREADED, COINIT_MULTITHREADED, CoInitializeEx,
    };
    let mode = match model {
        ComModel::STA => COINIT_APARTMENTTHREADED,
        ComModel::MTA => COINIT_MULTITHREADED,
    };
    unsafe {
        let _ = CoInitializeEx(None, mode);
    }
    ComGuard
}

/// Re-export block_on for macro usage
pub fn block_on<F: std::future::Future>(future: F) -> F::Output {
    futures::executor::block_on(future)
}

trait Task: Send {
    fn run(self: Box<Self>) -> Box<dyn Any + Send>;
}

struct TaskImpl<F>(Option<F>);

impl<F, R> Task for TaskImpl<F>
where
    F: FnOnce() -> R + Send + 'static,
    R: Any + Send + 'static,
{
    fn run(self: Box<Self>) -> Box<dyn Any + Send> {
        let this = *self;
        let f = this.0.expect("task already taken");
        let r = f();
        Box::new(r)
    }
}

enum Message {
    Sync(Box<dyn Task>, std::sync::mpsc::Sender<Box<dyn Any + Send>>),
    Async(
        Box<dyn Task>,
        futures::channel::oneshot::Sender<Box<dyn Any + Send>>,
    ),
}

static THREAD_MAP: OnceLock<Mutex<HashMap<ComModel, mpsc::Sender<Message>>>> = OnceLock::new();

fn ensure_sender(model: ComModel) -> mpsc::Sender<Message> {
    let map_mutex = THREAD_MAP.get_or_init(|| Mutex::new(HashMap::new()));
    let mut map = map_mutex.lock().unwrap();
    if let Some(s) = map.get(&model) {
        return s.clone();
    }

    let (s, r) = mpsc::channel::<Message>();

    // spawn background thread
    std::thread::spawn(move || {
        // initialize COM for this thread if windows is available
        #[allow(unused_unsafe)]
        unsafe {
            let _ = windows::Win32::System::Com::CoInitializeEx(
                None,
                match model {
                    ComModel::MTA => windows::Win32::System::Com::COINIT_MULTITHREADED,
                    ComModel::STA => windows::Win32::System::Com::COINIT_APARTMENTTHREADED,
                },
            );
        }

        struct ComGuard;
        impl Drop for ComGuard {
            fn drop(&mut self) {
                unsafe {
                    windows::Win32::System::Com::CoUninitialize();
                }
            }
        }
        let _guard = ComGuard;

        for msg in r {
            match msg {
                Message::Sync(task, resp_tx) => {
                    let res = task.run();
                    let _ = resp_tx.send(res);
                }
                Message::Async(task, resp_tx) => {
                    let res = task.run();
                    let _ = resp_tx.send(res);
                }
            }
        }
        // thread ends when receiver is closed
    });

    map.insert(model, s.clone());
    s
}

pub fn call_sync<F, R>(model: ComModel, f: F) -> R
where
    F: FnOnce() -> R + Send + 'static,
    R: Any + Send + 'static,
{
    let (resp_tx, resp_rx) = std::sync::mpsc::channel::<Box<dyn Any + Send>>();
    let task: Box<dyn Task> = Box::new(TaskImpl(Some(f)));

    // If the background thread has exited the receiver will be closed and
    // send will return Err(msg). In that case we retry once by acquiring a
    // fresh sender from `ensure_sender` and resending the message.
    let mut msg = Message::Sync(task, resp_tx);
    let mut sent = false;
    for _ in 0..2 {
        let sender = ensure_sender(model);
        match sender.send(msg) {
            Ok(()) => {
                sent = true;
                break;
            }
            Err(e) => {
                // take back ownership of the message and retry
                msg = e.0;
            }
        }
    }
    if !sent {
        panic!("failed to send task to COM thread after retry");
    }

    let boxed = resp_rx.recv().expect("COM thread closed");
    *boxed
        .downcast::<R>()
        .expect("type mismatch in runtime result")
}

pub fn call_async<F, R>(model: ComModel, f: F) -> impl std::future::Future<Output = R>
where
    F: FnOnce() -> R + Send + 'static,
    R: Any + Send + 'static,
{
    let (resp_tx, resp_rx) = futures::channel::oneshot::channel::<Box<dyn Any + Send>>();
    let task: Box<dyn Task> = Box::new(TaskImpl(Some(f)));

    let mut msg = Message::Async(task, resp_tx);
    let mut sent = false;
    for _ in 0..2 {
        let sender = ensure_sender(model);
        match sender.send(msg) {
            Ok(()) => {
                sent = true;
                break;
            }
            Err(e) => {
                msg = e.0;
            }
        }
    }
    if !sent {
        panic!("failed to send async task to COM thread after retry");
    }

    async move {
        let boxed = resp_rx.await.expect("COM thread closed");
        *boxed
            .downcast::<R>()
            .expect("type mismatch in runtime result")
    }
}
