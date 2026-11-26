# nsh-foundation

- Read [.agents/skills/karpathy-guidelines/SKILL.md](.agents/skills/karpathy-guidelines/SKILL.md) before implementing or reviewing changes.
- Keep the application, state and handlers in Rust. Use Native Shell's Rust APIs and Web target.
- Put reusable components in `crates/components`. The library must remain independent of the documentation app.
- For new or updated components, group base styles in functions that take and return `StyleRefinement`, and apply them with Native Shell's `.style_with(...)`. Example: `div().style_with(panel_style)`.
- Define public component inputs as named Rust props structs. Use Native Shell TSX-style tags and camelCase attributes in `rsx!` examples.
- Put the Web entry point, gallery layout, preview examples, and demonstration assets in `apps/docs`. Keep upstream sources and design references in `references`.
- Put tests that start the gallery in `apps/docs/tests` and standalone component tests in `crates/components/tests`.
- Use the upstream Base UI Drawer docs and pinned source listed in `references/README.md` as the behavior baseline.
- From the workspace root, verify changes with `cargo fmt --all --check`, `cargo test --workspace`, and `scripts/docs-web.sh`. For interaction changes, exercise applicable examples in a browser at desktop and mobile widths.
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

Keep the guide in sync with the [Rust snippet](apps/docs/src/examples/drawer_rust.rs),
[RSX example](apps/docs/src/examples/drawer_rsx.rs), and
[Native Shell TSX example](apps/docs/src/examples/drawer.tsx).

## Component documentation

Use a zero-argument builder when visible content belongs in children, as in
`button().child(text("Save"))` and `badge().child(text("In progress"))`.
Use `accessibility_label` for a name that the visible children cannot supply.
Preserve typed slots for compound components such as Card and Carousel.

Keep the Rust builder example and its RSX counterpart focused on the
component API. Use component tags when they exist and embed the actual Rust
builders when a component requires Rust state or typed parts. Copied RSX
examples must compile with the component library and Native Shell, without
importing the documentation app.

Put standalone Rust and RSX examples in a source file with a public
`example()` entry point. Declare it with
`documented_example!(name, "path.rs")` from the gallery macro module so
the compiler and code tab read the same file. Use the generated `CODE`
in the tab and use its **Run example** control to verify the copied code.
For larger gallery demos, display the actual implementation file.

Declare standalone TSX examples in
[`tsx_examples.in`](apps/docs/src/tsx_examples.in) and use the generated
`EXAMPLE` descriptor in the card. It supplies the source shown and copied
by the tab. `scripts/docs-web.sh` generates the Native Shell imports for
**Run example** from the same table. Examples that wrap an exported Rust
demo use [`gallery_examples.in`](apps/docs/src/gallery_examples.in) and
generate the displayed and compiled TSX from the same template.

The first demo on every component page must offer Preview, Rust, RSX,
and Native Shell TSX. Link to upstream React examples through Base UI
reference. Label TSX demo exports as such when the native module exposes
the Rust demo as a whole.

## Acceptance SOP

### 1. Define observable outcomes

- List the affected component routes, states, input methods, and breakpoints.
  Record the expected result for each case from the requested behavior and
  the repository's baseline. [Pinned references](references/README.md)
- Verify at a desktop width and a mobile width. Use 1280 × 900 and 390 × 844
  as starting points. For responsive changes, also check both sides of the
  actual breakpoint. For a regression, reproduce the original interaction
  in the affected example before checking the fix.

### 2. Write and verify the docs

- Put the primary interactive demo directly below the introduction. Its tabs
  are **Preview**, **Rust**, **RSX**, and **TSX**, in that order. Link to
  upstream material through **Base UI reference**.
- Start with a minimal runnable example, then show the variations that change
  usage or behavior. Include the required imports and registration, where
  needed. Explain state ownership, defaults, event payloads, and accessibility
  names where they matter. Keep the API reference tied to shipped names.
- Show the public Rust builders and real RSX component tags. Use Rust
  interpolations for typed parts or state when needed. In TSX, show the
  Native Shell API and identify any exported Rust demo by name so readers
  know what it renders.
- Open the first card and copy each source tab. Compare the clipboard with
  the intended source, compile the Rust and RSX samples, and type-check the
  TSX with `scripts/docs-web.sh check`. Run the TSX from its own tab and
  exercise its behavior. Check that hiding it and switching between demos
  place the right example in the right card. Make sure the initial preview
  illustrates the documented behavior.

### 3. Verify UI and animation frame by frame

- Observe the initial frame, the frame immediately after input, the active
  transition, the settling frame, and the final state. For a gesture, include
  finger down, movement before and after activation, release or cancel, and
  any snap or rebound. Record frames around a jump, flicker, or clipping issue
  as well as the endpoints.
- Use a browser recording or frame captures to inspect movement. For
  repeatable assertions, use `Clock::manual` with the headless harness,
  advance in small steps with `run_for` such as 16 ms samples around the
  transition, and inspect `screen().layout_nodes()`, scroll offsets, or
  accessibility state at each checkpoint. Keep motion
  enabled for the normal animation run. A screenshot or state assertion
  after settling covers the endpoint only.
- Verify direction, timing, easing or spring behavior, and final placement
  against the component's intended behavior. Check for one-frame flashes,
  layout shifts, clipping, or a stuck overlay. Exercise an interruption
  such as reopening during close or reversing a drag. Verify that focus,
  hit targets, and accessibility state are correct once the interaction
  settles. Repeat with reduced motion enabled.

### 4. Verify mobile gestures and input conflicts

- Test with touch input and report the device, emulator, or browser touch
  emulation. Verify OS-level gestures on the relevant platform. Cover the gestures
  the component supports, including edge swipes, drag handles, long press,
  and system back where applicable. Exercise slow drags, quick flicks,
  short moves below the threshold, reversals, and cancellation. Confirm
  that taps still work and the intended snap, dismiss, or rebound happens.
- Check nested scrolling in and across the gesture axis, including at both
  ends of the inner scroll range. Verify which surface consumes the input,
  when scrolling hands off, and whether the page moves or activates a
  control unexpectedly. Test desktop mouse drag, wheel or trackpad input,
  and keyboard behavior independently where supported.
- Where the component opens a mobile overlay, check safe areas, the on-screen
  keyboard, scroll locking, and access to close or confirm controls. Repeat
  with the relevant screen sizes and orientations if they change the layout.

### 5. Record the result

- Run the workspace checks listed above for code changes. Use the Web build
  under review for browser acceptance. Reuse the repository's regression
  patterns. [Drawer](apps/docs/tests/drawer.rs) ·
  [Carousel](apps/docs/tests/carousel.rs) · [Mobile edge swipes](apps/docs/tests/blocks.rs)
- For each affected route, record the build or commit, viewport, input method,
  test outcome, and any frame evidence relevant to the change. Mark each case
  as passed, failed, or unverified. List an unavailable browser or device
  separately from completed headless checks.
