#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(incomplete_features)]
#![allow(deprecated)]
#![feature(generic_const_exprs)]

mod nn;

pub use nn::{ Layer, LSTM, Dense };

pub struct Urbr<
    const IO : usize,
    const VIA: usize,
    const SEQ: usize > where
    [(); IO  * 4]:,
    [(); VIA * 4]: {
    prev : Vec  < f64      >,
    lstm : LSTM < IO , VIA >,
    dense: Dense< VIA, IO  >
}

impl<
    const IO : usize,
    const VIA: usize,
    const SEQ: usize > Urbr< IO, VIA, SEQ > where
    [(); IO  * 4]:,
    [(); VIA * 4]: {
    // TODO: Implement
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
