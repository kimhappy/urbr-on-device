// TODO: Inference code

#[no_mangle]
pub extern "C" fn inference(from: *const f32, to: *mut f32) {
    let from_slice = unsafe { std::slice::from_raw_parts    (from, 100) };
    let to_slice   = unsafe { std::slice::from_raw_parts_mut(to  ,   2) };

    to_slice[ 0 ] = from_slice[ 0 ];
    to_slice[ 1 ] = from_slice[ 1 ];
}
