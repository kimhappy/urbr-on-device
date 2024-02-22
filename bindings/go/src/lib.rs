use std::sync::Mutex;
use urbr::{ DIM, HIDDEN, Back, Preprocess, Layer, SCALER_DATA_MIN, SCALER_DATA_MAX, SCALER_TARGET_MIN, SCALER_TARGET_MAX, BACK_PARAMETERS };

static URBR_BACK: Mutex< Option< UrbrBack > > = Mutex::new(None);

struct UrbrBack {
    back      : Back      ,
    preprocess: Preprocess,
}

impl UrbrBack {
    fn new() -> Self {
        let mut back       = Back::new();
        let     preprocess = Preprocess {
            data_min  : SCALER_DATA_MIN  ,
            data_max  : SCALER_DATA_MAX  ,
            target_min: SCALER_TARGET_MIN,
            target_max: SCALER_TARGET_MAX,
        };

        back.load(&BACK_PARAMETERS);
        Self { back, preprocess }
    }

    fn inference(&mut self, from: &[f32; HIDDEN], to: &mut [f32; DIM]) {
        self.back.forward(&from);
        to.copy_from_slice(&self.preprocess.inverse_transform(&self.back.out()));
    }
}

#[no_mangle]
pub extern "C" fn inference(from: *const f32, to: *mut f32) {
    let from_slice = unsafe { std::slice::from_raw_parts    (from, 100) };
    let to_slice   = unsafe { std::slice::from_raw_parts_mut(to  ,   2) };

    let mut urbr_back = URBR_BACK.lock().unwrap();

    if urbr_back.is_none() {
        *urbr_back = Some(UrbrBack::new());
    }

    if let Some(ref mut back) = *urbr_back {
        back.inference(from_slice.try_into().unwrap(), to_slice.try_into().unwrap());
    }
}
