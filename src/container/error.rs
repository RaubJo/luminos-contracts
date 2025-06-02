#[derive(Debug)]
pub enum Error {
    ServiceNotFound(String),
    TypeMismatch(String)
}

impl std::fmt::Display for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::ServiceNotFound(t) => write!(f, "Service not found: {t}"),
			Self::TypeMismatch(t) => write!(f, "Type mismatch for service: {t}"),
		}
	}
}

impl std::error::Error for Error {}
