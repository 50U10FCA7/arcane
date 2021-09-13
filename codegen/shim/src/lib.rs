#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(
    feature = "doc",
    deny(rustdoc::broken_intra_doc_links, rustdoc::private_intra_doc_links)
)]
#![deny(
    nonstandard_style,
    rust_2018_idioms,
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

use arcana_codegen_impl as codegen;
use proc_macro::TokenStream;

/// Macro for deriving [`Event`] on enums.
///
/// For structs consider using [`#[derive(Versioned)]`](macro@VersionedEvent).
///
/// This macro ensures that every combination of [`Event::name`][0] and
/// [`Event::version`][1] corresponds to a single Rust type. The only limitation
/// is that all the underlying [`Event`] or [`Versioned`] impls should be
/// derived too.
///
/// Also, provides a blanket [`Sourced`] implementation for every state, which
/// can be sourced from all the enum variants.
///
/// > __WARNING:__ Currently may not work with complex generics using where
/// >              clause because of `const` evaluation limitations. Should be
/// >              lifted once [rust-lang/rust#57775] is resolved.
///
/// # Variant attributes
///
/// #### `#[event(ignore)]` (optional)
///
/// Aliases: `#[event(skip)]`
///
/// Use this on a particular enum variant to completely ignore it in code
/// generation.
///
/// > __WARNING:__ Calling [`Event::name()`][0] or [`Event::version()`][1] on
/// >              ignored variants will result in [`unreachable!`] panic.
///
/// # Example
///
/// ```rust,compile_fail,E0080
/// # use arcana::es::{event, Event};
/// #
/// #[derive(event::Versioned)]
/// #[event(name = "chat", version = 1)]
/// struct ChatEvent;
///
/// #[derive(event::Versioned)]
/// #[event(name = "chat", version = 1)]
/// struct DuplicateChatEvent;
///
/// // This fails to compile as contains different Rust types with the same
/// // `event::Name` and `event::Version`.
/// #[derive(Event)]
/// enum AnyEvent {
///     Chat(ChatEvent),
///     DuplicateChat(DuplicateChatEvent),
/// }
/// ```
///
/// ```rust
/// # use arcana::es::{event, Event};
/// #
/// # #[derive(event::Versioned)]
/// # #[event(name = "chat", version = 1)]
/// # struct ChatEvent;
/// #
/// # #[derive(event::Versioned)]
/// # #[event(name = "chat", version = 1)]
/// # struct DuplicateChatEvent;
/// #
/// #[derive(Event)]
/// enum AnyEvent {
///     Chat(ChatEvent),
///     #[event(ignore)] // not recommended for real usage
///     DuplicateChat(DuplicateChatEvent),
/// }
///
/// // This example doesn't need `#[event(ignore)]` attribute, as each
/// // combination of `event::Name` and `event::Version` corresponds to a single
/// // Rust type.
/// #[derive(Event)]
/// enum MoreEvents {
///     Chat(ChatEvent),
///     ChatOnceAgain(ChatEvent),
/// }
/// ```
///
/// [`Event`]: arcana_core::es::Event
/// [`Sourced`]: arcana_core::es::event::Sourced
/// [`Versioned`]: arcana_core::es::event::Versioned
/// [0]: arcana_core::es::Event::name()
/// [1]: arcana_core::es::Event::version()
/// [rust-lang/rust#57775]: https://github.com/rust-lang/rust/issues/57775
#[proc_macro_derive(Event, attributes(event))]
pub fn derive_event(input: TokenStream) -> TokenStream {
    codegen::es::event::derive(input.into())
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

/// Macro for deriving [`Versioned`] on structs.
///
/// For enums consisting of different [`Versioned`] events consider using
/// [`#[derive(Event)]`](macro@Event).
///
/// # Struct attributes
///
/// #### `#[event(name = "...")]`
///
/// Value of [`Versioned::NAME`][0] constant.
///
/// #### `#[event(version = <non-zero-u16>)]`
///
/// Aliases: `#[event(ver = <non-zero-u16>)]`
///
/// Value of [`Versioned::VERSION`][1] constant.
///
/// # Example
///
/// ```rust
/// # use arcana::es::event;
/// #
/// #[derive(event::Versioned)]
/// #[event(name = "event", version = 1)]
/// struct Event;
/// ```
///
/// [`Versioned`]: arcana_core::es::event::Versioned
/// [0]: arcana_core::es::event::Versioned::NAME
/// [1]: arcana_core::es::event::Versioned::VERSION
#[proc_macro_derive(VersionedEvent, attributes(event))]
pub fn derive_versioned_event(input: TokenStream) -> TokenStream {
    codegen::es::event::versioned::derive(input.into())
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

/// Macro for deriving [`Transformer`] on [`Adapter`]s to transform derived
/// [`Event`] enum.
///
/// # Struct attributes
///
/// #### `#[event(transformer(adapter = <ty>))]`
///
/// [`Adapter`] to derive [`Transformer`] on.
///
/// Provided [`Adapter`] must be able to [`Transformer::transform`][0] every
/// enum's variant.
///
/// #### `#[event(transformer(transformed = <ty>))]`
///
/// [`Transformer::Transformed`][1] type for [`Transformer`] impl.
///
/// #### `#[event(transformer(context = <ty>))]`
///
/// [`Transformer::Context`][2] type for [`Transformer`] impl.
///
/// #### `#[event(transformer(error = <ty>))]`
///
/// [`Transformer::Error`][3] type for [`Transformer`] impl.
///
/// # Example
///
/// ```rust
/// # #![feature(generic_associated_types)]
/// #
/// # use std::{any::Any, convert::Infallible};
/// #
/// # use arcana::es::adapter::transformer::{self, strategy, Transformer};
/// # use derive_more::From;
/// #
/// struct Adapter;
///
/// struct InputEvent;
///
/// impl transformer::WithStrategy<InputEvent> for Adapter {
///     type Strategy = strategy::Into<OutputEvent>;
/// }
///
/// impl From<InputEvent> for OutputEvent {
///     fn from(_: InputEvent) -> Self {
///         OutputEvent
///     }
/// }
///
/// struct OutputEvent;
///
/// #[derive(From, Transformer)]
/// #[event(
///     transformer(
///         adapter = Adapter,
///         transformed = OutputEvents,
///         context = dyn Any,
///         error = Infallible,
///     )
/// )]
/// enum InputEvents {
///     Input(InputEvent),
/// }
///
/// #[derive(From)]
/// enum OutputEvents {
///     Output(OutputEvent),
/// }
/// ```
///
/// > __NOTE__: Single [`Event`] enum can be [`Transformer::transform`][0]ed by
/// >           multiple [`Adapter`]s.
///
/// ```rust
/// # #![feature(generic_associated_types)]
/// #
/// # use std::{any::Any, convert::Infallible};
/// #
/// # use arcana::es::adapter::transformer::{self, strategy, Transformer};
/// # use derive_more::From;
/// #
/// # struct FirstAdapter;
/// #
/// # struct SecondAdapter;
/// #
/// # struct InputEvent;
/// #
/// # impl transformer::WithStrategy<InputEvent> for FirstAdapter {
/// #     type Strategy = strategy::Into<OutputEvent>;
/// # }
/// #
/// # impl transformer::WithStrategy<InputEvent> for SecondAdapter {
/// #     type Strategy = strategy::Into<OutputEvent>;
/// # }
/// #
/// # impl From<InputEvent> for OutputEvent {
/// #     fn from(_: InputEvent) -> Self {
/// #         OutputEvent
/// #     }
/// # }
/// #
/// # struct OutputEvent;
/// #
/// #[derive(From, Transformer)]
/// #[event(
///     transformer(
///         adapter = FirstAdapter,
///         transformed = OutputEvents,
///         context = dyn Any,
///         error = Infallible,
///     ),
///     transformer(
///         adapter = SecondAdapter,
///         transformed = OutputEvents,
///         context = dyn Any,
///         error = Infallible,
///     ),
/// )]
/// enum InputEvents {
///     Input(InputEvent),
/// }
/// #
/// # #[derive(From)]
/// # enum OutputEvents {
/// #     Output(OutputEvent),
/// # }
/// ```
/// [0]: arcana_core::es::adapter::Transformer::transform()
/// [1]: arcana_core::es::adapter::Transformer::Transformed
/// [2]: arcana_core::es::adapter::Transformer::Context
/// [3]: arcana_core::es::adapter::Transformer::Error
/// [`Adapter`]: arcana_core::es::Adapter
/// [`Event`]: trait@arcana_core::es::Event
/// [`Transformer`]: arcana_core::es::adapter::Transformer
#[proc_macro_derive(EventTransformer, attributes(event))]
pub fn derive_event_transformer(input: TokenStream) -> TokenStream {
    codegen::es::event::transformer::derive(input.into())
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
