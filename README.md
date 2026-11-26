# nsh-foundation

A Native Shell component library written in Rust. Every component page
includes Rust, Rust RSX, and Native Shell TSX examples. The **shadcn/ui**
tab shows the upstream React reference where available.

The [gallery example exports](apps/docs/src/native_examples.rs) let RSX and
Native Shell TSX render the same runnable Rust demo. To embed them in a
Native Shell TSX project, add `apps/docs` as a local native dependency and
import the example from `nsh-foundation/native`. The Drawer, Button, Badge,
and Switch examples also demonstrate their direct Native Shell bindings.
Follow the [component plan](PLAN.md) and [release progress](PROGRESS.md).

## Workspace

| Path | Responsibility |
| --- | --- |
| [crates/components](crates/components) | Reusable Rust components and standalone component tests. |
| [crates/rsx-macro](crates/rsx-macro) | TSX-style `rsx!` procedural macro. |
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

Use `components::rsx!` with the same tag and attribute spelling as the
Native Shell TSX example. Use `{expression}` to embed Rust values.
`DrawerRoot` creates the open state by default, and `DrawerContent` builds
its children when the drawer opens. Pass `drawer={drawer}` to share explicit
open state with Rust code.

```rust
use components::rsx;
use nsh_core::prelude::IntoElement;

pub fn example() -> impl IntoElement {
    rsx! {
        <DrawerRoot>
            <DrawerTrigger>
                <Text>Open Drawer</Text>
            </DrawerTrigger>
            <DrawerContent accessibilityLabel="Move Goal">
                <View style={{ padding: 16, gap: 8 }}>
                    <Text style={{ fontSize: 20 }}>Move Goal</Text>
                    <Text>Set your daily activity goal.</Text>
                    <DrawerClose>
                        <Text class="text-primary-foreground">Done</Text>
                    </DrawerClose>
                </View>
            </DrawerContent>
        </DrawerRoot>
    }
}
```

Inline style objects accept numeric `padding`, `paddingHorizontal`,
`paddingVertical`, `gap`, `fontSize`, `width`, `height`, and `borderRadius` values. Use `style={body_style}` to apply a Rust
`StyleRefinement` function for other styles.

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
