use super::{ Layer, Loader, sigmoid };

pub struct LSTM<
    const IN : usize,
    const OUT: usize,
> where
    [(); IN  * 4]:,
    [(); OUT * 4]:, {
    // Parameters: i, f, g, o
    weight_i_: [[[f32; 4]; IN ]; OUT],
    weight_h_: [[[f32; 4]; OUT]; OUT],
    bias_    : [ [f32; 4]      ; OUT], // Pre-calculated bias_i + bias_h

    // States (and outputs)
    o_       : [  f32          ; OUT],
    c_       : [  f32          ; OUT],
    h_       : [  f32          ; OUT],
}

impl<
    const IN : usize,
    const OUT: usize,
> LSTM< IN, OUT > where
    [(); IN  * 4]:,
    [(); OUT * 4]:, {
    pub fn new() -> Self {
        Self {
            weight_i_: [[[0.0; 4]; IN ]; OUT],
            weight_h_: [[[0.0; 4]; OUT]; OUT],
            bias_    : [ [0.0; 4]      ; OUT],
            o_       : [  0.0          ; OUT],
            c_       : [  0.0          ; OUT],
            h_       : [  0.0          ; OUT],
        }
    }
}

impl<
    const IN : usize,
    const OUT: usize,
> Layer< [f32; IN] > for LSTM< IN, OUT > where
    [(); IN  * 4]:,
    [(); OUT * 4]: {
    type OUT = [f32; OUT];

    const NUM_PARAMETERS: usize = 4 * OUT * (IN + OUT + 2);

    fn load(&mut self, parameter: &[f32]) {
        let mut loader     = Loader::new(parameter);
        let mut weight_i_i = [[0.0; IN ]; OUT];
        let mut weight_i_f = [[0.0; IN ]; OUT];
        let mut weight_i_g = [[0.0; IN ]; OUT];
        let mut weight_i_o = [[0.0; IN ]; OUT];
        let mut weight_h_i = [[0.0; OUT]; OUT];
        let mut weight_h_f = [[0.0; OUT]; OUT];
        let mut weight_h_g = [[0.0; OUT]; OUT];
        let mut weight_h_o = [[0.0; OUT]; OUT];
        let mut bias_i_i_  = [ 0.0      ; OUT];
        let mut bias_i_f_  = [ 0.0      ; OUT];
        let mut bias_i_g_  = [ 0.0      ; OUT];
        let mut bias_i_o_  = [ 0.0      ; OUT];
        let mut bias_h_i_  = [ 0.0      ; OUT];
        let mut bias_h_f_  = [ 0.0      ; OUT];
        let mut bias_h_g_  = [ 0.0      ; OUT];
        let mut bias_h_o_  = [ 0.0      ; OUT];

        loader.load(&mut weight_i_i);
        loader.load(&mut weight_i_f);
        loader.load(&mut weight_i_g);
        loader.load(&mut weight_i_o);
        loader.load(&mut weight_h_i);
        loader.load(&mut weight_h_f);
        loader.load(&mut weight_h_g);
        loader.load(&mut weight_h_o);
        loader.load(&mut bias_i_i_ );
        loader.load(&mut bias_i_f_ );
        loader.load(&mut bias_i_g_ );
        loader.load(&mut bias_i_o_ );
        loader.load(&mut bias_h_i_ );
        loader.load(&mut bias_h_f_ );
        loader.load(&mut bias_h_g_ );
        loader.load(&mut bias_h_o_ );

        for i in 0..OUT {
            for j in 0..IN {
                self.weight_i_[ i ][ j ][ 0 ] = weight_i_i[ i ][ j ];
                self.weight_i_[ i ][ j ][ 1 ] = weight_i_f[ i ][ j ];
                self.weight_i_[ i ][ j ][ 2 ] = weight_i_g[ i ][ j ];
                self.weight_i_[ i ][ j ][ 3 ] = weight_i_o[ i ][ j ];
            }

            for j in 0..OUT {
                self.weight_h_[ i ][ j ][ 0 ] = weight_h_i[ i ][ j ];
                self.weight_h_[ i ][ j ][ 1 ] = weight_h_f[ i ][ j ];
                self.weight_h_[ i ][ j ][ 2 ] = weight_h_g[ i ][ j ];
                self.weight_h_[ i ][ j ][ 3 ] = weight_h_o[ i ][ j ];
            }

            self.bias_[ i ][ 0 ] = bias_i_i_[ i ] + bias_h_i_[ i ];
            self.bias_[ i ][ 1 ] = bias_i_f_[ i ] + bias_h_f_[ i ];
            self.bias_[ i ][ 2 ] = bias_i_g_[ i ] + bias_h_g_[ i ];
            self.bias_[ i ][ 3 ] = bias_i_o_[ i ] + bias_h_o_[ i ];
        }
    }

    fn out(&self) -> &Self::OUT {
        &self.h_
    }

    fn forward(&mut self, input: &[f32; IN]) {
        for i in 0..OUT {
            let mut acc = self.bias_[ i ];

            for j in 0..IN {
                acc[ 0 ] += input  [ j ] * self.weight_i_[ i ][ j ][ 0 ];
                acc[ 1 ] += input  [ j ] * self.weight_i_[ i ][ j ][ 1 ];
                acc[ 2 ] += input  [ j ] * self.weight_i_[ i ][ j ][ 2 ];
                acc[ 3 ] += input  [ j ] * self.weight_i_[ i ][ j ][ 3 ];
            }

            for j in 0..OUT {
                acc[ 0 ] += self.h_[ j ] * self.weight_h_[ i ][ j ][ 0 ];
                acc[ 1 ] += self.h_[ j ] * self.weight_h_[ i ][ j ][ 1 ];
                acc[ 2 ] += self.h_[ j ] * self.weight_h_[ i ][ j ][ 2 ];
                acc[ 3 ] += self.h_[ j ] * self.weight_h_[ i ][ j ][ 3 ];
            }

            self.o_[ i ] = sigmoid(acc[ 3 ]);
            self.c_[ i ] = sigmoid(acc[ 0 ]) * acc[ 2 ].tanh() + sigmoid(acc[ 1 ]) * self.c_[ i ];
        }

        for i in 0..OUT {
            self.h_[ i ] = self.o_[ i ] * self.c_[ i ].tanh();
        }
    }
}
