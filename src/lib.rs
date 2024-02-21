#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(incomplete_features)]
#![allow(deprecated)]
#![feature(generic_const_exprs)]
#![feature(associated_type_defaults)]

mod nn;
mod scaler;
mod data;

use nn::{ LSTM, Dense };
use scaler::Scaler;
pub use nn::Layer;
pub use data::*;

pub const DIM   : usize =   2;
pub const HIDDEN: usize = 100;

pub type Front      = LSTM  < DIM   , HIDDEN >;
pub type Back       = Dense < HIDDEN, DIM    >;
pub type Preprocess = Scaler< DIM            >;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
