/// Composes Native Shell elements with nested, comma-separated children.
///
/// A call followed by `{ ... }` adds each child via `.child(...)`. A call
/// followed by `=> { ... }` passes a `move` closure as its final argument.
/// A struct literal followed by `=> { ... }` passes that closure to `.with_body(...)`.
/// Any Rust expression can be used as a leaf or as a parenthesized root builder.
#[macro_export]
macro_rules! rsx {
    (@children ($parent:expr) $(,)?) => {
        $parent
    };
    (@children ($parent:expr) $($call:ident)::+ ($($args:tt)*) {$($children:tt)*}, $($rest:tt)*) => {
        $crate::rsx!(@children (
            $parent.child($crate::rsx!($($call)::+ ($($args)*) {$($children)*}))
        ) $($rest)*)
    };
    (@children ($parent:expr) $($call:ident)::+ ($($args:tt)*) => {$($body:tt)*}, $($rest:tt)*) => {
        $crate::rsx!(@children (
            $parent.child($crate::rsx!($($call)::+ ($($args)*) => {$($body)*}))
        ) $($rest)*)
    };
    (@children ($parent:expr) $($props:ident)::+ {$($fields:tt)*} => {$($body:tt)*}, $($rest:tt)*) => {
        $crate::rsx!(@children (
            $parent.child($crate::rsx!($($props)::+ {$($fields)*} => {$($body)*}))
        ) $($rest)*)
    };
    (@children ($parent:expr) $child:expr, $($rest:tt)*) => {
        $crate::rsx!(@children ($parent.child($child)) $($rest)*)
    };
    ($($call:ident)::+ ($($args:tt)*) {$($children:tt)*}) => {
        $crate::rsx!(@children ($($call)::+ ($($args)*)) $($children)*,)
    };
    (($parent:expr) {$($children:tt)*}) => {
        $crate::rsx!(@children ($parent) $($children)*,)
    };
    ($($call:ident)::+ ($($args:expr),* $(,)?) => {$($body:tt)*}) => {
        $($call)::+ ($($args,)* move || $crate::rsx! {$($body)*})
    };
    ($($props:ident)::+ {$($fields:tt)*} => {$($body:tt)*}) => {
        ($($props)::+ {$($fields)*}).with_body(move || $crate::rsx! {$($body)*})
    };
    ($element:expr) => {
        $element
    };
}
