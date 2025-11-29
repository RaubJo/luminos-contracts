mod error;
mod contract;

pub use error::*;
pub use contract::Contract;

pub trait Injectable {
    fn __register<C: Contract>(container: &C);
}
