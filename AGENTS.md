# nsh-foundation

- Read [.agents/skills/karpathy-guidelines/SKILL.md](.agents/skills/karpathy-guidelines/SKILL.md) before implementing or reviewing changes.
- Keep the application, state and handlers in Rust. Use Native Shell's Rust APIs and Web target.
- Put reusable components in `crates/components`. The library must remain independent of the documentation app.
- For new or updated components, group base styles in functions that take and return `StyleRefinement`, and apply them with Native Shell's `.style_with(...)`. Example: `div().style_with(panel_style)`.
- Define public component inputs as named Rust props structs. Use Native Shell TSX-style tags and camelCase attributes in `rsx!` examples.
- Put the Web entry point, gallery layout, preview examples, and demonstration assets in `apps/docs`. Keep upstream sources and design references in `references`.
- Put tests that start the gallery in `apps/docs/tests` and standalone component tests in `crates/components/tests`.
- Use the upstream Base UI Drawer docs and pinned source listed in `references/README.md` as the behavior baseline.
- From the workspace root, verify changes with `cargo fmt --all --check`, `cargo test --workspace`, and `nsh package --target web --out dist`. For interaction changes, exercise applicable examples in a browser at desktop and mobile widths.
- Keep downloaded upstream checkouts and generated builds out of Git.

## Drawer examples

Use these three forms when documenting Drawer. The trigger and close button
get their visible text from child elements. Set an accessible label on the
panel and on any control whose children do not provide an accessible name.

### Rust builders

Create the shared state with `Drawer::new(signal(false))` and pass it to
`DrawerRoot::new`. Pass a closure to `drawer_content().child(...)` so the body
builds when the drawer opens. Set the panel name with `accessibility_label`.

```rust
use components::{
    div,
    drawer::{
        Drawer, DrawerRoot, drawer_close, drawer_content, drawer_description,
        drawer_footer, drawer_header, drawer_title, drawer_trigger,
    },
};
use nsh_core::{element::StyleRefinement, prelude::*};

fn body_style(style: StyleRefinement) -> StyleRefinement {
    style.flex_1().gap_2()
}

fn example() -> impl IntoElement {
    let drawer = Drawer::new(signal(false));
    DrawerRoot::new(drawer, move || {
        div()
            .child(drawer_trigger().child(text("Open Drawer")))
            .child(
                drawer_content()
                    .accessibility_label("Move Goal")
                    .child(|| {
                        div().style_with(body_style)
                            .child(drawer_header()
                                .child(drawer_title().child(text("Move Goal")))
                                .child(drawer_description().child(text("Set your daily activity goal."))))
                            .child(drawer_footer()
                                .child(drawer_close().child(text("Done"))))
                    }),
            )
    })
}
```

### Rust RSX

Import `components::rsx` and use Native Shell TSX-style tags and camelCase
attributes. Braced expressions contain Rust. `<DrawerRoot>` creates its open
state by default. Pass `drawer={drawer}` to share an existing Rust state.

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

### Native Shell TSX

Use `Text` and `View` from `nsh`, and use the Drawer parts from
`nsh-foundation/native` in the TSX example.

```tsx
import { Text, View } from "nsh";
import {
  DrawerClose,
  DrawerContent,
  DrawerRoot,
  DrawerTrigger,
} from "nsh-foundation/native";

export function DrawerExample() {
  return (
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
  );
}
```

Keep the guide in sync with the [Rust snippet](apps/docs/src/docs.rs),
[RSX example](apps/docs/src/examples/drawer_rsx.rs), and
[Native Shell TSX example](apps/docs/src/examples/drawer.tsx).

## Component documentation

Use a zero-argument builder when visible content belongs in children, as in
`button().child(text("Save"))` and `badge().child(text("In progress"))`.
Use `accessibility_label` for a name that the visible children cannot supply.
Preserve typed slots for compound components such as Card and Carousel.

Keep each documented Rust builder example and its RSX counterpart in sync
with the preview. The composition examples for
[Button, Badge, Card, Carousel, and Switch](apps/docs/src/examples/composition) show
how to do this. Label Native Shell TSX examples as **TSX** and upstream
React examples as **shadcn/ui**. The first demo on every gallery component page must offer
Rust, RSX, and Native Shell TSX code. Use the
[gallery example exports](apps/docs/src/native_examples.rs) to share a
runnable Rust demo with RSX and TSX when the native module exposes the demo
as a whole. Document which API the example uses.
