#![cfg(test)]

use crate::app::tests::{browser_item, test_app};
use crate::frontend::browser::{Browser, Source};
use crate::frontend::scene::layout::GridParams;

#[test]
fn viewport_fit_fills_both_axes_and_preserves_gap_proportions() {
    let mut app = test_app();
    let wall = &mut app.source_browser.wall;
    wall.set_layout(GridParams {
        cols: 4,
        rows: 4,
        thumb_w: 100.0,
        thumb_h: 60.0,
        gap_x: 8.0,
        gap_y: 8.0,
        ..GridParams::default()
    });

    wall.set_viewport(212.0, 264.0);

    assert!((wall.scene.gp.thumb_w - 50.0).abs() < 0.001);
    assert!((wall.scene.gp.thumb_h - 63.0).abs() < 0.001);
    assert!((wall.scene.gp.gap_x - 4.0).abs() < 0.001);
    assert!((wall.scene.gp.gap_y - 4.0).abs() < 0.001);
    assert!((wall.scene.gp.total_w() - 212.0).abs() < 0.001);
    assert!((wall.scene.gp.total_h() - 264.0).abs() < 0.001);
}

#[test]
fn reopening_browser_refits_layout_without_a_viewport_change() {
    let mut app = test_app();
    let wall = &mut app.source_browser.wall;
    let grid = GridParams { cols: 6, rows: 3, ..GridParams::default() };
    wall.set_layout(grid);
    wall.set_viewport(1552.0, 546.0);
    let fitted = wall.scene.gp;

    wall.set_layout(grid);
    wall.set_viewport(1552.0, 546.0);

    assert!(wall.scene.gp.settled_to(&fitted));
    assert!((wall.scene.gp.total_w() - 1552.0).abs() < 0.001);
    assert!((wall.scene.gp.total_h() - 546.0).abs() < 0.001);

    wall.set_layout(GridParams { thumb_w: 400.0, ..grid });
    assert!((wall.scene.gp.total_w() - 1552.0).abs() < 0.001);
    assert!((wall.scene.gp.total_h() - 546.0).abs() < 0.001);
    wall.set_viewport(2400.0, 1400.0);
    wall.set_layout(grid);
    assert!((wall.scene.gp.total_w() - 2400.0).abs() < 0.001);
    assert!((wall.scene.gp.total_h() - 1400.0).abs() < 0.001);
    assert_eq!(wall.layout_grid().thumb_w, grid.thumb_w);
    assert_eq!(wall.layout_grid().thumb_h, grid.thumb_h);
}

#[test]
fn browser_rows_and_columns_fill_the_results_frame_at_different_aspect_ratios() {
    let mut app = test_app();
    let wall = &mut app.source_browser.wall;
    for (cols, rows) in [(6, 3), (4, 4), (1, 1)] {
        wall.set_layout(GridParams { cols, rows, ..GridParams::default() });
        for (width, height) in [(1552.0, 546.0), (1248.0, 728.0), (400.0, 900.0), (1.0, 1.0)] {
            wall.set_viewport(width, height);
            assert!((wall.scene.gp.total_w() - width).abs() < 0.001);
            assert!((wall.scene.gp.total_h() - height).abs() < 0.001);
            assert!(wall.scene.gp.thumb_w > 0.0);
            assert!(wall.scene.gp.thumb_h > 0.0);
        }
    }
}

#[test]
fn begin_session_keeps_atlas() {
    let mut app = test_app();
    let mut browser = Browser::new(Source::Wallhaven);
    browser.session.items.push(browser_item("w1"));

    let wall = &mut app.source_browser.wall;
    wall.rebuild_catalogue(&browser, true);
    assert_eq!(wall.catalog.items.len(), 1);
    assert_eq!(wall.filtered, vec![0]);
    {
        let atlas = wall.atlas.as_mut().expect("remote atlas");
        atlas.near.acquire(0);
        atlas.near.mark_ready(0);
    }

    wall.begin_session();

    let atlas = wall.atlas.as_ref().expect("remote atlas");
    assert!(atlas.near.ready(0).is_some());
    wall.rebuild_catalogue(&browser, true);
    assert_eq!(wall.catalog.items.len(), 1);
    assert_eq!(wall.filtered, vec![0]);
}

#[test]
fn source_tabs_restore_each_browser_session() {
    let mut app = test_app();
    let mut wallhaven = Browser::new(Source::Wallhaven);
    wallhaven.request.query = String::from("forest");
    wallhaven.session.items.push(browser_item("wall"));
    app.source_browser.browser = Some(wallhaven);

    assert!(app.source_browser.activate(Source::Steam));
    app.source_browser.browser.as_mut().unwrap().request.query = String::from("rain");
    app.source_browser.browser.as_mut().unwrap().session.items.push(browser_item("steam"));

    assert!(app.source_browser.activate(Source::Wallhaven));
    let active = app.source_browser.browser.as_ref().unwrap();
    assert_eq!(active.request.query, "forest");
    assert_eq!(active.session.items[0].id, "wall");
    let steam = app.source_browser.tabs.get(&Source::Steam).unwrap();
    assert_eq!(steam.request.query, "rain");
    assert_eq!(steam.session.items[0].id, "steam");
}
