use crate::container::Contract;

pub trait ServiceProvider {
    fn register(&mut self, container: &mut dyn Contract);
    fn boot(&mut self, container: &mut dyn Contract);
}
