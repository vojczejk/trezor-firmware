use super::super::constant::SCREEN;
use super::super::theme;
use crate::ui::display::Color;
use crate::ui::shape::{self, Renderer};

/// Custom shape for a full screen border overlay, parameterizable by color.
///
/// The native Eckhart border is a set of TOIF images cut for the 380x520 display.
/// On the 240x240 display we use a single rounded-rectangle TOIF generated for
/// this size instead.
pub struct ScreenBorder {
    color: Color,
}

impl ScreenBorder {
    pub const WIDTH: i16 = 5;
    pub const TOP_ARC_HEIGHT: i16 = Self::RADIUS;
    pub const RADIUS: i16 = 18;

    pub const fn new(color: Color) -> Self {
        Self { color }
    }

    pub const fn color(&self) -> Color {
        self.color
    }

    pub fn render<'s>(&'s self, alpha: u8, target: &mut impl Renderer<'s>) {
        shape::ToifImage::new(SCREEN.top_left(), theme::ICON_BORDER_240.toif)
            .with_fg(self.color)
            .with_alpha(alpha)
            .render(target);
    }
}
