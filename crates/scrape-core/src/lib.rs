//! # scrape-core
//!
//! High-performance HTML parsing library with CSS selector support.
//!
//! This crate provides the core functionality for parsing HTML documents
//! and querying them using CSS selectors. It is designed to be fast,
//! memory-efficient, and spec-compliant.
//!
//! ## Quick Start
//!
//! ```rust,ignore
//! use scrape_core::{Soup, SoupConfig};
//!
//! let html = "<html><body><div class=\"product\">Hello</div></body></html>";
//! let soup = Soup::parse(html);
//!
//! // Find elements by CSS selector
//! let products = soup.select("div.product");
//! for product in products {
//!     println!("{}", product.text());
//! }
//! ```
//!
//! ## Features
//!
//! - **Fast parsing**: Built on `html5ever` for spec-compliant HTML5 parsing
//! - **CSS selectors**: Full CSS selector support via the `selectors` crate
//! - **Memory efficient**: Arena-based allocation for DOM nodes
//! - **SIMD acceleration**: Optional SIMD support for faster byte scanning

#![warn(missing_docs)]
#![warn(clippy::all)]
#![warn(clippy::pedantic)]

mod dom;
mod error;
mod parser;
mod query;
mod soup;
mod tag;

pub use error::{Error, Result};
pub use soup::{Soup, SoupConfig};
pub use tag::Tag;
