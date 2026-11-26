//! TSX source descriptors used by the gallery code tabs.

use nsh_core::prelude::*;

use crate::WebGalleryEvent;

#[derive(Clone)]
pub(crate) struct TsxRunner {
    pub selected: Signal<Option<&'static str>>,
    pub slot: Prop<Option<Slot>>,
    pub scope: Scope<WebGalleryEvent>,
}

#[derive(Clone, Copy)]
pub(crate) struct TsxExample {
    pub id: &'static str,
    pub source: &'static str,
    pub byline: &'static str,
}

impl TsxExample {
    pub(crate) const fn gallery(id: &'static str, source: &'static str) -> Self {
        Self {
            id,
            source,
            byline: "Native Shell · renders the Rust gallery example",
        }
    }
}

macro_rules! tsx_examples {
    ($($id:ident => $path:literal, $export:ident;)*) => {
        $(pub(crate) mod $id {
            pub(crate) const EXAMPLE: super::TsxExample = super::TsxExample {
                id: stringify!($id),
                source: include_str!($path),
                byline: "Native Shell",
            };
        })*
    };
}

include!("tsx_examples.in");
