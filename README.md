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

Use the [Native Shell CLI](https://nsh.dev/docs/installation/) v0.6.1, Rust with the `wasm32-unknown-unknown` target, and Clang for compiling the Web runtime's C dependencies.

```sh
rustup target add wasm32-unknown-unknown
nsh package --target web --out dist
```

The CLI prints the packaged site's directory under `dist`. Use `nsh package --target web --run` to build and serve it locally, or deploy the generated directory as a static site.

Open `/#blocks` for Relay, the responsive chat block. Create and switch conversations, add files with the native picker, and send with Enter or the Send button. Shift+Enter starts a new line. Messages and attachment metadata stay in the current demo session.

If the CLI reports a stale generated package name after a crate rename, move aside `.nsh/rust` and run the package command again. Native Shell regenerates that directory.

## Compose a Drawer

Share a `Drawer` state across its trigger, content, and close action. Build
the content using the individual Rust elements.

```rust
use nsh_core::prelude::*;
use components::drawer::{
    Drawer, DrawerClose, DrawerContent, DrawerDescription, DrawerFooter, DrawerHeader,
    DrawerTitle, DrawerTrigger,
};

fn example() -> impl IntoElement {
    let drawer = Drawer::new(signal(false));
    view()
        .child(DrawerTrigger::new(drawer, "Open Drawer"))
        .child(DrawerContent::new(drawer, "Move Goal", move || {
            view()
                .child(DrawerHeader::new()
                    .child(DrawerTitle::new("Move Goal"))
                    .child(DrawerDescription::new("Set your daily activity goal.")))
                .child(DrawerFooter::new()
                    .child(DrawerClose::new(drawer, "Done")))
        }))
}
```

Use `DrawerTrigger::render` or `DrawerClose::render` to use your own
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
