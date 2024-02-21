use num_traits::{ One, Float };

pub fn sigmoid< F: One + Float >(x: F) -> F {
    (< F as One >::one() + (-x).exp()).recip()
}
