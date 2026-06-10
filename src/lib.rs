//! **Tempt** adds templating for tokens with the [`tempt!`] macro.
//!
//! # Setup
//!
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! tempt = "0.1.0"
//! ```
//!
//! # Usage
//!
//! In the example below, Tempt is used to generate builder methods for
//! `UserBuilder`.
//!
//! ```
//! # {} /*
//! use tempt::tempt;
//!
//! struct User {
//!     // ...
//! }
//!
//! #[derive(Default)]
//! struct UserBuilder {
//!     name: Option<String>,
//!     age: Option<u8>,
//!     email: Option<String>,
//! }
//!
//! tempt! {
//!       field_name    field_type
//!     [ name       ][ String     ]
//!     [ age        ][ u8         ]
//!     [ email      ][ String     ];
//!
//!     impl UserBuilder {
//!         #(
//!             fn #field_name(mut self, value: impl Into<#field_type>) -> Self {
//!                 self.#field_name = Some(value.into());
//!                 self
//!             }
//!         )*
//!
//!         fn build(self) -> Option<User> {
//!             // ...
//!         }
//!     }
//! }
//! # */
//! ```
//!
//! The [`tempt!`] macro expands to the following code:
//!
//! ```
//! # {} /*
//! impl UserBuilder {
//!     fn name(mut self, value: impl Into<String>) -> Self {
//!         self.name = Some(value.into());
//!         self
//!     }
//!
//!     fn age(mut self, value: impl Into<u8>) -> Self {
//!         self.age = Some(value.into());
//!         self
//!     }
//!
//!     fn email(mut self, value: impl Into<String>) -> Self {
//!         self.email = Some(value.into());
//!         self
//!     }
//!
//!     fn build(self) -> Option<User> {
//!         // ...
//!     }
//! }
//! # */
//! ```
//!
//! The full example is available at [examples/usage.rs].
//!
//! # Syntax
//!
//! The [`tempt!`] macro consists of tables and a template, separated by a
//! semicolon (`;`).
//!
//! ```
//! # {} /*
//! tempt! {
//!     <tables>;
//!     <template>
//! }
//! # */
//! ```
//!
//! ## Tables
//!
//! A table consists of placeholders (identifiers) and their replacements
//! (bracketed tokens). Replacements can contain any tokens, and Tempt will
//! substitute them verbatim.
//!
//! ```text
//!   field_name    field_type
//! [ name       ][ String     ]
//! [ age        ][ u8         ]
//! [ email      ][ String     ];
//! ```
//!
//! In the table above:
//!
//! - `field_name` is a placeholder whose replacements are `name`, `age`, and
//!   `email`.
//! - `field_type` is a placeholder whose replacements are `String`, `u8`, and
//!   `String`.
//!
//! Repetitions in the template expand once per table entry. This table produces
//! the following entries:
//!
//! 1. `field_name` = `name`, `field_type` = `String`
//! 2. `field_name` = `age`, `field_type` = `u8`
//! 3. `field_name` = `email`, `field_type` = `String`
//!
//! ### Alternative Layout
//!
//! Tables can also be written with placeholders on the left-hand side. The
//! table below is equivalent to the table above:
//!
//! ```text
//! field_name [ name   ][ age ][ email  ]
//! field_type [ String ][ u8  ][ String ];
//! ```
//!
//! ### Multiple Tables
//!
//! Multiple tables can be declared in a single macro invocation. Entries from
//! each table are processed in declaration order, so the tables below are
//! equivalent to the tables above:
//!
//! ```text
//!   field_name    field_type
//! [ name       ][ String     ]
//! [ age        ][ u8         ]
//!
//! field_name [ email  ]
//! field_type [ String ];
//! ```
//!
//! ### String Literals
//!
//! Tempt does not substitute placeholders inside string literals. If you need
//! this functionality, consider using [`stringify!`] and [`concat!`].
//!
//! ```
//! # {} /*
//! #[doc = concat!("Documentation for ", stringify!(#struct_name))]
//! struct #struct_name {
//!     // ...
//! }
//! # */
//! ```
//!
//! ## Template
//!
//! Templates support repetitions in the following forms:
//!
//! - `#(repetition)*` — repeats `repetition` with no separator.
//! - `#(repetition)(separator)*` — repeats `repetition`, inserting `separator`
//!   between each expansion.
//!
//! Inside a repetition, placeholders can be referenced using `#placeholder`.
//!
//! ```
//! # {} /*
//! #(
//!     fn #field_name(mut self, value: impl Into<#field_type>) -> Self {
//!         self.#field_name = Some(value.into());
//!         self
//!     }
//! )*
//! # */
//! ```
//!
//! ### Multiple Repetitions
//!
//! A single template can have multiple repetitions. Returning to the example
//! from the [usage section](#usage), we can use the same table to generate the
//! struct definition of `UserBuilder`.
//!
//! ```
//! # {} /*
//! #[derive(Default)]
//! struct UserBuilder {
//!     #(
//!         #field_name: Option<#field_type>,
//!     )*
//! }
//!
//! impl UserBuilder {
//!     #(
//!         fn #field_name(mut self, value: impl Into<#field_type>) -> Self {
//!             self.#field_name = Some(value.into());
//!             self
//!         }
//!     )*
//!
//!     fn build(self) -> Option<User> {
//!         // ...
//!     }
//! }
//! # */
//! ```
//!
//! ### Nested Invocations
//!
//! Templates can contain other [`tempt!`] invocations. In the example below,
//! the replacements in the outer macro are substituted into the table of the
//! inner macro.
//!
//! ```
//! use tempt::tempt;
//!
//! trait IsNonnegative {
//!     fn is_nonnegative(self) -> bool;
//! }
//!
//! tempt! {
//!     signed   [ self >= 0 ]
//!     unsigned [ true      ];
//!
//!     #(
//!         tempt! {
//!               number_type    implementation
//!             [ i8          ][ #signed        ]
//!             [ i16         ][ #signed        ]
//!             [ u8          ][ #unsigned      ]
//!             [ u16         ][ #unsigned      ];
//!
//!             #(
//!                 impl IsNonnegative for #number_type {
//!                     fn is_nonnegative(self) -> bool {
//!                         #implementation
//!                     }
//!                 }
//!             )*
//!         }
//!     )*
//! }
//! ```
//!
//! [examples/usage.rs]: https://github.com/michaelni678/tempt/tree/main/examples/usage.rs
use std::convert;

use proc_macro::TokenStream;

mod core;

#[proc_macro]
pub fn tempt(input: TokenStream) -> TokenStream {
    core::expand(input.into())
        .unwrap_or_else(convert::identity)
        .into()
}
