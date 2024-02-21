pub struct Scaler< const DIM: usize > {
    pub data_min  : [f32; DIM],
    pub data_max  : [f32; DIM],
    pub target_min: [f32; DIM],
    pub target_max: [f32; DIM],
}

impl< const DIM: usize > Scaler< DIM > {
    pub fn transform(&self, x: [f32; DIM]) -> [f32; DIM] {
        let mut result = [0.0; DIM];

        for i in 0..DIM {
            let std = (x[ i ] - self.data_min[ i ]) / (self.data_max[ i ] - self.data_min[ i ]);
            result[ i ] = std * (self.target_max[ i ] - self.target_min[ i ]) + self.target_min[ i ];
        }

        result
    }

    pub fn inverse_transform(&self, x: [f32; DIM]) -> [f32; DIM] {
        let mut result = [0.0; DIM];

        for i in 0..DIM {
            let std = (x[ i ] - self.target_min[ i ]) / (self.target_max[ i ] - self.target_min[ i ]);
            result[ i ] = std * (self.data_max[ i ] - self.data_min[ i ]) + self.data_min[ i ];
        }

        result
    }
}
