//! # Maud for Iron
//!
//! This library combines [Maud templating and [Iron web framework](http://ironframework.io) together. It gives you a `MaudEngine` as Iron `AfterMiddleware`, so you can render your data with specified handlebars template.
//!
//! ## Setup
//!
//!
//! ## Usage
//!
//! We have implemented Modifier for `Template` on `Response`, so you can just use `response.set` to put set template into response and let it processed by our middleware.
//!
//! Also we made `Response` plugin for `Template` via `MaudEngine`. So you can test your handler from a test case, and retrieve the html you set into it by `response.get::<MaudEngine>`.
//!

extern crate iron;
extern crate plugin;

#[macro_use]
extern crate log;

pub use self::middleware::Template;
pub use self::middleware::MaudEngine;

mod middleware;
