pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn greet() {
    web_sys::console::log_1(&"Hello from Rust!".into());
}
