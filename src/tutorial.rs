#[cfg(feature = "chapter-1")]
pub use _1_getting_started::*;
#[cfg(feature = "chapter-2")]
pub use _2_lighting::*;
#[cfg(feature = "chapter-3")]
pub use _3_model_loading::*;
#[cfg(feature = "chapter-4")]
pub use _4_advanced_opengl::*;
use internal::*;

#[cfg(feature = "chapter-1")]
pub mod _1_getting_started;
#[cfg(feature = "chapter-2")]
pub mod _2_lighting;
#[cfg(feature = "chapter-3")]
pub mod _3_model_loading;
#[cfg(feature = "chapter-4")]
pub mod _4_advanced_opengl;
mod internal;
