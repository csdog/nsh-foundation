//! Pairs a documentation snippet with the file compiled for its preview.

use nsh_core::prelude::AnyElement;

#[derive(Clone, Copy)]
pub(crate) struct DemoSource {
    pub text: &'static str,
    pub render: Option<fn() -> AnyElement>,
}

impl DemoSource {
    pub const fn text(text: &'static str) -> Self {
        Self { text, render: None }
    }
}

/// Declares a standalone example file with an `example()` entry point.
macro_rules! documented_example {
    ($name:ident, $path:literal) => {
        pub(crate) mod $name {
            mod source {
                include!($path);
            }

            pub(crate) use source::example;

            pub(crate) const CODE: $crate::demo_source::DemoSource =
                $crate::demo_source::DemoSource {
                    text: include_str!($path),
                    render: Some(|| nsh_core::prelude::IntoElement::into_element(example())),
                };
        }
    };
}

pub(crate) use documented_example;
