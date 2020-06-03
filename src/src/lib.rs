#![recursion_limit="1024"]
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use crate::logical_expression::{LogicalExpression};

extern crate wee_alloc;

#[macro_use]
mod util;
mod logical_expression;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn run_app() {
    clog!("Hello,wasm world!");
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    if let Some(entry) = document.get_element_by_id("app") {
        App::<LogicalExpression>::new().mount(entry);
    } else {
        clog!("entry point element is not found.");
    }
}
