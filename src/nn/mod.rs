mod lstm ;
mod dense;
mod util ;

pub use lstm ::*;
pub use dense::*;
pub use util ::*;

pub trait Layer< IN > {
    type OUT;

    const NUM_PARAMETERS: usize;

    fn load(&mut self, parameter: &[f64]);
    fn out(&self) -> &Self::OUT;
    fn forward(&mut self, input: &IN);
}
