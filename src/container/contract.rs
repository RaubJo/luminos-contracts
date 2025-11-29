use crate::container::Injectable;
use crate::support::ServiceProvider;
use std::sync::Arc;

// pub trait ContractCore {
//     fn add_provider(&self, provider: Box<dyn ServiceProvider<Self> + 'static>);
//     fn add_providers(&self, providers: Vec<Box<dyn ServiceProvider<Self> + 'static>>);
//     fn boot(&self);
// }

pub trait Contract {
    fn add_provider(&self, provider: Box<dyn ServiceProvider<Self> + 'static>) -> &Self;
    fn add_providers(&self, providers: Vec<Box<dyn ServiceProvider<Self> + 'static>>) -> &Self;
    fn boot(&self) -> &Self;
    fn bind<T, F>(&self, factory: F)
    where
        T: Sized + Send + Sync + 'static,
        F: Fn(&Self) -> Arc<T> + Send + Sync + 'static;

    fn resolve<T>(&self) -> Arc<T>
    where
        T: Injectable + Send + Sync + 'static;

    /// Builder pattern for adding a provider.
    fn with_provider(self, provider: Box<dyn ServiceProvider<Self> + 'static>) -> Self;

    /// Build the container. Calls `boot()`.
    fn build(&self) -> &Self;
}
