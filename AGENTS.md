# nsh-foundation

- Read [.agents/skills/karpathy-guidelines/SKILL.md](.agents/skills/karpathy-guidelines/SKILL.md) before implementing or reviewing changes.
- Keep the application, state and handlers in Rust. Use Native Shell's Rust APIs and Web target.
- Put reusable Drawer behavior in `src/drawer.rs`, example composition in `src/examples.rs`, and documentation layout in `src/docs.rs`.
- Use the upstream Base UI Drawer docs and pinned source listed in `references/README.md` as the behavior baseline.
- Verify changes with `cargo fmt --check`, `cargo test`, and `nsh package --target web --out dist`. For interaction changes, exercise applicable examples in a browser at desktop and mobile widths.
- Keep downloaded upstream checkouts and generated builds out of Git.
