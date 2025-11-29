mod contract;
mod error;

pub use contract::Contract;
pub use error::*;

pub trait Injectable {
    fn __register<C: Contract>(container: &C);
}
