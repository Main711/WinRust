// Compile without Cargo: rustc --edition=2024 --test plugins/topbar/tests/dock_geometry.rs
#[path = "../src/dock_geometry.rs"]
mod geometry;
use geometry::{Rect, dock_rect, popup_origin, secondary_monitors, should_hide};

const MAIN: Rect = Rect::new(0, 0, 2560, 1440);
const LEFT: Rect = Rect::new(-1920, 0, 0, 1080);

#[test]
fn current_primary_geometry_stays_unchanged() {
    assert_eq!(dock_rect(MAIN, 720, 40, 8), Some(Rect::new(512,1392,2048,1432)));
}
#[test]
fn second_dock_uses_its_own_bottom_and_signed_origin() {
    assert_eq!(dock_rect(LEFT, 720, 40, 8), Some(Rect::new(-1536,1032,-384,1072)));
    assert_eq!(dock_rect(Rect::new(1400,-960,2800,0), 1080, 60, 12), Some(Rect::new(1560,-72,2640,-12)));
}
#[test]
fn dock_clamps_width_and_rejects_invalid_or_too_short_display() {
    assert_eq!(dock_rect(Rect::new(0,0,600,800), 720, 40, 8), Some(Rect::new(0,752,600,792)));
    assert_eq!(dock_rect(Rect::new(0,0,600,45), 720, 40, 8), None);
    assert_eq!(dock_rect(Rect::new(1,0,0,800), 720, 40, 8), None);
}
#[test]
fn topology_is_clean_after_disconnect_clone_or_duplicate() {
    assert!(secondary_monitors(&[(MAIN,true)]).is_empty());
    assert!(secondary_monitors(&[(MAIN,false),(LEFT,false)]).is_empty());
    assert_eq!(secondary_monitors(&[(LEFT,false),(MAIN,true),(LEFT,false),(MAIN,false)]), vec![LEFT]);
    assert_eq!(secondary_monitors(&[(LEFT,true),(MAIN,false)]), vec![MAIN]);
}
#[test]
fn popups_stay_on_screen_even_above_primary_or_on_a_short_screen() {
    assert_eq!(popup_origin(LEFT, -2000, 840, 360, 200, 8), (-1912,840));
    assert_eq!(popup_origin(LEFT, -30, 1040, 360, 200, 8), (-368,872));
    assert_eq!(popup_origin(Rect::new(1400,-960,2800,0), 2500, -1700, 2000, 1200, 12), (1400,-960));
}
#[test]
fn fullscreen_wins_and_idle_hide_is_opt_in() {
    assert!(should_hide(true, false, true, true));
    assert!(!should_hide(false, false, true, false));
    assert!(!should_hide(false, true, false, false));
    assert!(should_hide(false, true, true, false));
    assert!(!should_hide(false, true, true, true));
}
