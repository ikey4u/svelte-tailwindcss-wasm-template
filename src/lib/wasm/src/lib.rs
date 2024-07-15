use chrono::{DateTime, Local};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[wasm_bindgen]
pub fn timestamp_to_string(timestamp: i64) -> String {
    let dt: DateTime<Local> = DateTime::from_timestamp(timestamp, 0).unwrap().into();
    dt.format("%Y-%m-%d %H:%M:%S").to_string()
}

#[wasm_bindgen]
pub fn timestamp_fmt(timestamp: i64, fmt: String) -> String {
    let dt: DateTime<Local> = DateTime::from_timestamp(timestamp, 0).unwrap().into();
    dt.format(&fmt).to_string()
}

#[wasm_bindgen]
pub fn get_timestamp() -> i64 {
    chrono::offset::Local::now().timestamp()
}
