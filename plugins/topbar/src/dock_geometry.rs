//! Pure desktop geometry. All rectangles use the virtual desktop's signed coordinates.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct Rect {
    pub(crate) left: i32,
    pub(crate) top: i32,
    pub(crate) right: i32,
    pub(crate) bottom: i32,
}

impl Rect {
    pub(crate) const fn new(left: i32, top: i32, right: i32, bottom: i32) -> Self {
        Self { left, top, right, bottom }
    }
    pub(crate) fn width(self) -> i32 { self.right.saturating_sub(self.left) }
    pub(crate) fn height(self) -> i32 { self.bottom.saturating_sub(self.top) }
    fn valid(self) -> bool { self.width() > 0 && self.height() > 0 }
}

pub(crate) fn dock_rect(screen: Rect, minimum: i32, height: i32, gap: i32) -> Option<Rect> {
    if !screen.valid() || height <= 0 || gap < 0 || height.saturating_add(gap) > screen.height() {
        return None;
    }
    let width = (((screen.width() as i64 * 3) / 5) as i32).max(minimum).min(screen.width()).max(1);
    let x = screen.left + (screen.width() - width) / 2;
    let bottom = screen.bottom - gap;
    Some(Rect::new(x, bottom - height, x + width, bottom))
}

/// The attached screens apart from the primary one, without duplicates (disconnect, clone, mirroring).
/// The window creation itself uses `all_monitors`; this is the rule and it is covered by the unit tests.
#[allow(dead_code)]
pub(crate) fn secondary_monitors(screens: &[(Rect, bool)]) -> Vec<Rect> {
    let Some(&(primary, _)) = screens.iter().find(|(r, primary)| *primary && r.valid()) else { return Vec::new() };
    let mut result = Vec::new();
    for &(rect, is_primary) in screens {
        if !is_primary && rect != primary && rect.valid() && !result.contains(&rect) {
            result.push(rect);
        }
    }
    result
}

pub(crate) fn popup_origin(screen: Rect, x: i32, y: i32, width: i32, height: i32, margin: i32) -> (i32, i32) {
    fn axis(pos: i32, span: i32, start: i32, end: i32, margin: i32) -> i32 {
        let size = end.saturating_sub(start);
        let free = size.saturating_sub(span.max(0));
        if free <= 0 { return start; }
        let pad = margin.max(0).min(free / 2);
        pos.clamp(start + pad, end - span.max(0) - pad)
    }
    (axis(x, width, screen.left, screen.right, margin), axis(y, height, screen.top, screen.bottom, margin))
}

pub(crate) fn should_hide(fullscreen: bool, autohide: bool, idle: bool, engaged: bool) -> bool {
    fullscreen || (autohide && idle && !engaged)
}
