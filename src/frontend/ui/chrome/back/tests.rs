#![cfg(test)]

use iced::{Point, Rectangle, Size};

use crate::frontend::scene::BackPanel;

use super::background::{embedded_section_radii, masthead_divider};
use super::layout::back_layout;

#[test]
fn masthead_divider_embedded() {
    let masthead = Rectangle::new(Point::new(190.0, 105.0), Size::new(900.0, 32.0));
    assert_eq!(masthead_divider(masthead, true), None);
    let (from, to) = masthead_divider(masthead, false).expect("divider");
    assert_eq!(from, Point::new(190.0, 137.0));
    assert_eq!(to, Point::new(1090.0, 137.0));
}

#[test]
fn embedded_sections_preserve_matching_square_and_rounded_corners() {
    let panel = BackPanel {
        embedded: true,
        cx: 500.0,
        cy: 400.0,
        hw: 260.0,
        hh: 340.0,
        edge_tilt: 72.0,
        radii: [48.0, 0.0, 76.0, 0.0],
        ..BackPanel::default()
    };
    let layout = back_layout(&panel);
    let rect = |tuple: (f32, f32, f32, f32)| {
        Rectangle::new(Point::new(tuple.0, tuple.1), Size::new(tuple.2, tuple.3))
    };
    let masthead = embedded_section_radii(&panel, &layout, rect(layout.masthead), true, false);
    let sheet = embedded_section_radii(&panel, &layout, rect(layout.sheet), false, true);

    assert!(masthead[0] > 0.0);
    assert_eq!(masthead[1], 0.0);
    assert_eq!(masthead[2..], [0.0, 0.0]);
    assert_eq!(sheet[..2], [0.0, 0.0]);
    assert!(sheet[2] > 0.0);
    assert_eq!(sheet[3], 0.0);
}
