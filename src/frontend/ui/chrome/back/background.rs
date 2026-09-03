use iced::widget::canvas::{Frame, Path, Stroke};
use iced::{Point, Rectangle, Size};

use crate::frontend::scene::BackPanel;
use crate::frontend::theme::Palette;
use crate::frontend::ui::with_alpha;

use super::geometry::BackGeometry;
use super::layout::BackLayout;

fn rectangle(tuple: (f32, f32, f32, f32)) -> Rectangle {
    Rectangle::new(Point::new(tuple.0, tuple.1), Size::new(tuple.2, tuple.3))
}

#[cfg(test)]
pub(super) fn embedded_section_span(
    panel: &BackPanel,
    layout: &BackLayout,
    section: (f32, f32, f32, f32),
    y: f32,
) -> (f32, f32) {
    let (card_x, card_y, card_width, card_height) = layout.card;
    let skew = panel.skew;
    let t = ((y - card_y) / card_height.max(1.0)).clamp(0.0, 1.0);
    let card_left = card_x + skew.max(0.0) * (1.0 - t) - skew.min(0.0) * t;
    let card_right = card_x + card_width + skew.min(0.0) * (1.0 - t) - skew.max(0.0) * t;
    let left_inset = (section.0 - card_x).max(0.0);
    let right_inset = (card_x + card_width - section.0 - section.2).max(0.0);
    (card_left + left_inset, card_right - right_inset)
}

fn embedded_section(
    panel: &BackPanel,
    layout: &BackLayout,
    section: Rectangle,
    radii: [f32; 4],
) -> Path {
    BackGeometry::new(panel, layout).rounded_rectangle(section, radii)
}

#[cfg(test)]
pub(super) fn embedded_section_points(
    panel: &BackPanel,
    layout: &BackLayout,
    section: Rectangle,
) -> [Point; 4] {
    let geometry = BackGeometry::new(panel, layout);
    [
        geometry.point(section.position()),
        geometry.point(Point::new(section.x + section.width, section.y)),
        geometry.point(Point::new(section.x + section.width, section.y + section.height)),
        geometry.point(Point::new(section.x, section.y + section.height)),
    ]
}

pub(super) fn embedded_section_radii(
    panel: &BackPanel,
    layout: &BackLayout,
    section: Rectangle,
    top: bool,
    bottom: bool,
) -> [f32; 4] {
    let card = rectangle(layout.card);
    let left = (section.x - card.x).max(0.0);
    let right = (card.x + card.width - section.x - section.width).max(0.0);
    let top_inset = (section.y - card.y).max(0.0);
    let bottom_inset = (card.y + card.height - section.y - section.height).max(0.0);
    [
        if top { (panel.radii[0] - left.max(top_inset)).max(0.0) } else { 0.0 },
        if top { (panel.radii[1] - right.max(top_inset)).max(0.0) } else { 0.0 },
        if bottom { (panel.radii[2] - right.max(bottom_inset)).max(0.0) } else { 0.0 },
        if bottom { (panel.radii[3] - left.max(bottom_inset)).max(0.0) } else { 0.0 },
    ]
}

pub(super) fn draw_surface(
    frame: &mut Frame,
    palette: &Palette,
    panel: &BackPanel,
    layout: &BackLayout,
    fade: f32,
) {
    let card = rectangle(layout.card);
    let masthead = rectangle(layout.masthead);
    let sheet = rectangle(layout.sheet);

    if panel.embedded {
        let masthead_path = embedded_section(
            panel,
            layout,
            masthead,
            embedded_section_radii(panel, layout, masthead, true, false),
        );
        let sheet_path = embedded_section(
            panel,
            layout,
            sheet,
            embedded_section_radii(panel, layout, sheet, false, true),
        );
        frame.fill(&masthead_path, with_alpha(palette.surface, 0.78 * fade));
        frame.fill(&sheet_path, with_alpha(palette.surface, 0.92 * fade));
        frame.stroke(
            &masthead_path,
            Stroke::default().with_color(with_alpha(palette.outline, 0.48 * fade)).with_width(1.0),
        );
        frame.stroke(
            &sheet_path,
            Stroke::default().with_color(with_alpha(palette.outline, 0.58 * fade)).with_width(1.0),
        );
    } else {
        frame.fill(
            &Path::rectangle(masthead.position(), masthead.size()),
            with_alpha(palette.background, 0.90 * fade),
        );
        frame.fill(
            &Path::rectangle(sheet.position(), sheet.size()),
            with_alpha(palette.background, 0.90 * fade),
        );
        frame.stroke(
            &Path::line(
                Point::new(sheet.x + sheet.width, sheet.y),
                Point::new(sheet.x + sheet.width, sheet.y + sheet.height),
            ),
            Stroke::default().with_color(with_alpha(palette.outline, 0.46 * fade)).with_width(1.0),
        );
        frame.stroke(
            &Path::rectangle(card.position(), card.size()),
            Stroke::default().with_color(with_alpha(palette.outline, 0.72 * fade)).with_width(1.0),
        );
    }

    if let Some((from, to)) = masthead_divider(masthead, panel.embedded) {
        frame.stroke(
            &Path::line(from, to),
            Stroke::default().with_color(with_alpha(palette.primary, 0.48 * fade)).with_width(1.0),
        );
    }
}

pub(super) fn masthead_divider(masthead: Rectangle, embedded: bool) -> Option<(Point, Point)> {
    if embedded {
        return None;
    }
    let rule_y = masthead.y + masthead.height;
    Some((Point::new(masthead.x, rule_y), Point::new(masthead.x + masthead.width, rule_y)))
}
