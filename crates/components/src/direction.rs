//! Scoped layout direction and a reactive getter for descendant components.

use nsh_core::{
    element::{LayoutDirection, layout_rtl},
    prelude::*,
};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Direction {
    #[default]
    Ltr,
    Rtl,
}

impl Direction {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Ltr => "ltr",
            Self::Rtl => "rtl",
        }
    }
}

impl From<Direction> for LayoutDirection {
    fn from(direction: Direction) -> Self {
        match direction {
            Direction::Ltr => Self::Ltr,
            Direction::Rtl => Self::Rtl,
        }
    }
}

impl IntoReactive<Direction> for Direction {
    fn into_reactive(self) -> Reactive<Direction> {
        Reactive::Static(self)
    }
}

#[derive(Clone)]
struct DirectionContext(Reactive<Direction>);

pub struct DirectionProvider<F> {
    direction: Reactive<Direction>,
    content: F,
}

impl<F> DirectionProvider<F> {
    /// Builds the children inside the direction scope.
    pub fn new(direction: impl IntoReactive<Direction>, content: F) -> Self {
        Self {
            direction: direction.into_reactive(),
            content,
        }
    }
}

impl<F, E> IntoElement for DirectionProvider<F>
where
    F: Fn() -> E + 'static,
    E: IntoElement,
{
    fn into_element(self) -> AnyElement {
        dynamic(move || {
            provide_context(DirectionContext(self.direction.clone()));
            view()
                .direction(self.direction.clone().map(LayoutDirection::from))
                .child((self.content)())
        })
        .into_element()
    }
}

/// Reads the nearest provider, or the native surface direction.
pub fn use_direction() -> Reactive<Direction> {
    use_context::<DirectionContext>().map_or_else(
        || {
            (|| {
                if layout_rtl() {
                    Direction::Rtl
                } else {
                    Direction::Ltr
                }
            })
            .into_reactive()
        },
        |context| context.0,
    )
}

#[cfg(test)]
mod tests {
    use nsh_core::{Size, platform::Os};
    use nsh_engine::EngineConfig;
    use nsh_testing::Headless;

    use super::*;

    fn status(label: &'static str) -> impl IntoElement {
        let direction = use_direction();
        text(move || format!("{label}: {}", direction.get().as_str()))
    }

    fn row(ids: [&'static str; 2]) -> View {
        view()
            .flex_row()
            .children(ids.map(|id| view().size(px(40.0)).test_id(id)))
    }

    #[component]
    fn DirectionScopes() -> impl IntoElement {
        let direction = signal(Direction::Rtl);
        view()
            .child(status("Surface"))
            .child(
                view()
                    .label("Switch direction")
                    .role(AccessibilityRole::Button)
                    .on_press(move || direction.set(Direction::Ltr))
                    .child(text("Switch direction")),
            )
            .child(DirectionProvider::new(direction, || {
                view()
                    .child(status("Outer"))
                    .child(row(["outer-first", "outer-last"]))
                    .child(DirectionProvider::new(Direction::Ltr, || {
                        view()
                            .child(status("Inner"))
                            .child(row(["inner-first", "inner-last"]))
                    }))
            }))
            .child(status("Sibling"))
    }

    #[test]
    fn providers_are_reactive_and_nested_overrides_do_not_leak() {
        let mut config = EngineConfig::new(nsh_testing::windowed_root::<DirectionScopes>());
        config.viewport = Size::new(px(600.0), px(500.0));
        config.environment.os = Os::Linux;
        let mut app = Headless::start(config).unwrap();
        app.first_render().unwrap();
        for expected in ["Surface: ltr", "Outer: rtl", "Inner: ltr", "Sibling: ltr"] {
            app.wait_for(&format!("text:{expected}")).unwrap();
        }
        let nodes = |app: &Headless, id| {
            app.screen()
                .layout_nodes()
                .into_iter()
                .find(|node| node.test_id.as_deref() == Some(id))
                .unwrap()
        };
        assert!(
            nodes(&app, "outer-first").bounds.origin.x > nodes(&app, "outer-last").bounds.origin.x
        );
        assert!(
            nodes(&app, "inner-first").bounds.origin.x < nodes(&app, "inner-last").bounds.origin.x
        );
        let outer = nodes(&app, "outer-first").id;
        app.press("role:button[Switch direction]").unwrap();
        app.wait_for("text:Outer: ltr").unwrap();
        assert!(
            nodes(&app, "outer-first").bounds.origin.x < nodes(&app, "outer-last").bounds.origin.x
        );
        assert_eq!(nodes(&app, "outer-first").id, outer);
        assert!(app.shows("Inner: ltr"));
        assert!(app.shows("Sibling: ltr"));
    }
}
