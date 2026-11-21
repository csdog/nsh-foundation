use nsh_core::prelude::*;

use crate::{
    badge::{Badge, BadgeVariant},
    button::{Button, ButtonSize, ButtonVariant},
    code_block::{CodeBlock, CodeBlockContent, CodeBlockSize, CodeBlockVariant},
    dialog::{Dialog, DialogContent, DialogDescription, DialogHeader, DialogTitle, DialogTrigger},
    sheet::{Sheet, SheetContent, SheetDescription, SheetHeader, SheetTitle, SheetTrigger},
    trace::{
        Trace, TraceCollapsible, TraceCollapsibleContent, TraceCollapsibleTrigger,
        TraceDescription, TraceIcon, TraceLine, TraceStatus, TraceTitle,
    },
};

mod replay;
pub use replay::{demo, thinking, tool_calls};

const FILE: &str = "https://github.com/shadcn-ui/ui/blob/f50887ae68d0fdcbfbe86b57be08af94a4c88a68/apps/v4/lib/docs.ts";
const ASTRO: &str = "https://docs.astro.build/en/install-and-setup/";

fn narrow() -> View {
    view().w_full().max_w(px(384.0)).min_w(px(0.0))
}

fn line(icon: &'static str, label: impl IntoReactive<SharedString>) -> TraceLine {
    let row = TraceLine::new(TraceTitle::new(label));
    if icon.is_empty() {
        row
    } else {
        row.icon(TraceIcon::glyph(icon))
    }
}

fn fold(row: TraceLine, content: impl IntoElement) -> TraceCollapsible {
    TraceCollapsible::new(
        TraceCollapsibleTrigger::new(row),
        TraceCollapsibleContent::new(content),
    )
}

fn note(value: impl IntoReactive<SharedString>) -> impl IntoElement {
    text(value).text_sm().line_height(px(22.0))
}

fn output(value: &'static str, error: bool) -> CodeBlock {
    CodeBlock::new(CodeBlockContent::new(value).wrap(true).max_height(256.0))
        .size(CodeBlockSize::Small)
        .variant(if error {
            CodeBlockVariant::Destructive
        } else {
            CodeBlockVariant::Default
        })
}

pub fn static_line() -> View {
    narrow().child(Trace::new().child(line("⑂", "Created branch fix/frozen-lockfile")))
}

pub fn status() -> View {
    narrow().child(Trace::new().children([
        line("✓", "Ran pnpm install"),
        line("!", "Failed to run pnpm lint").status(TraceStatus::Error),
        line("›", "Running pnpm build").status(TraceStatus::Running),
        line("○", "Run pnpm test").status(TraceStatus::Pending),
    ]))
}

pub fn description() -> View {
    narrow().child(
        Trace::new().children([
            line(
                "▤",
                "Edited components/settings/notification-preferences-form.tsx",
            )
            .description(TraceDescription::new("+48 −12")),
            line("›", "Ran pnpm test").description(TraceDescription::new("14s")),
            line("⌕", "Searched for useChat").description(TraceDescription::new("8 results")),
        ]),
    )
}

const NODE: &str = "$ node --version\nv24.18.0";
const PNPM: &str = "$ pnpm --version\n10.33.4";
const MISSING: &str = "command not found: pnpm";

pub fn collapsible() -> View {
    narrow().child(fold(line("›", "Ran node --version"), output(NODE, false)))
}

pub fn open() -> View {
    let open = signal(false);
    narrow()
        .items_start()
        .gap_4()
        .child(fold(line("›", "Ran node --version"), output(NODE, false)).default_open(true))
        .child(
            fold(line("›", "Ran pnpm --version"), output(PNPM, false))
                .open(open)
                .on_open_change(move |value| open.set(value)),
        )
        .child(
            Button::new(move || {
                SharedString::from(if open.get() {
                    "Hide the pnpm output"
                } else {
                    "Show the pnpm output"
                })
            })
            .variant(ButtonVariant::Outline)
            .size(ButtonSize::Small)
            .on_press(move || open.update(|value| *value = !*value)),
        )
}

pub fn link() -> View {
    narrow().child(
        Trace::new()
            .child(line("▤", "Read apps/v4/lib/docs.ts").link(FILE))
            .child(line("◎", "Install Astro | docs.astro.build").link(ASTRO)),
    )
}

const SEARCH: &str = "Searched the web for cache keys";
const RESULTS: [(&str, &str); 3] = [
    (
        "Caching build outputs",
        "https://nextjs.org/docs/app/guides/caching",
    ),
    (
        "Image optimization reference",
        "https://nextjs.org/docs/app/api-reference/components/image",
    ),
    (
        "Deploy hooks and cache keys",
        "https://vercel.com/docs/deploy-hooks",
    ),
];

fn source_rows(rows: &[(&'static str, &'static str)]) -> View {
    Trace::new()
        .children(rows.iter().map(|&(name, url)| line("◎", name).link(url)))
        .surface()
}

pub fn dialog() -> View {
    let dialog = Dialog::new(signal(false));
    narrow()
        .child(DialogTrigger::render(
            dialog,
            line("◎", SEARCH).indicator(TraceIcon::glyph("›")).trigger(),
        ))
        .child(
            DialogContent::new(
                dialog,
                DialogHeader::new(DialogTitle::new(SEARCH)).description(DialogDescription::new(
                    "Three results were used in the response.",
                )),
            )
            .body(|| source_rows(&RESULTS)),
        )
}

pub fn sheet() -> View {
    let sheet = Sheet::new(signal(false));
    const TITLE: &str = "Explored 4 files";
    let files = [
        "apps/v4/lib/docs.ts",
        "apps/v4/lib/components.ts",
        "apps/v4/next.config.mjs",
        "apps/v4/package.json",
    ];
    narrow()
        .child(SheetTrigger::render(
            sheet,
            line("▤", TITLE).indicator(TraceIcon::glyph("›")).trigger(),
        ))
        .child(
            SheetContent::new(
                sheet,
                SheetHeader::new(SheetTitle::new(TITLE)).description(SheetDescription::new(
                    "Files read while preparing the response.",
                )),
            )
            .body(move || {
                view()
                    .gap_2()
                    .children(files.map(|file| text(file).text_xs().font_family("monospace")))
            }),
        )
}

pub fn group() -> View {
    narrow().child(Trace::new().children([
        line("⌕", "Searched for minimumReleaseAge"),
        line("▤", "Read pnpm-workspace.yaml"),
        line("›", "Ran pnpm install"),
    ]))
}

const SOURCES: [(&str, &str); 3] = [
    ("Install Astro | docs.astro.build", ASTRO),
    (
        "create-astro | npmjs.com",
        "https://www.npmjs.com/package/create-astro",
    ),
    (
        "Settings: minimumReleaseAge | pnpm.io",
        "https://pnpm.io/settings#minimumreleaseage",
    ),
];

pub fn sources() -> View {
    narrow().child(fold(line("◎", "Used 3 sources"), source_rows(&SOURCES)).default_open(true))
}

pub fn steps() -> View {
    narrow().child(
        fold(
            line("", "Looking into the slow deploy").status(TraceStatus::Running),
            Trace::new()
                .child(fold(
                    line("⌕", "Searched for the checkout build log"),
                    view()
                        .flex_row()
                        .flex_wrap()
                        .gap_2()
                        .child(Badge::new("vercel.com").variant(BadgeVariant::Secondary))
                        .child(Badge::new("github.com").variant(BadgeVariant::Secondary)),
                ))
                .child(
                    line("▤", "Read the last twelve deploys")
                        .link("https://vercel.com/docs/deployments"),
                )
                .child(fold(
                    line("⌕", "Searching for the cache key").status(TraceStatus::Running),
                    output("cache key image optimization", false),
                )),
        )
        .default_open(true),
    )
}

const DIFFS: [(&str, bool, &str, &str, &str); 2] = [
    (
        "app/login/page.tsx",
        false,
        "+2 −2",
        "2 additions, 2 deletions",
        "@@ -14,7 +14,7 @@ export default async function LoginPage({\n   searchParams,\n }) {\n   const { next } = await searchParams\n-  const redirectTo =\n-    next && next.startsWith(\"/\") ? next : \"/\"\n+  const redirectTo = safeRedirect(next)\n+\n   return <LoginForm redirectTo={redirectTo} />\n }",
    ),
    (
        "lib/auth/redirect.ts",
        true,
        "+7 −0",
        "7 additions, 0 deletions",
        "@@ -0,0 +1,7 @@\n+export function safeRedirect(next: string | undefined) {\n+  if (!next || !next.startsWith(\"/\") || next.startsWith(\"//\")) {\n+    return \"/\"\n+  }\n+\n+  return next\n+}",
    ),
];

pub fn commit() -> View {
    let files = Trace::new().children(DIFFS.map(|(file, added, count, description, diff)| {
        let sheet = Sheet::new(signal(false));
        view()
            .child(SheetTrigger::render(
                sheet,
                line(
                    if added { "+" } else { "▤" },
                    SharedString::from(format!(
                        "{} {file}",
                        if added { "Added" } else { "Edited" }
                    )),
                )
                .description(TraceDescription::new(count))
                .indicator(TraceIcon::glyph("›"))
                .trigger(),
            ))
            .child(
                SheetContent::new(
                    sheet,
                    SheetHeader::new(SheetTitle::new(file))
                        .description(SheetDescription::new(description)),
                )
                .body(move || {
                    view().child(CodeBlock::new(
                        CodeBlockContent::new(diff).diff().wrap(true),
                    ))
                }),
            )
    }));
    narrow().child(
        fold(line("⊙", "Committed a1b2c3: Fix the login redirect"), files).default_open(true),
    )
}

pub fn turn() -> View {
    let steps = Trace::new()
        .child(fold(
            line("◌", "Thought").description(TraceDescription::new("12s")),
            note("I should check which package managers are installed before I scaffold anything."),
        ))
        .child(
            fold(
                line("›", "Ran 2 commands, 1 failed").status(TraceStatus::Error),
                Trace::new()
                    .child(line("›", "Ran node --version"))
                    .child(fold(
                        line("›", "Failed to run pnpm --version").status(TraceStatus::Error),
                        output(MISSING, true),
                    )),
            )
            .default_open(true),
        )
        .child(note(
            "Node is installed and pnpm is not. Reading the docs config.",
        ))
        .child(line("▤", "Read apps/v4/lib/docs.ts").link(FILE));
    narrow().child(
        fold(
            line("", "Worked for 2m 4s, 1 command failed").status(TraceStatus::Error),
            steps,
        )
        .default_open(true),
    )
}
