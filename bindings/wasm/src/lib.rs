#![allow(dead_code)]

use std::collections::VecDeque;
use js_sys::{ wasm_bindgen, Float64Array };
use js_sys::wasm_bindgen::prelude::wasm_bindgen;
use urbr::{ Layer, LSTM };

const DIM: usize =   2;
const HID: usize = 100;
const LB : usize =  10;

#[wasm_bindgen]
struct UrbrFront {
    prev: VecDeque< [f64; DIM] >,
    lstm: LSTM    < DIM, HID   >,
}

#[wasm_bindgen]
impl UrbrFront {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            prev: VecDeque::new(),
            lstm: LSTM    ::new()
        }
    }

    #[wasm_bindgen]
    pub fn num_parameters() -> usize {
        LSTM::< DIM, HID >::NUM_PARAMETERS
    }

    #[wasm_bindgen]
    pub fn out_buffer() -> Float64Array {
        Float64Array::new_with_length(100)
    }

    #[wasm_bindgen]
    pub fn inference(&mut self, alt: f64, lat: f64, dest: &Float64Array) {
        if self.prev.is_empty() {
           self.prev.extend(std::iter::repeat([alt, lat]).take(LB));
        }
        else {
            self.prev.pop_front();
            self.prev.push_back([alt, lat]);
        }

        for al in &self.prev {
            self.lstm.forward(al);
        }

        dest.copy_from(self.lstm.out());
    }
}
