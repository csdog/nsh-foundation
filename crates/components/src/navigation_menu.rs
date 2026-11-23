//! Site navigation with a shared preview panel and directional keyboard navigation.

use std::time::Duration;

use nsh_base::{link, theme};
use nsh_core::{
    element::LayoutDirection,
    keyboard::KeyCapture,
    keymap::{Keystroke, When},
    prelude::*,
};

use crate::direction::{Direction, use_direction};

#[derive(Clone)]
pub struct NavigationMenuLink {
    label: &'static str,
    href: &'static str,
    description: Option<&'static str>,
    icon: Option<&'static str>,
    current: bool,
}

impl NavigationMenuLink {
    pub fn new(label: &'static str, href: &'static str) -> Self {
        Self {
            label,
            href,
            description: None,
            icon: None,
            current: false,
        }
    }

    pub fn description(mut self, description: &'static str) -> Self {
        self.description = Some(description);
        self
    }

    pub fn icon(mut self, icon: &'static str) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn current(mut self, current: bool) -> Self {
        self.current = current;
        self
    }

    fn render(
        self,
        node: ElementRef,
        in_panel: bool,
        on_press: impl Fn() + 'static,
    ) -> impl IntoElement {
        let vertical = view()
            .min_w(px(0.0))
            .gap_1()
            .child(text(self.label).text_sm().font_weight(FontWeight::MEDIUM))
            .child(self.description.map(|description| {
                text(description)
                    .text_xs()
                    .line_height(px(17.0))
                    .text_color(|| theme().muted_foreground)
            }));
        link()
            .href(self.href)
            .accessibility_label(self.label)
            .current(self.current)
            .element_ref(&node)
            .on_press(on_press)
            .min_w(px(0.0))
            .min_h(px(32.0))
            .px(px(if in_panel { 8.0 } else { 10.0 }))
            .py(px(6.0))
            .rounded(px(if in_panel { 12.0 } else { 16.0 }))
            .hover(|style| style.bg(theme().muted))
            .focus_visible(|style| {
                style
                    .bg(theme().muted)
                    .ring(px(2.0))
                    .ring_color(theme().ring.opacity(0.4))
            })
            .when(self.current, |surface| {
                surface.bg(theme().muted.opacity(0.5))
            })
            .child(
                view()
                    .flex_row()
                    .items_center()
                    .gap_2()
                    .min_w(px(0.0))
                    .child(
                        self.icon
                            .map(|icon| text(icon).text_sm().accessibility_elements_hidden(true)),
                    )
                    .child(vertical),
            )
    }
}

pub struct NavigationMenuTrigger(&'static str);

impl NavigationMenuTrigger {
    pub fn new(label: &'static str) -> Self {
        Self(label)
    }
}

#[derive(Clone)]
pub struct NavigationMenuContent {
    links: Vec<NavigationMenuLink>,
    width: f32,
    columns: u16,
}

impl Default for NavigationMenuContent {
    fn default() -> Self {
        Self::new()
    }
}

impl NavigationMenuContent {
    pub fn new() -> Self {
        Self {
            links: Vec::new(),
            width: 384.0,
            columns: 1,
        }
    }

    pub fn link(mut self, link: NavigationMenuLink) -> Self {
        self.links.push(link);
        self
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = width.max(120.0);
        self
    }

    pub fn columns(mut self, columns: u16) -> Self {
        self.columns = columns.max(1);
        self
    }
}

enum Entry {
    Panel(NavigationMenuTrigger, NavigationMenuContent),
    Link(NavigationMenuLink),
}

pub struct NavigationMenuItem(Entry);

impl NavigationMenuItem {
    pub fn new(trigger: NavigationMenuTrigger, content: NavigationMenuContent) -> Self {
        Self(Entry::Panel(trigger, content))
    }

    pub fn link(link: NavigationMenuLink) -> Self {
        Self(Entry::Link(link))
    }
}

#[derive(Default)]
pub struct NavigationMenuList(Vec<NavigationMenuItem>);

impl NavigationMenuList {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn item(mut self, item: NavigationMenuItem) -> Self {
        self.0.push(item);
        self
    }
}

#[derive(Clone)]
struct Panel {
    label: &'static str,
    content: NavigationMenuContent,
    links: Vec<ElementRef>,
}

#[derive(Clone, Copy)]
struct MenuState {
    active: Signal<Option<usize>>,
    hover_revision: Signal<u64>,
    in_rail: Signal<bool>,
    focused_link: Signal<Option<usize>>,
    over_rail: Signal<bool>,
    over_panel: Signal<bool>,
    previous: Stored<Option<usize>>,
    pending_focus: Stored<Option<(usize, ElementRef)>>,
}

impl MenuState {
    fn new() -> Self {
        Self {
            active: signal(None),
            hover_revision: signal(0),
            in_rail: signal(false),
            focused_link: signal(None),
            over_rail: signal(false),
            over_panel: signal(false),
            previous: stored(None),
            pending_focus: stored(None),
        }
    }

    fn cancel_hover(self) -> u64 {
        let revision = self.hover_revision.get_untracked().wrapping_add(1);
        self.hover_revision.set(revision);
        revision
    }

    fn set(self, next: Option<usize>) {
        self.cancel_hover();
        self.pending_focus.with(|pending| {
            if pending.is_some_and(|(index, _)| Some(index) != next) {
                *pending = None;
            }
        });
        let before = self.active.get_untracked();
        if before != next {
            self.previous.replace(before);
            self.focused_link.set(None);
            self.active.set(next);
        }
    }

    fn hover(self, index: usize) {
        if !is_desktop() {
            return;
        }
        if self.active.get_untracked().is_some() {
            self.set(Some(index));
        } else {
            let revision = self.cancel_hover();
            set_timeout(Duration::from_millis(110), move || {
                if self.hover_revision.get_untracked() == revision {
                    self.set(Some(index));
                }
            });
        }
    }

    fn close_on_leave(self) {
        if !is_desktop() {
            return;
        }
        let revision = self.cancel_hover();
        set_timeout(Duration::from_millis(170), move || {
            if self.hover_revision.get_untracked() == revision
                && self.focused_link.get_untracked().is_none()
                && !self.over_rail.get_untracked()
                && !self.over_panel.get_untracked()
            {
                self.set(None);
            }
        });
    }

    fn close_on_focus_leave(self) {
        let Some(index) = self.active.get_untracked() else {
            return;
        };
        let revision = self.hover_revision.get_untracked();
        set_timeout(Duration::from_millis(50), move || {
            if self.active.get_untracked() == Some(index)
                && self.hover_revision.get_untracked() == revision
                && !self.in_rail.get_untracked()
                && self.focused_link.get_untracked().is_none()
            {
                self.set(None);
            }
        });
    }
}

pub struct NavigationMenu {
    label: &'static str,
    list: NavigationMenuList,
    indicator: bool,
}

impl NavigationMenu {
    pub fn new(label: &'static str, list: NavigationMenuList) -> Self {
        Self {
            label,
            list,
            indicator: false,
        }
    }

    pub fn indicator(mut self, show: bool) -> Self {
        self.indicator = show;
        self
    }
}

fn bind_navigation_keys(handler: impl Fn(&str) + Copy + 'static) {
    for key in ["left", "right", "up", "down", "home", "end"] {
        on_key_with(
            key,
            KeyOptions {
                for_focus: true,
                when: When::NotEditing,
                repeat: true,
                ..Default::default()
            },
            move |_| handler(key),
        );
    }
}

impl IntoElement for NavigationMenu {
    fn into_element(self) -> AnyElement {
        let rail = element_ref();
        let direction = use_direction();
        let state = MenuState::new();
        let focused = signal(None::<usize>);
        let entries = self.list.0;
        let count = entries.len();
        let indicator = self.indicator;
        let triggers = stored((0..count).map(|_| element_ref()).collect::<Vec<_>>());
        let panels = stored(
            entries
                .iter()
                .map(|item| match &item.0 {
                    Entry::Panel(trigger, content) => Some(Panel {
                        label: trigger.0,
                        content: content.clone(),
                        links: content.links.iter().map(|_| element_ref()).collect(),
                    }),
                    Entry::Link(_) => None,
                })
                .collect::<Vec<_>>(),
        );
        let keyboard_direction = stored(direction.clone());
        rail.on_focus_in(move || state.in_rail.set(true));
        rail.on_focus_out(move || {
            state.in_rail.set(false);
            state.close_on_focus_leave();
        });
        effect(move || {
            if count == 0 || !state.in_rail.get() || state.focused_link.get().is_some() {
                return;
            }
            bind_navigation_keys(move |key| {
                let Some(index) = focused.get_untracked() else {
                    return;
                };
                let rtl = keyboard_direction.with(|direction| direction.get()) == Direction::Rtl;
                let next = match key {
                    "left" if !rtl => (index + count - 1) % count,
                    "right" if rtl => (index + count - 1) % count,
                    "left" | "right" => (index + 1) % count,
                    "home" => 0,
                    "end" => count - 1,
                    "up" | "down" => {
                        let target = panels.with(|items| {
                            let panel = items[index].as_ref()?;
                            let link_index = if key == "up" {
                                panel.links.len().checked_sub(1)?
                            } else {
                                0
                            };
                            panel
                                .links
                                .get(link_index)
                                .map(|target| (link_index, *target))
                        });
                        if let Some((link_index, target)) = target {
                            let already_open = state.active.get_untracked() == Some(index);
                            state.set(Some(index));
                            state.focused_link.set(Some(link_index));
                            if already_open && target.node().is_some() {
                                target.focus();
                            } else {
                                state.pending_focus.replace(Some((index, target)));
                            }
                        }
                        return;
                    }
                    _ => return,
                };
                if state.active.get_untracked().is_some() {
                    state.set(panels.with(|items| items[next].as_ref().map(|_| next)));
                }
                triggers.with(|nodes| nodes[next]).focus();
            });
        });
        let overlay_direction = direction.clone();
        let rail_items = entries.into_iter().enumerate().map(|(index, item)| {
            let node = triggers.with(|nodes| nodes[index]);
            node.on_focus_in(move || focused.set(Some(index)));
            match item.0 {
                Entry::Panel(trigger, _) => {
                    trigger_surface(trigger, node, index, state, indicator).into_element()
                }
                Entry::Link(link) => view()
                    .on_mouse_enter(move |_: &MouseEvent| {
                        if is_desktop() {
                            state.set(None);
                        }
                    })
                    .child(link.render(node, false, move || state.set(None)))
                    .into_element(),
            }
        });

        view()
            .role(AccessibilityRole::Navigation)
            .label(self.label)
            .direction(direction.clone().map(LayoutDirection::from))
            .min_w(px(0.0))
            .child(
                view()
                    .element_ref(&rail)
                    .flex_row()
                    .flex_wrap()
                    .items_center()
                    .test_id("navigation-menu-list")
                    .on_mouse_enter(move |_: &MouseEvent| state.over_rail.set(true))
                    .on_mouse_leave(move |_: &MouseEvent| {
                        state.over_rail.set(false);
                        state.close_on_leave();
                    })
                    .children(rail_items),
            )
            .child(dynamic(move || {
                let index = state.active.get()?;
                let panel = panels.with(|items| items[index].clone())?;
                let previous = state.previous.get();
                let direction = overlay_direction.get();
                let trigger = triggers.with(|nodes| nodes[index]);
                Some(viewport(
                    panel, rail, trigger, state, index, previous, direction,
                ))
            }))
            .into_element()
    }
}

fn trigger_surface(
    trigger: NavigationMenuTrigger,
    node: ElementRef,
    index: usize,
    state: MenuState,
    indicator: bool,
) -> View {
    let open = move || state.active.get() == Some(index);
    view()
        .element_ref(&node)
        .role(AccessibilityRole::Button)
        .label(trigger.0)
        .relative()
        .flex_row()
        .items_center()
        .gap_1()
        .min_h(px(32.0))
        .px(px(10.0))
        .py(px(6.0))
        .rounded(px(16.0))
        .test_id("navigation-menu-trigger")
        .accessibility_state(move || AccessibilityState {
            expanded: Some(open()),
            ..Default::default()
        })
        .style_when(open, |style| style.bg(theme().muted.opacity(0.5)))
        .hover(|style| style.bg(theme().muted))
        .focus_visible(|style| {
            style
                .bg(theme().muted)
                .ring(px(2.0))
                .ring_color(theme().ring.opacity(0.4))
        })
        .on_mouse_enter(move |_: &MouseEvent| state.hover(index))
        .on_press(move || state.set((state.active.get_untracked() != Some(index)).then_some(index)))
        .child(
            text(trigger.0)
                .text_sm()
                .font_weight(FontWeight::MEDIUM)
                .selectable(false),
        )
        .child(
            text("⌄")
                .text_xs()
                .accessibility_elements_hidden(true)
                .style_when(open, |style| style.rotate(deg(180.0))),
        )
        .child(when(
            move || indicator && open(),
            || {
                view()
                    .absolute()
                    .bottom(px(-4.0))
                    .left(percentage(0.5))
                    .size(px(7.0))
                    .rotate(deg(45.0))
                    .bg(|| theme().border)
                    .accessibility_elements_hidden(true)
            },
        ))
}

fn viewport(
    panel: Panel,
    rail: ElementRef,
    trigger: ElementRef,
    state: MenuState,
    index: usize,
    previous: Option<usize>,
    direction: Direction,
) -> impl IntoElement {
    cleanup(move || {
        state.over_panel.try_set(false);
    });
    let count = panel.links.len();
    let refs = stored(panel.links);
    let cols = panel.content.columns;
    let width = panel.content.width;
    let tracks = move || if window().width.0 < 650.0 { 1 } else { cols };
    effect(move || {
        if count == 0 || state.focused_link.get().is_none() {
            return;
        }
        bind_navigation_keys(move |key| {
            let Some(pos) = state.focused_link.get_untracked() else {
                return;
            };
            let step = usize::from(tracks());
            let rtl = direction == Direction::Rtl;
            let next = match key {
                "down" => (pos + step).min(count - 1),
                "up" => pos.saturating_sub(step),
                "left" if !rtl => pos.saturating_sub(1),
                "right" if rtl => pos.saturating_sub(1),
                "left" | "right" => (pos + 1).min(count - 1),
                "home" => 0,
                "end" => count - 1,
                _ => return,
            };
            state.focused_link.set(Some(next));
            refs.with(|nodes| nodes[next]).focus();
        });
    });
    let mut rows = view().grid().grid_cols(tracks).gap(px(5.0)).p(px(6.0));
    for (i, link) in panel.content.links.into_iter().enumerate() {
        let node = refs.with(|nodes| nodes[i]);
        node.on_focus_in(move || {
            if state.active.get_untracked() == Some(index) {
                state.focused_link.set(Some(i));
                state.cancel_hover();
            }
        });
        node.on_focus_out(move || {
            if state.active.get_untracked() == Some(index)
                && state.focused_link.get_untracked() == Some(i)
            {
                state.focused_link.set(None);
                state.close_on_focus_leave();
            }
        });
        rows = rows.child(link.render(node, true, move || {
            state.set(None);
            trigger.focus();
        }));
    }
    let mut content = view().child(rows);
    if !reduce_motion() {
        let incoming = previous.map_or(0.0, |old| if old < index { 16.0 } else { -16.0 });
        let x = animated_value(if direction == Direction::Rtl {
            -incoming
        } else {
            incoming
        });
        let alpha = animated_value(0.0);
        let _ = x.timing(0.0, 180.0, Easing::Bezier(0.22, 1.0, 0.36, 1.0));
        let _ = alpha.timing(1.0, 140.0, Easing::Linear);
        content = content.bind_translate_x(&x).bind_opacity(&alpha);
    }

    overlay()
        .test_id("navigation-menu-viewport")
        .role(AccessibilityRole::Group)
        .label(panel.label)
        .direction(LayoutDirection::from(direction))
        .anchor(&rail)
        .placement(Placement::Bottom)
        .align(if direction == Direction::Rtl {
            Align::End
        } else {
            Align::Start
        })
        .offset(px(4.0))
        .margin(px(8.0))
        .w(move || px(width.min((window().width.0 - 16.0).max(1.0))))
        .rounded(px(16.0))
        .border(px(1.0))
        .border_color(|| theme().border)
        .bg(|| theme().popover)
        .text_color(|| theme().popover_foreground)
        .shadow_lg()
        .on_position(move |_: &OverlayPosition| {
            let target = state.pending_focus.with(|request| {
                if request.is_some_and(|(requested, target)| {
                    requested == index && target.node().is_some()
                }) {
                    request.take().map(|(_, target)| target)
                } else {
                    None
                }
            });
            if let Some(target) = target {
                target.focus();
            }
        })
        .on_mouse_enter(move |_: &MouseEvent| {
            state.over_panel.set(true);
            state.cancel_hover();
        })
        .on_mouse_leave(move |_: &MouseEvent| {
            state.over_panel.set(false);
            state.close_on_leave();
        })
        .on_press_outside(move |_: &Point<Points>| state.set(None))
        .on_cancel(move || {
            state.set(None);
            trigger.focus();
        })
        .capture_keys(KeyCapture::keys(Keystroke::parse("escape", os()).ok()))
        .on_key_down(move |event: &KeyEvent| {
            if event.default_prevented && event.key.as_ref() == "Escape" {
                state.set(None);
                trigger.focus();
            }
        })
        .child(content)
}

#[cfg(test)]
mod tests {
    use nsh_core::{
        element::Root,
        platform::{Idiom, Os},
    };
    use nsh_engine::EngineConfig;
    use nsh_testing::Headless;

    use super::*;

    #[component]
    fn SiteNavigation() -> impl IntoElement {
        NavigationMenu::new(
            "Site navigation",
            NavigationMenuList::new()
                .item(NavigationMenuItem::new(
                    NavigationMenuTrigger::new("First"),
                    NavigationMenuContent::new().link(NavigationMenuLink::new(
                        "Intro",
                        "https://example.com/intro",
                    )),
                ))
                .item(NavigationMenuItem::new(
                    NavigationMenuTrigger::new("Second"),
                    NavigationMenuContent::new()
                        .link(NavigationMenuLink::new("API", "https://example.com/api")),
                ))
                .item(NavigationMenuItem::link(NavigationMenuLink::new(
                    "Docs",
                    "https://example.com/docs",
                ))),
        )
    }

    #[test]
    fn keyboard_opens_the_next_panel_and_enters_a_real_link() {
        let mut config = EngineConfig::new(Root::of::<SiteNavigation>());
        config.environment.os = Os::Web;
        config.environment.idiom = Idiom::Desktop;
        config.environment.reduce_motion = true;
        let mut app = Headless::start(config).unwrap();
        app.first_render().unwrap();
        app.press_key("tab").unwrap();
        app.press_key("right").unwrap();
        app.press_key("enter").unwrap();
        app.wait_for("role:link[API]").unwrap();
        app.press_key("enter").unwrap();
        app.wait_for_gone("role:link[API]", Duration::from_secs(2))
            .unwrap();
        app.press_key("down").unwrap();
        app.wait_for("role:link[API]").unwrap();
        app.press_key("enter").unwrap();
        assert_eq!(app.screen().opened_urls(), ["https://example.com/api"]);

        app.press_key("end").unwrap();
        app.press_key("enter").unwrap();
        assert_eq!(
            app.screen().opened_urls(),
            ["https://example.com/api", "https://example.com/docs"]
        );
    }
}
