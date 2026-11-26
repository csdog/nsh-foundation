# nsh-foundation

A Native Shell component library written in Rust. Its responsive Web gallery includes component previews with Rust and TSX tabs. Follow the [component plan](PLAN.md) and [release progress](PROGRESS.md).

## Workspace

| Path | Responsibility |
| --- | --- |
| [crates/components](crates/components) | Reusable Rust components and standalone component tests. |
| [apps/docs](apps/docs) | Native Shell `App` entry point, gallery layout, previews, demo assets, and gallery tests. |
| [references](references) | Upstream TSX samples and design references. |

To consume the components from another project, add `components` as a path dependency pointing to `crates/components`. Import components through `components`. The gallery entry point is `docs::App`, selected by [nsh.json](nsh.json).

## Run

Use the [Native Shell CLI](https://nsh.dev/docs/installation/) v0.7.0, Rust with the `wasm32-unknown-unknown` target, and Clang for compiling the Web runtime's C dependencies.
If there is no release binary for your platform, install the pinned CLI with `cargo install --locked nsh-cli --version '=0.7.0'`.

```sh
rustup target add wasm32-unknown-unknown
nsh package --target web --out dist
```

The CLI prints the packaged site's directory under `dist`. Use `nsh package --target web --run` to build and serve it locally, or deploy the generated directory as a static site.

Open `/#blocks` for Relay, the responsive chat block. Create and switch conversations, add files with the native picker, and send with Enter or the Send button. Shift+Enter starts a new line. Messages and attachment metadata stay in the current demo session.

If the CLI reports a stale generated package name after a crate rename, move aside `.nsh/rust` and run the package command again. Native Shell regenerates that directory.

## Compose a Drawer

Pass the shared state to `DrawerRoot::new` once. Set component inputs with
named fields and apply reusable style functions with `style_with`.
In `rsx!`, `=> { ... }` supplies a closure so the drawer body is built
only when it opens.

```rust
use components::{
    div,
    drawer::{
        Drawer, DrawerCloseProps, DrawerContentProps, DrawerDescriptionProps,
        DrawerFooter, DrawerHeader, DrawerRoot, DrawerTitleProps, DrawerTrigger,
    },
    rsx,
};
use nsh_core::{element::StyleRefinement, prelude::*};

fn body_style(style: StyleRefinement) -> StyleRefinement {
    style.flex_1().gap_2()
}

fn example() -> impl IntoElement {
    let drawer = Drawer::new(signal(false));
    rsx! {
        DrawerRoot::new(drawer) => {
            div() {
                DrawerTrigger {
                    accessibility_label: "Move Goal".into(),
                    children: vec![text("Move Goal").into_element()],
                },
                DrawerContentProps { label: "Move Goal" } => {
                    (div().style_with(body_style)) {
                        DrawerHeader::new() {
                            DrawerTitleProps { text: "Move Goal" },
                            DrawerDescriptionProps { text: "Set your daily activity goal." },
                        },
                        DrawerFooter::new() {
                            DrawerCloseProps { label: "Done" },
                        },
                    }
                },
            }
        }
    }
}
```

In plain Rust, use `drawer_trigger().child(text("Open Drawer"))` inside
the `DrawerRoot::new` closure. Use `drawer_content().child(|| ...)` for
the lazy body. Compose the heading with `drawer_header().child(...)` and
its `drawer_title()` and `drawer_description()` children. Group actions with
`drawer_footer().child(...)` and use `drawer_close()` to close the panel.
Set `accessibility_label`
when those contents do not name the button, such as an icon-only trigger. Use
`DrawerTrigger::render` or `DrawerClose::render` to use your own
Native Shell view as the button. `DrawerContent` creates the `DrawerPortal`
and `DrawerOverlay`, and exposes `swipe_handle` to style its interactive
`DrawerSwipeHandle`.

## Drawer examples

| Example | Behavior |
| --- | --- |
| Delivery time | Select a slot and confirm. The drawer opens on the right on desktop and from the bottom on mobile. |
| Position | Opens from any of the four edges. |
| Swipe Handle | Drag the bottom panel toward the edge to dismiss it. |
| Nested | Open four layers and return to the previous layer when one closes. |
| Non Modal | Keeps the background interactive and stays open when you click outside. |
| Snap Points | Drag or operate the handle to switch between 31 rem and full height. |
| Responsive | Edit a profile in a centered dialog from 768 px upward and a bottom drawer on narrower windows. |

Drawer state, positioning, gestures, and slots live in the component library. The gallery owns the Drawer examples and page layout. [Drawer component](crates/components/src/drawer.rs) · [Drawer previews](apps/docs/src/examples/drawer.rs) · [Gallery layout](apps/docs/src/docs.rs) · [Upstream references](references/README.md)

```sh
cargo fmt --all --check
cargo test --workspace
```
