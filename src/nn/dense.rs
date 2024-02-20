use super::{ Layer, util::Loader };

pub struct Dense<
    const IN : usize,
    const OUT: usize,
> {
    // Parameters
    weight_: [[f64; OUT]; IN],
    bias_  :  [f64; OUT]     ,

    // Outputs
    out_   :  [f64; OUT]     ,
}

impl<
    const IN : usize,
    const OUT: usize,
> Dense< IN, OUT > {
    pub fn new() -> Self {
        Self {
            weight_: unsafe { std::mem::uninitialized() },
            bias_  : unsafe { std::mem::uninitialized() },
            out_   : unsafe { std::mem::uninitialized() },
        }
    }
}

impl<
    const IN : usize,
    const OUT: usize,
> Layer< [f64; IN] > for Dense< IN, OUT > {
    const NUM_PARAMETERS: usize = OUT * (IN + 1);

    type OUT = [f64; OUT];

    fn load(&mut self, parameter: &[f64]) {
        let mut loader = Loader::new(parameter);

        loader.load(&mut self.weight_);
        loader.load(&mut self.bias_  );
    }

    fn out(&self) -> &Self::OUT {
        &self.out_
    }

    fn forward(&mut self, input: &[f64; IN]) {
        for i in 0..OUT {
            self.out_[ i ] = self.bias_[ i ];

            for j in 0..IN {
                self.out_[ i ] += input[ j ] * self.weight_[ j ][ i ];
            }
        }
    }
}