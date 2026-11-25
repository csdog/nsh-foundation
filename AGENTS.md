# nsh-foundation

- Read [.agents/skills/karpathy-guidelines/SKILL.md](.agents/skills/karpathy-guidelines/SKILL.md) before implementing or reviewing changes.
- Keep the application, state and handlers in Rust. Use Native Shell's Rust APIs and Web target.
- Put reusable components in `crates/components`. The library must remain independent of the documentation app.
- For new or updated components, group base styles in functions that take and return `StyleRefinement`, and apply them with Native Shell's `.style_with(...)`. Example: `div().style_with(panel_style)`.
- Define public component inputs as named Rust props structs. In Rust and `rsx!` examples, use struct literals with field names, such as `DrawerTriggerProps { label: "Open Drawer" }`.
- Put the Web entry point, gallery layout, preview examples, and demonstration assets in `apps/docs`. Keep upstream sources and design references in `references`.
- Put tests that start the gallery in `apps/docs/tests` and standalone component tests in `crates/components/tests`.
- Use the upstream Base UI Drawer docs and pinned source listed in `references/README.md` as the behavior baseline.
- From the workspace root, verify changes with `cargo fmt --all --check`, `cargo test --workspace`, and `nsh package --target web --out dist`. For interaction changes, exercise applicable examples in a browser at desktop and mobile widths.
- Keep downloaded upstream checkouts and generated builds out of Git.
