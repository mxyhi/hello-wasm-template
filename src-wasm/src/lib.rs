mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, basic!");
}

#[wasm_bindgen]
pub fn get_char() -> char {
    'A'
}

#[wasm_bindgen]
pub fn get_str() -> String {
    "hello str".to_owned()
}

#[wasm_bindgen]
pub fn get_usize() -> usize {
    123
}

#[wasm_bindgen]
pub fn get_isize() -> isize {
    -123
}

/// ### string inversion
/// ### 字符串反转
#[wasm_bindgen]
pub fn parse_str(str: String) -> String {
    let mut buf = String::new();
    let mut arr: Vec<char> = Vec::new();
    for c in str.chars() {
        arr.push(c);
    }
    get_isize();
    arr.reverse();
    for &c in arr.iter() {
        buf.push(c);
    }
    buf
}

/// ### fibonacci series
/// ### 斐波那契数列
#[wasm_bindgen]
pub fn fib(x: usize) -> usize {
    if x <= 0 {
        return 0;
    }
    if x < 2 {
        1
    } else {
        fib(x - 1) + fib(x - 2)
    }
}
