//! [`Event`] machinery.

#[doc(inline)]
pub use arcane_core::es::event::{
    Event, Initial, Initialized, Name, Sourced, Sourcing, Version, Versioned,
};

#[cfg(feature = "derive")]
#[doc(inline)]
pub use arcane_codegen::es::event::{Event, Versioned};
#[cfg(feature = "derive")]
#[doc(inline)]
pub use arcane_core::es::event::codegen;
