#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use std::cell::RefCell;
use tch::{ nn, Device, Kind, Tensor, nn::{ RNN, LSTMState, Module } };
use urbr::{ Back, Front, Layer, DIM, HIDDEN, FRONT_PARAMETERS, BACK_PARAMETERS };
use rand::{ Rng, SeedableRng, rngs::SmallRng };

struct ModelUrbr {
    front: Front,
    back : Back ,
}

#[derive(Debug)]
struct ModelTorch {
    lstm      : nn::LSTM            ,
    linear    : nn::Linear          ,
    lstm_state: RefCell< LSTMState >,
}

impl ModelUrbr {
    fn new() -> ModelUrbr {
        let mut front = Front::new();
        let mut back  = Back ::new();
        front.load(&FRONT_PARAMETERS);
        back .load(&BACK_PARAMETERS );
        ModelUrbr { front, back }
    }

    fn forward(&mut self, input: [f32; DIM]) -> [f32; DIM] {
        let mut ret = [0f32; DIM];
        self.front.forward(&input);
        self.back.forward(&self.front.out());
        ret.copy_from_slice(self.back.out());
        ret
    }
}

impl ModelTorch {
    fn new(vs: &nn::Path, io: usize, hidden: usize) -> ModelTorch {
        let lstm       = nn::lstm  (vs, io     as i64, hidden as i64, Default::default());
        let linear     = nn::linear(vs, hidden as i64, io     as i64, Default::default());
        let h          = Tensor::zeros(&[1, 1, hidden as i64], (Kind::Float, Device::Cpu));
        let c          = Tensor::zeros(&[1, 1, hidden as i64], (Kind::Float, Device::Cpu));
        let lstm_state = RefCell::new(LSTMState((h, c)));
        ModelTorch { lstm, linear, lstm_state }
    }
}

impl Module for ModelTorch {
    fn forward(&self, xs: &Tensor) -> Tensor {
        let new_state = self.lstm.step(xs, &self.lstm_state.borrow());
        let linear_output = self.linear.forward(&new_state.h().get(0));
        self.lstm_state.replace(new_state);
        linear_output.get(0)
    }
}

#[test]
fn full() {
    const TEST_SIZE: usize = 1000000;

    let mut rng = SmallRng::seed_from_u64(0);

    let     device      = Device::Cpu;
    let mut vs          = nn::VarStore::new(device);
    let     model_torch = ModelTorch::new(&vs.root(), DIM, HIDDEN);
    vs.load("parameter_for_tch.pt").unwrap();

    let mut diff_max_a = 0f32;
    let mut diff_max_b = 0f32;
    let mut model_urbr = ModelUrbr::new();

    for _ in 0..TEST_SIZE {
        let input        = [(); DIM].map(|_| rng.gen_range(-1f32..1f32));
        let input_torch  = Tensor::from_slice(&input).view([1, DIM as i64]);
        let output_torch = model_torch.forward(&input_torch);
        let output_urbr  = model_urbr.forward(input);
        let torch_a      = output_torch.double_value(&[0]) as f32;
        let torch_b      = output_torch.double_value(&[1]) as f32;
        let urbr_a       = output_urbr[ 0 ];
        let urbr_b       = output_urbr[ 1 ];
        let abs_diff_a   = (torch_a - urbr_a).abs();
        let abs_diff_b   = (torch_b - urbr_b).abs();
        if abs_diff_a > diff_max_a { diff_max_a = abs_diff_a }
        if abs_diff_b > diff_max_b { diff_max_b = abs_diff_b }
    }

    println!("Max diff A: {}", diff_max_a);
    println!("Max diff B: {}", diff_max_b);
}
