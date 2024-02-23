from joblib import load
import torch

data_rs = ''

scaler = load("reference/minmax_scaler.pkl")

data_rs += 'use super::{ DIM, HIDDEN, Front, Back, Layer };'
data_rs += 'pub const LOOK_BACK: usize = 10;'
data_rs += 'pub const SCALER_DATA_MIN: [f32; DIM] = [{}];'.format(', '.join(map(str, scaler.data_min_)))
data_rs += 'pub const SCALER_DATA_MAX: [f32; DIM] = [{}];'.format(', '.join(map(str, scaler.data_max_)))
data_rs += 'pub const SCALER_TARGET_MIN: [f32; DIM] = [-1.0, -1.0];'
data_rs += 'pub const SCALER_TARGET_MAX: [f32; DIM] = [1.0, 1.0];'

sd    = dict(torch.load("reference/lstm_model.pth"))
front = []
back  = []

for name, tensor in sd.items():
    if name.startswith("lstm"):
        front += tensor.flatten().tolist()
    else:
        back  += tensor.flatten().tolist()

data_rs += 'pub const FRONT_PARAMETERS: [f32; Front::NUM_PARAMETERS] = [{}];'.format(', '.join(map(str, front)))
data_rs += 'pub const BACK_PARAMETERS : [f32; Back ::NUM_PARAMETERS] = [{}];'.format(', '.join(map(str, back )))

sd[ 'weight_ih_l0' ] = sd.pop('lstm.weight_ih_l0')
sd[ 'weight_hh_l0' ] = sd.pop('lstm.weight_hh_l0')
sd[ 'bias_ih_l0'   ] = sd.pop('lstm.bias_ih_l0'  )
sd[ 'bias_hh_l0'   ] = sd.pop('lstm.bias_hh_l0'  )
sd[ 'weight'       ] = sd.pop('linear.weight'    )
sd[ 'bias'         ] = sd.pop('linear.bias'      )

file = open("src/data.rs", 'w')
file.write(data_rs)
file.close()

torch.save(sd, "parameter_for_tch.pt")
