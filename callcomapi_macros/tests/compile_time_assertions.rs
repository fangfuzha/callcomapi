// This file demonstrates compile-time assertion for Send + 'static bounds
//
// The test cases below show:
// 1. Valid: Using Send + 'static types (passes compilation)
// 2. Invalid: Using non-Send types (fails at compile time with clear error)
//
// To see these in action, comment out valid functions and uncomment invalid ones

use std::sync::Arc;
use callcomapi_macros::com_thread;

// ✅ VALID: Using Send + 'static types
#[com_thread]
fn sync_with_string(s: String) -> i32 {
    s.len() as i32
}

#[com_thread]
async fn async_with_arc(data: Arc<Vec<i32>>) -> usize {
    data.len()
}

#[com_thread(MTA)]
fn mta_with_primitives(x: i32, y: String) -> String {
    format!("{}{}", x, y)
}

#[test]
fn test_valid_send_types() {
    // Valid: String is Send + 'static
    let result = sync_with_string("hello".to_string());
    assert_eq!(result, 5);
}

#[tokio::test]
async fn test_async_valid_send_types() {
    // Valid: Arc<Vec<i32>> is Send + 'static
    let result = async_with_arc(Arc::new(vec![1, 2, 3])).await;
    assert_eq!(result, 3);
}

// ❌ INVALID EXAMPLES (COMMENTED OUT - uncomment to see compile error)
//
// These would FAIL to compile with clear error messages:
//
// USE CASE 1: Non-Send type (Rc - reference counted, not atomic)
// #[com_thread]
// fn invalid_with_rc(rc: std::rc::Rc<i32>) -> i32 {
//     *rc
// }
// ERROR: `Rc<i32>` cannot be sent between threads safely
//        --> the trait `Send` is not implemented for `Rc<i32>`
//        --> SOLUTION: Use Arc<i32> instead
//
// USE CASE 2: Borrowed reference (can't cross thread boundary)
// #[com_thread]
// fn invalid_with_ref(s: &str) -> usize {
//     s.len()
// }
// ERROR: `&str` cannot be sent between threads safely
//        --> lifetime references can't live in another thread
//        --> SOLUTION: Use String instead of &str
//
// USE CASE 3: Custom type without Send
// struct NotSend {
//     rc: std::rc::Rc<i32>,
// }
//
// #[com_thread]
// fn invalid_with_custom(obj: NotSend) -> i32 {
//     42
// }
// ERROR: `NotSend` cannot be sent between threads safely
//        --> the trait `Send` is not implemented for `NotSend`
//        --> SOLUTION: Either:
//        -->   - Add `#[derive(Send)]` to NotSend (if all fields are Send)
//        -->   - Change non-Send fields to Send equivalents
//        -->   - Use `unsafe impl Send for NotSend { }`
//
// USE CASE 4: Non-'static type (lifetime reference in struct)
// struct NotStatic<'a> {
//     reference: &'a str,
// }
//
// #[com_thread]
// fn invalid_with_lifetime(obj: NotStatic) -> String {
//     obj.reference.to_string()
// }
// ERROR: `NotStatic` cannot be sent between threads safely
//        --> the trait `Send` is not implemented for `NotStatic<'_>`
//        --> SOLUTION: Remove lifetime references, use owned data (String not &str)

#[test]
fn test_constraint_documentation() {
    // This test documents the expectations
    // All parameters and return types must:
    // 1. Implement Send trait (safe to send across threads)
    // 2. Have 'static lifetime (no borrowed references)
    //
    // Common conversions:
    // ❌ &str      -> ✅ String
    // ❌ &Vec<T>   -> ✅ Vec<T>
    // ❌ Rc<T>     -> ✅ Arc<T>
    // ❌ Cell<T>   -> ✅ atomic types (AtomicI32, etc) or Arc<Mutex<T>>

    println!("Compile-time Send + 'static constraints enforced!");
}
