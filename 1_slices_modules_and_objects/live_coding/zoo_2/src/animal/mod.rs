// Talk about how there are private and public modules
pub mod ape;
pub mod gistre;
pub mod skunk;

// Talk about pub use and how it affects the main. Now, we can do `animal::Ape` instead
// of `animal::ape::Ape`
pub use ape::Ape;
pub use gistre::Gistre;
pub use skunk::Skunk;
