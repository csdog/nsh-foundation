use nsh_base::theme;
use nsh_core::{AccessibilityValue, prelude::*};

use crate::{
    card::{Card, CardContent},
    dialog::{Dialog, DialogClose, DialogContent, DialogHeader, DialogTitle, DialogTrigger},
    native_select::{NativeSelect, NativeSelectOption},
    questionnaire::{
        Questionnaire, QuestionnaireAnswers, QuestionnaireChoice as Choice, QuestionnaireError,
        QuestionnaireInput as Input, QuestionnaireItem as Item, QuestionnaireProgress,
        QuestionnaireShortcuts as Shortcuts,
    },
    ui::muted,
};

macro_rules! choices {
    ($($value:literal => $label:literal),+ $(,)?) => {
        &[$(Choice::new($value, $label)),+]
    };
}

const DEMO: &[Item] = &[
    Item::new("direction", "What should the agent build next?")
        .description("Choose a direction or describe another task.")
        .choices(&[
            Choice::new("tool-calls", "Tool call timeline").description("Show what the agent ran and what came back."),
            Choice::new("approvals", "Approval checkpoints").description("Ask before sensitive or destructive actions."),
            Choice::new("handoffs", "Sub-agent handoffs").description("Make delegated work and results easier to follow."),
        ])
        .input(Input::new("Another agent feature", "Describe another feature…")),
    Item::new("signals", "What should every progress update include?")
        .description("Select all that apply, or skip this question.")
        .optional().multiple()
        .choices(choices!["progress" => "Progress", "decisions" => "Decisions", "risks" => "Risks", "next-step" => "Next step"]),
    Item::new("timing", "When should work begin?")
        .description("Choose when the agent should begin the work.")
        .choices(choices!["now" => "Start now", "next-cycle" => "Next development cycle", "backlog" => "Add it to the backlog"]),
];

const MULTIPLE: &[Item] = &[
    Item::new("context", "What context should the agent inspect?")
        .description("Select every source that may affect the implementation.")
        .multiple()
        .choices(choices!["source" => "Relevant source files", "tests" => "Existing tests", "docs" => "Architecture documentation", "history" => "Recent commit history"]),
];

const FREEFORM: &[Item] = &[
    Item::new("approach", "How should the agent approach this refactor?")
        .description("Choose a strategy or write a more specific instruction.")
        .choices(choices!["incremental" => "Make the smallest safe change", "module" => "Refactor one module at a time", "rewrite" => "Replace the implementation completely"])
        .input(Input::new("Another refactoring approach", "Describe another approach…")),
];

const SKIP: &[Item] = &[
    Item::new("task", "What kind of change is this?")
        .description("Choose the category that best describes the work.")
        .choices(choices!["feature" => "New feature", "fix" => "Bug fix", "refactor" => "Refactor"]),
    Item::new("constraints", "Are there any implementation constraints?")
        .description("Answer if needed, or intentionally skip this question.")
        .optional().multiple()
        .choices(choices!["no-dependencies" => "Do not add dependencies", "no-migrations" => "Do not change the database", "preserve-api" => "Preserve the public API"])
        .input(Input::new("Another implementation constraint", "Describe another constraint…")),
    Item::new("review", "How should the work be reviewed?")
        .description("Choose the checks the agent should complete before handoff.")
        .choices(choices!["tests" => "Run the test suite", "diff" => "Review the final diff", "both" => "Tests and diff review"]),
];

const SHORTCUTS: &[Item] = &[
    Item::new("action", "What should the agent do next?")
        .description("Use the displayed shortcut or navigate with the keyboard.")
        .choices(choices!["inspect" => "Inspect the implementation", "tests" => "Run the relevant tests", "patch" => "Prepare the patch"]),
];

const VALIDATION: &[Item] = &[
    Item::new("detail", "How much detail should the answer include?")
        .description("Choose the response depth.")
        .choices(&[
            Choice::new("summary", "Concise summary")
                .description("Best for a quick internal update."),
            Choice::new("complete", "Complete answer")
                .description("Keep context, decisions, and verification."),
        ]),
    Item::new("audience", "Who will read the answer?")
        .description("Public answers require complete context.")
        .choices(choices!["team" => "Internal team", "public" => "Public audience"]),
];

const CONTROLLED: &[Item] = &[
    Item::new("scope", "What may the agent change?")
        .description("The host stores the active checkpoint while Questionnaire navigates.")
        .choices(choices!["component" => "Only the target component", "tests" => "Component and related tests", "feature" => "The complete feature area"]),
    Item::new("checks", "Which verification level should it use?")
        .choices(choices!["targeted" => "Targeted tests", "package" => "Package tests and typecheck", "full" => "Full workspace verification"]),
    Item::new("output", "What should the agent return when finished?")
        .choices(choices!["summary" => "Concise summary", "diff" => "Summary with changed files", "handoff" => "Detailed implementation handoff"]),
];

const RESUME: &[Item] = &[
    Item::new("change", "What kind of migration is this?")
        .description("This answer was saved during the previous session.")
        .choices(choices!["incremental" => "Incremental migration", "cutover" => "Single cutover"]),
    Item::new("verification", "How should the migration be verified?")
        .description("These checks were selected during the previous session.")
        .multiple()
        .choices(choices!["tests" => "Run migration tests", "typecheck" => "Run the typecheck", "manual" => "Perform a manual smoke test"]),
    Item::new("notes", "Anything else the agent should remember?")
        .description("This note was saved with the draft.")
        .optional()
        .input(Input::new("Saved migration note", "")),
];

const CONDITIONAL: &[Item] = &[
    Item::new("runtime", "Where should the agent run?")
        .description("Cloud runs add an environment question to this flow.")
        .choices(choices!["local" => "Local workspace", "cloud" => "Cloud environment"]),
    Item::new("environment", "Which cloud environment should it use?")
        .enabled_when(|answers| answers.first("runtime") == Some("cloud"))
        .choices(choices!["preview" => "Preview", "staging" => "Staging", "isolated" => "Isolated sandbox"]),
    Item::new("approval", "When should the agent request approval?")
        .choices(choices!["writes" => "Before writing files", "commands" => "Before running commands", "sensitive" => "Only for sensitive actions"]),
];

const NAVIGATION: &[Item] = &[
    Item::new("permission", "What may the agent modify?")
        .description("Next is intentionally disabled until an answer is selected.")
        .choices(choices!["files" => "Project files", "tests" => "Project files and tests", "config" => "Files, tests, and configuration"]),
    Item::new("verification", "What must pass before completion?")
        .choices(choices!["tests" => "Tests", "types" => "Tests and types", "all" => "Tests, types, and visual QA"]),
];

const PROGRESS: &[Item] = &[
    Item::new("scope", "How large is the change?")
        .choices(choices!["small" => "Small patch", "medium" => "Feature-sized change", "large" => "Cross-package change"]),
    Item::new("strategy", "How should commits be organized?")
        .choices(choices!["single" => "Single commit", "logical" => "Logical commits", "squash" => "Squash before review"]),
    Item::new("tests", "Which tests should run?")
        .choices(choices!["targeted" => "Targeted tests", "package" => "Package suite", "workspace" => "Full workspace"]),
    Item::new("delivery", "How should the work be delivered?")
        .choices(choices!["patch" => "Patch only", "commit" => "Committed locally", "branch" => "Push a review branch"]),
];

const ANIMATED: &[Item] = &[
    Item::new("task", "What should the agent do?")
        .description("Choose the task for this run.")
        .choices(choices!["implement" => "Implement the requested change", "debug" => "Debug the current behavior", "review" => "Review the implementation"]),
    Item::new("review", "How should the work be reviewed?")
        .description("Select the verification depth.")
        .choices(choices!["targeted" => "Targeted checks", "complete" => "Complete test suite", "manual" => "Tests and manual QA"]),
    Item::new("delivery", "How should the result be delivered?")
        .description("Choose the final handoff format.")
        .choices(choices!["summary" => "Concise summary", "diff" => "Summary and changed files", "handoff" => "Detailed review handoff"]),
];

const CARD: &[Item] = &[
    Item::new("task", "What should the agent work on?")
        .description("Choose the task that should be handled next.")
        .choices(choices!["fix" => "Fix the failing tests", "refactor" => "Refactor the data layer", "docs" => "Update the integration guide"]),
    Item::new("output", "What should the final handoff include?")
        .description("Pick the level of detail needed for review.")
        .choices(choices!["summary" => "Summary only", "files" => "Summary and changed files", "review" => "Full review handoff"]),
];

const DIALOG: &[Item] = &[
    Item::new("scope", "Which files are in scope?")
        .description("Choose how broadly the agent can update the workspace.")
        .choices(choices!["component" => "Component only", "feature" => "Complete feature directory", "workspace" => "Any related workspace file"]),
    Item::new("tests", "How much verification is needed?")
        .description("Choose the checks the agent should run before handoff.")
        .choices(choices!["targeted" => "Targeted tests", "package" => "Package tests", "full" => "Full workspace verification"]),
];

fn feedback(answers: QuestionnaireAnswers, title: &str) -> String {
    let values = answers
        .iter()
        .map(|(name, answers)| format!("{name}: {}", answers.join(", ")))
        .collect::<Vec<_>>()
        .join(" · ");
    if values.is_empty() {
        title.to_owned()
    } else {
        format!("{title}. {values}")
    }
}

fn status(value: Signal<Option<String>>) -> impl IntoElement {
    dynamic(move || {
        value.get().map(|value| {
            muted(value)
                .role(AccessibilityRole::Summary)
                .test_id("questionnaire-response")
        })
    })
}

fn preview(questionnaire: Questionnaire, title: &'static str) -> View {
    let result = signal(None);
    view()
        .w_full()
        .max_w(px(448.0))
        .gap_4()
        .child(questionnaire.on_submit(move |answers| result.set(Some(feedback(answers, title)))))
        .child(status(result))
}

pub fn demo() -> View {
    preview(
        Questionnaire::new(DEMO)
            .shortcuts(Shortcuts::Letters)
            .submit_label("Save plan"),
        "Agent brief submitted",
    )
}

pub fn multiple() -> View {
    preview(
        Questionnaire::new(MULTIPLE)
            .shortcuts(Shortcuts::Letters)
            .submit_label("Share context"),
        "Context shared",
    )
}

pub fn freeform() -> View {
    preview(
        Questionnaire::new(FREEFORM)
            .shortcuts(Shortcuts::Letters)
            .submit_label("Use this approach"),
        "Approach selected",
    )
}

pub fn skip() -> View {
    preview(
        Questionnaire::new(SKIP).submit_label("Submit brief"),
        "Brief submitted",
    )
}

pub fn shortcuts() -> View {
    let mode = signal(Some(SharedString::from("letters")));
    let picker = NativeSelect::new("Shortcut mode")
        .value(mode)
        .option(NativeSelectOption::new("none", "No shortcuts"))
        .option(NativeSelectOption::new("letters", "Letters"))
        .option(NativeSelectOption::new("numbers", "Numbers"));
    view()
        .w_full()
        .max_w(px(448.0))
        .gap_4()
        .child(view().w(px(160.0)).child(picker))
        .child(preview(
            Questionnaire::new(SHORTCUTS)
                .shortcuts(move || {
                    mode.with(|mode| match mode.as_deref() {
                        Some("letters") => Shortcuts::Letters,
                        Some("numbers") => Shortcuts::Numbers,
                        _ => Shortcuts::None,
                    })
                })
                .submit_label("Confirm action"),
            "Action confirmed",
        ))
}

pub fn validation() -> View {
    preview(
        Questionnaire::new(VALIDATION)
            .validate(|answers| {
                if answers.first("detail") == Some("summary")
                    && answers.first("audience") == Some("public")
                {
                    Err(QuestionnaireError::new(
                        "detail",
                        "Public answers need enough context. Choose a complete answer.",
                    ))
                } else {
                    Ok(())
                }
            })
            .submit_label("Validate answers"),
        "Agent response configured",
    )
}

pub fn controlled() -> View {
    let item = signal(0_usize);
    view()
        .w_full()
        .max_w(px(448.0))
        .gap_4()
        .child(muted(move || {
            format!(
                "Current checkpoint: {}",
                ["Scope", "Verification", "Output"][item.get()]
            )
        }))
        .child(preview(
            Questionnaire::new(CONTROLLED)
                .active_item(item)
                .submit_label("Save workflow"),
            "Workflow saved",
        ))
}

pub fn resume() -> View {
    preview(
        Questionnaire::new(RESUME)
            .default_item("verification")
            .default_answers("change", &["incremental"])
            .default_answers("verification", &["tests", "typecheck"])
            .default_answers("notes", &["Keep the existing public API stable."])
            .reset_action(true)
            .skip_action(false)
            .submit_label("Update draft"),
        "Draft updated",
    )
}

pub fn conditional() -> View {
    preview(
        Questionnaire::new(CONDITIONAL).submit_label("Save execution plan"),
        "Execution plan saved",
    )
}

pub fn navigation_state() -> View {
    preview(
        Questionnaire::new(NAVIGATION)
            .disable_unanswered(true)
            .submit_label("Save permissions"),
        "Permissions saved",
    )
}

fn segments(step: QuestionnaireProgress) -> View {
    let caption = format!("Checkpoint {} of {}", step.current, step.total);
    view()
        .gap_2()
        .role(AccessibilityRole::Progressbar)
        .label(caption.clone())
        .accessibility_value(Some(AccessibilityValue {
            min: Some(0.0),
            max: Some(step.total as f32),
            now: Some(step.current as f32),
            text: None,
        }))
        .child(
            view()
                .flex_row()
                .gap_1()
                .accessibility_elements_hidden(true)
                .children((0..step.total).map(move |index| {
                    view().flex_1().h(px(6.0)).rounded_full().bg(move || {
                        if index < step.current {
                            theme().primary
                        } else {
                            theme().muted
                        }
                    })
                })),
        )
        .child(muted(caption).text_xs())
}

pub fn progress() -> View {
    preview(
        Questionnaire::new(PROGRESS)
            .render_progress(segments)
            .submit_label("Finish plan"),
        "Plan finished",
    )
}

pub fn animated() -> View {
    preview(
        Questionnaire::new(ANIMATED)
            .animated(true)
            .submit_label("Save workflow"),
        "Workflow saved",
    )
}

pub fn card() -> View {
    view().w_full().max_w(px(448.0)).child(
        Card::new().content(CardContent::new(preview(
            Questionnaire::new(CARD)
                .shortcuts(Shortcuts::Numbers)
                .submit_label("Create task"),
            "Task created",
        ))),
    )
}

pub fn dialog() -> View {
    let dialog = Dialog::new(signal(false));
    let result = signal(None);
    view()
        .items_center()
        .gap_4()
        .child(DialogTrigger::new(dialog, "Open clarification"))
        .child(status(result))
        .child(
            DialogContent::new(dialog, DialogHeader::new(DialogTitle::new("Clarification"))).body(
                move || {
                    view()
                        .gap_3()
                        .child(
                            Questionnaire::new(DIALOG)
                                .submit_label("Send answer")
                                .on_submit(move |answers| {
                                    result.set(Some(feedback(answers, "Clarification sent")));
                                    dialog.close();
                                }),
                        )
                        .child(view().items_end().child(DialogClose::new(dialog, "Cancel")))
                },
            ),
        )
}
