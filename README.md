# nsh-foundation

A Native Shell project written in Rust. The Web target renders a responsive component documentation site and an interactive Drawer gallery.

## Run

Use the [Native Shell CLI](https://nsh.dev/docs/installation/) v0.6.1, Rust with the `wasm32-unknown-unknown` target, and Clang for compiling the Web runtime's C dependencies.

```sh
rustup target add wasm32-unknown-unknown
nsh package --target web --out dist
```

The CLI prints the packaged site's directory under `dist`. Use `nsh package --target web --run` to build and serve it locally, or deploy the generated directory as a static site.

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

Reusable state, positioning, gestures, and slots live in `src/drawer.rs`. The examples and documentation layout live in `src/examples.rs` and `src/docs.rs`. See [upstream references](references/README.md) for the shadcn/ui revision and Figma materials.

```sh
cargo fmt --check
cargo test
```
