pub mod data;
pub mod domain;
pub mod service;
pub mod web;

pub use data::DataError;
pub use domain::clip::field::Shortcode;
pub use domain::clip::Clip;
pub use domain::clip::ClipError;
pub use domain::time::Time;
pub use service::ServiceError;
