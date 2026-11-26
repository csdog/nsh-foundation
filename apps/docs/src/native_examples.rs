//! Reusable Rust and Native Shell exports for the gallery's runnable examples.

use nsh_core::prelude::*;

use crate::demo_source::{DemoSource, documented_example};
use crate::examples::*;
use crate::tsx_examples::TsxExample;

include!("gallery_tsx.in");

pub(crate) struct ExampleSource {
    pub rsx: DemoSource,
    pub tsx: TsxExample,
}

fn toast_preview() -> impl IntoElement {
    let toaster = components::toast::Toaster::new();
    view()
        .child(toast_examples::demo(toaster))
        .child(toaster.viewport())
}

macro_rules! gallery_examples {
    ($($page:ident $component:ident $factory:ident $label:literal => $render:path, $rsx:literal;)*) => {
        $(
            documented_example!($factory, $rsx);

            #[doc = concat!("Builds the gallery's ", stringify!($page), " example.")]
            #[doc = concat!("\n```no_run\n", include_str!($rsx), "```")]
            pub fn $factory() -> impl IntoElement {
                $render()
            }
        )*

        mod bindings {
            use super::*;

            $(
                #[doc = concat!("Renders the gallery's ", stringify!($page), " example.")]
                #[component]
                pub fn $component() -> impl IntoElement {
                    super::$factory()
                }
            )*
        }

        pub(crate) fn register(registry: &mut Registry) {
            $(registry.register::<bindings::$component>();)*
        }

        pub(crate) fn source(label: &str) -> Option<ExampleSource> {
            match label {
                $(
                    $label => Some(ExampleSource {
                        rsx: $factory::CODE,
                        tsx: TsxExample::gallery(stringify!($component), gallery_tsx!($component)),
                    }),
                )*
                _ => None,
            }
        }

    };
}

include!("gallery_examples.in");
