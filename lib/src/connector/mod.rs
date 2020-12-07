//! IO backend.

mod _base;
#[cfg(feature = "hyper_connector")]
pub mod hyper;

pub use self::_base::Connector;
#[cfg(feature = "hyper_connector")]
pub use self::hyper::HyperConnector;

use errors::Result;

/// Returns default connector.
///
/// See module level documentation for details.
#[cfg(feature = "hyper_connector")]
pub fn default_connector() -> Result<Box<Connector>> {
    hyper::default_connector()
}
