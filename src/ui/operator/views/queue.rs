use clay_layout::{grow, id::Id};

use crate::ui::design::Scope;
use crate::ui::design::components::list::text_item;
use crate::ui::design::components::panel::{PanelProps, panel};
use crate::ui::design::components::scroll_area::{ScrollIds, scroll_area};
use crate::ui::design::metrics::{Metrics, Role};
use crate::ui::design::theme::Theme;
use crate::ui::operator::action::Panel;
use crate::ui::operator::frame::Frame;
use crate::ui::operator::views::verses::state_of;

/// Cola de versos preparados de antemano.
pub fn view<'render>(
    c: &mut Scope<'render>,
    metrics: &Metrics,
    theme: &Theme,
    frame: &'render Frame<'render>,
    ids: ScrollIds,
    selected: Id,
) {
    let focused = frame.focus == Panel::Queue;

    panel(
        c,
        metrics,
        theme,
        PanelProps {
            title: Some("Cola"),
            focused,
            width: grow!(),
            height: grow!(),
        },
        |c| {
            if frame.queue.is_empty() {
                c.text(
                    "Vacia · + encola",
                    metrics.text_no_wrap(Role::Label, theme.muted),
                );
                return;
            }

            scroll_area(c, metrics, ids, frame.scroll.queue, 0, |c| {
                for (index, cite) in frame.queue.iter().enumerate() {
                    text_item(
                        c,
                        metrics,
                        theme,
                        state_of(index == frame.queue_index, focused),
                        (index == frame.queue_index).then_some(selected),
                        cite,
                    );
                }
            });
        },
    );
}
