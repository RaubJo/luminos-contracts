use crate::container::Contract;

pub trait ServiceProvider<C: Contract>: Send + Sync {
    fn register(&self, container: &C) {}

    fn boot(&self, container: &C) {}
}
