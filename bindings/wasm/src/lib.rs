#![allow(dead_code)]

use std::collections::VecDeque;
use js_sys::{ wasm_bindgen, Float32Array };
use js_sys::wasm_bindgen::prelude::wasm_bindgen;
use urbr::{ DIM, HIDDEN, Front, Preprocess, Layer, LOOK_BACK, SCALER_DATA_MIN, SCALER_DATA_MAX, SCALER_TARGET_MIN, SCALER_TARGET_MAX, FRONT_PARAMETERS };

#[wasm_bindgen]
struct UrbrFront {
    prev      : VecDeque< [f32; DIM] >,
    preprocess: Preprocess            ,
    front     : Front                 ,
}

#[wasm_bindgen]
impl UrbrFront {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let mut ret = Self {
            prev: VecDeque::new(),
            preprocess: Preprocess {
                data_min  : SCALER_DATA_MIN  ,
                data_max  : SCALER_DATA_MAX  ,
                target_min: SCALER_TARGET_MIN,
                target_max: SCALER_TARGET_MAX, },
            front: Front::new()
        };

        ret.front.load(&FRONT_PARAMETERS);
        ret
    }

    #[wasm_bindgen]
    pub fn out_buffer() -> Float32Array {
        Float32Array::new_with_length(HIDDEN as u32)
    }

    #[wasm_bindgen]
    pub fn inference(&mut self, a: f32, b: f32, dest: &Float32Array) {
        // for Inference
        let scaled = self.preprocess.transform(&[a, b]);

        if self.prev.is_empty() {
           self.prev.extend(std::iter::repeat(scaled).take(LOOK_BACK));
        }
        else {
            self.prev.pop_front();
            self.prev.push_back(scaled);
        }

        for al in &self.prev {
            self.front.forward(al);
        }

        dest.copy_from(self.front.out());

        // for test
        // dest.set_index(0, a);
        // dest.set_index(1, b);
    }
}
