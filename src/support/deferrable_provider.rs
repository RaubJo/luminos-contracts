pub trait DeferrableProvider {
    /// Get the services provided by the provider.
    fn provides() -> &'static [&'static str]; 
}
