#![doc = include_str!("../README.md")]
#![deny(
    nonstandard_style,
    rust_2018_idioms,
    rustdoc::broken_intra_doc_links,
    rustdoc::private_intra_doc_links,
    trivial_casts,
    trivial_numeric_casts
)]
#![forbid(non_ascii_idents, unsafe_code)]
#![warn(
    deprecated_in_future,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    unreachable_pub,
    unused_import_braces,
    unused_labels,
    unused_qualifications,
    unused_results
)]

#[doc(hidden)]
pub mod private;

#[doc(inline)]
pub use arcana_core::{Event, EventInitialized, EventSourced, InitialEvent, VersionedEvent};

/// Macro for deriving [`Event`](trait@Event) on enums. For structs consider
/// [`VersionedEvent`](macro@VersionedEvent).
///
/// This macro ensures that every combination of `event_type` and `ver` are
/// unique. The only limitation is that every underlying
/// [`Event`](trait@Event) or [`VersionedEvent`](trait@VersionedEvent) impls are
/// generated with proc macros.
///
/// # Example
///
/// ```compile_fail
/// # use arcana::{Event, VersionedEvent};
///
/// #[derive(VersionedEvent)]
/// #[event(type = "chat", version = 1)]
/// struct ChatEvent;
///
/// #[derive(VersionedEvent)]
/// #[event(type = "file", version = 1)]
/// struct FileEvent;
///
/// #[derive(Event)]
/// enum AnyEvent {
///     Chat(ChatEvent),
///     File { event: FileEvent },
/// }
///
/// #[derive(Event)]
/// enum DuplicatedEvent {
///     Any(AnyEvent),
///     File(event: FileEvent),
/// }
/// ```
pub use arcana_codegen::Event;

/// Macro for deriving [`VersionedEvent`](trait@VersionedEvent) on structs. For
/// enums, consisting of different events consider [`Event`](macro@Event).
pub use arcana_codegen::VersionedEvent;
