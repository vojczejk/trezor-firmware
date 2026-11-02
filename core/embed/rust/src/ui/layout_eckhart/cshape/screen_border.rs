use super::super::constant::SCREEN;
use crate::ui::display::Color;
use crate::ui::geometry::Insets;
use crate::ui::shape::{self, Renderer};

/// Custom shape for a full screen border overlay, parameterizable by color.
///
/// The native Eckhart border is a set of TOIF images cut for the 380x520 display.
/// On the 240x240 display we draw a plain rounded rectangle instead.
pub struct ScreenBorder {
    color: Color,
}

impl ScreenBorder {
    pub const WIDTH: i16 = 4;
    pub const TOP_ARC_HEIGHT: i16 = Self::WIDTH;
    pub const RADIUS: i16 = 10;

    pub const fn new(color: Color) -> Self {
        Self { color }
    }

    pub fn render<'s>(&'s self, alpha: u8, target: &mut impl Renderer<'s>) {
        // Inset by half the stroke width so the whole outline stays on screen.
        let area = SCREEN.inset(Insets::uniform(Self::WIDTH / 2));
        // `draw_round_rect` is a 1px outline, so draw concentric outlines to
        // get a border of the requested thickness.
        for i in 0..Self::WIDTH {
            shape::Bar::new(area.inset(Insets::uniform(i)))
                .with_fg(self.color)
                .with_thickness(1)
                .with_radius((Self::RADIUS - i).max(0))
                .with_alpha(alpha)
                .render(target);
        }
    }
}
