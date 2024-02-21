use super::{ DIM, HIDDEN, Front, Back, Layer };

pub const LOOK_BACK        : usize      = 10;
pub const SCALER_DATA_MIN  : [f32; DIM] = [   1.044024  , -179.9695933];
pub const SCALER_DATA_MAX  : [f32; DIM] = [ 400.16666667,  179.9969416];
pub const SCALER_TARGET_MIN: [f32; DIM] = [-  1.0       , -  1.0      ];
pub const SCALER_TARGET_MAX: [f32; DIM] = [   1.0       ,    1.0      ];

pub const FRONT_PARAMETERS: [f32; Front::NUM_PARAMETERS] = [
    0.0; Front::NUM_PARAMETERS
];

pub const BACK_PARAMETERS : [f32; Back ::NUM_PARAMETERS] = [
    0.0; Back ::NUM_PARAMETERS
];
