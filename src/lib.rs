mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// #[wasm_bindgen]
// extern {
//     fn alert(s: String);
// }

// #[wasm_bindgen]
// pub fn greet() {
//     alert("Hello, wasm-game-of-life!".to_string());
// }

#[wasm_bindgen]

pub fn add_two_numbers(a: f64, b: f64) -> f64 {
  let sum: f64 = a + b;
  return sum;
  //alert(sum.to_string());
}



#[wasm_bindgen]

pub fn sub_two_numbers(a: f64, b: f64)-> f64 {
  let sum: f64 = a - b;
  return sum;
  //alert(sum.to_string());
}

#[wasm_bindgen]

pub fn mul_two_numbers(a: f64, b: f64)-> f64 {
  let sum: f64 = a * b;
  return sum;
  //alert(sum.to_string());
}

#[wasm_bindgen]

pub fn div_two_numbers(a: f64, b: f64)-> f64 {
  let sum: f64 = a / b;
  return sum;
  //alert(sum.to_string());
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_add_two_numbers() {
        let a = 2.00;
        let b = 1.00;

        let result = add_two_numbers(a,b);
        assert_eq!("3.00", result);
    }
}