mod loader;
mod lstm;
mod dense;
mod sigmoid;

pub use loader ::*;
pub use lstm   ::*;
pub use dense  ::*;
pub use sigmoid::*;

pub trait Layer< IN > {
    type IN  = IN;
    type OUT     ;

    const NUM_PARAMETERS: usize;

    fn load(&mut self, parameter: &[f32]);
    fn out(&self) -> &Self::OUT;
    fn forward(&mut self, input: &IN);
}
