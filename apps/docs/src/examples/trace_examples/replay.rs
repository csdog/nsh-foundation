use std::time::Duration;

use components::{
    bubble::{Bubble, BubbleAlign, BubbleContent, BubbleVariant},
    message::{Message, MessageAlign, MessageContent},
    spinner::Spinner,
};

use super::*;

#[derive(Clone, Copy)]
struct Command {
    name: &'static str,
    output: &'static str,
    error: bool,
}

const CI: [Command; 3] = [
    Command {
        name: "pnpm --version",
        output: "10.18.0",
        error: false,
    },
    Command {
        name: "git diff --stat HEAD~1 -- package.json pnpm-lock.yaml",
        output: " package.json | 2 +-\n 1 file changed, 1 insertion(+), 1 deletion(-)",
        error: false,
    },
    Command {
        name: "pnpm install --frozen-lockfile",
        output: "ERR_PNPM_OUTDATED_LOCKFILE  Cannot install with \"frozen-lockfile\" because pnpm-lock.yaml is not up to date with package.json",
        error: true,
    },
];
const VERSIONS: [Command; 3] = [
    Command {
        name: "node --version",
        output: "v24.18.0",
        error: false,
    },
    Command {
        name: "npm --version",
        output: "11.6.2",
        error: false,
    },
    Command {
        name: "pnpm --version",
        output: MISSING,
        error: true,
    },
];

fn command_row(
    command: Command,
    phase: Signal<usize>,
    start: usize,
    default_open: bool,
) -> impl IntoElement {
    let running = computed(move || phase.get() == start);
    when(
        move || phase.get() >= start,
        move || {
            dynamic(move || {
                if running.get() {
                    line("", SharedString::from(format!("Running {}", command.name)))
                        .status(TraceStatus::Running)
                        .icon(TraceIcon::new(Spinner::new().decorative()))
                        .into_element()
                } else {
                    let header = if command.error {
                        "Failed to run"
                    } else {
                        "Ran"
                    };
                    fold(
                        line(
                            if command.error { "!" } else { "›" },
                            SharedString::from(format!("{header} {}", command.name)),
                        )
                        .status(if command.error {
                            TraceStatus::Error
                        } else {
                            TraceStatus::Default
                        }),
                        output(command.output, command.error),
                    )
                    .default_open(default_open)
                    .into_element()
                }
            })
        },
    )
}

fn thought(
    phase: Signal<usize>,
    short: &'static str,
    full: &'static str,
    completed: &'static str,
) -> impl IntoElement {
    let requested = signal(None::<bool>);
    let running = move || phase.get() < 3;
    let open = move || requested.get().unwrap_or_else(running);
    fold(
        line("◌", move || {
            SharedString::from(if running() { "Thinking" } else { completed })
        })
        .status(move || {
            if running() {
                TraceStatus::Running
            } else {
                TraceStatus::Default
            }
        }),
        note(move || SharedString::from(if phase.get() == 1 { short } else { full })),
    )
    .open(open)
    .on_open_change(move |open| requested.set(Some(open)))
}

fn question(text: &'static str) -> Message {
    Message::new(
        MessageContent::new().child(
            Bubble::new(BubbleContent::text(text))
                .variant(BubbleVariant::Muted)
                .align(BubbleAlign::End),
        ),
    )
    .align(MessageAlign::End)
}

fn answer(trace: Trace, response: &'static str, phase: Signal<usize>, last: usize) -> Message {
    Message::new(
        MessageContent::new()
            .child(trace)
            .child(when(move || phase.get() >= last, move || note(response))),
    )
}

fn play(phase: Signal<usize>, last: usize, delay: u64) {
    phase.set(1);
    spawn(async move {
        for next in 2..=last {
            timer(Duration::from_millis(delay)).await;
            phase.set(next);
        }
    });
}

fn replay_button(phase: Signal<usize>, last: usize, replay: bool, delay: u64) -> impl IntoElement {
    let owner = Owner::current();
    when(
        move || phase.get() == 0 || phase.get() == last,
        move || {
            Button::new(move || {
                SharedString::from(if replay {
                    "Replay"
                } else if phase.get() == 0 {
                    "Send message"
                } else {
                    "Reset"
                })
            })
            .variant(ButtonVariant::Outline)
            .size(ButtonSize::Small)
            .surface()
            .self_start()
            .on_press(move || {
                if !replay && phase.get_untracked() == last {
                    phase.set(0);
                } else if phase.get_untracked() == 0 || phase.get_untracked() == last {
                    owner.run(|| play(phase, last, delay));
                }
            })
        },
    )
}

pub fn demo() -> View {
    const LAST: usize = 7;
    let phase = signal(LAST);
    narrow().min_h(px(288.0)).py_6().gap_4()
        .child(question("Why does pnpm install fail in CI?"))
        .child(when(move || phase.get() > 0, move || {
            let trace = Trace::new()
                .child(thought(
                    phase,
                    "CI installs with a frozen lockfile, so the install stops when package.json and pnpm-lock.yaml disagree.",
                    "CI installs with a frozen lockfile, so the install stops when package.json and pnpm-lock.yaml disagree. I should check what the last commit changed, then reproduce the install.",
                    "Thought for 5s",
                ))
                .children(CI.into_iter().enumerate().map(move |(index, command)| command_row(command, phase, index + 3, index == 1)))
                .child(when(move || phase.get() >= 6, move || {
                    fold(
                        line("◎", move || SharedString::from(if phase.get() == 6 { "Using 2 sources" } else { "Used 2 sources" }))
                            .status(move || if phase.get() == 6 { TraceStatus::Running } else { TraceStatus::Default }),
                        source_rows(&[
                            ("pnpm install | pnpm.io", "https://pnpm.io/cli/install#--frozen-lockfile"),
                            ("Continuous Integration | pnpm.io", "https://pnpm.io/continuous-integration"),
                        ]),
                    )
                }));
            answer(trace, "The last commit bumped a version in package.json without updating pnpm-lock.yaml, so the frozen install fails. Run pnpm install locally and commit the lockfile.", phase, LAST)
        }))
        .child(replay_button(phase, LAST, true, 650))
}

pub fn thinking() -> View {
    const LAST: usize = 3;
    let phase = signal(0);
    narrow().min_h(px(288.0)).py_6().gap_4()
        .child(when(move || phase.get() > 0, move || {
            view().gap_4()
                .child(question("Why is the checkout deploy so slow?"))
                .child(answer(
                    Trace::new().child(thought(
                        phase,
                        "The build log shows install and compile finish in under a minute.",
                        "The build log shows install and compile finish in under a minute. Image optimization takes almost four, and it reruns on every deploy because the cache key includes the commit hash.",
                        "Thought",
                    )),
                    "Image optimization is the bottleneck. Key its cache on the lockfile and the image directory instead of the commit hash.",
                    phase,
                    LAST,
                ))
        }))
        .child(replay_button(phase, LAST, false, 850))
}

pub fn tool_calls() -> View {
    const LAST: usize = 4;
    let phase = signal(0);
    narrow()
        .min_h(px(288.0))
        .py_6()
        .gap_4()
        .child(when(
            move || phase.get() > 0,
            move || {
                view()
                    .gap_4()
                    .child(question("Which package managers are installed?"))
                    .child(answer(
                        Trace::new().children(VERSIONS.into_iter().enumerate().map(
                            move |(index, command)| command_row(command, phase, index + 1, false),
                        )),
                        "Node 24 and npm 11 are installed. pnpm is not.",
                        phase,
                        LAST,
                    ))
            },
        ))
        .child(replay_button(phase, LAST, false, 900))
}
