use std::any::{Any, TypeId};
use crate::support::ServiceProvider;

#[allow(clippy::type_complexity)]
pub trait Contract {
    /// Register a Service Provider to the container.
    fn register_provider(&mut self, provider: Box<dyn ServiceProvider>);

    /// Boot the container and process the registered providers.
    fn boot(&mut self);

    /// Bind a service to the container. 
    fn bind_any(&mut self, type_id: TypeId, value: Box<dyn Any>);

    /// Bind a factory to the container.
    fn bind_factory(&mut self, type_id: TypeId, factory: Box<dyn Fn(&dyn Contract) -> Box<dyn Any>>);

    /// Bind a singleton service to the container.
    fn singleton(&mut self, type_id: TypeId, value: Box<dyn Any>);

    /// Bind a factory that resolves a singletion to the container.
    fn singleton_factory(&mut self, type_id: TypeId, factory: Box<dyn Fn(&dyn Contract) -> Box<dyn Any>>);

    /// Resolve a type from the container.
    fn resolve_any(&self, type_id: TypeId) -> Option<&dyn Any>;

    /// Execute a factory to resolve the type.
    fn transient(&self, type_id: TypeId) -> Option<Box<dyn Any>>;
}
