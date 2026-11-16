use super::geometry::Rect;
#[cfg(any(feature = "ui_debug_overlay", feature = "ui_performance_overlay"))]
use super::shape;
use super::CommonUI;
#[cfg(feature = "ui_performance_overlay")]
use super::PerformanceOverlay;
#[cfg(feature = "ui_debug_overlay")]
use super::{display::Color, geometry::Offset};

#[cfg(feature = "bootloader")]
pub mod bootloader;

#[cfg(feature = "prodtest")]
pub mod prodtest;

pub mod common_messages;
pub mod component;
#[cfg(feature = "micropython")]
pub mod component_msg_obj;
pub mod constant;
pub mod cshape;
pub mod fonts;
pub mod theme;
#[cfg(all(feature = "touch", feature = "button"))]
pub mod touch;

use component::{ErrorScreen, WelcomeScreen};

use crate::ui::layout::simplified::show;

pub struct UICaesar {}

#[cfg(feature = "micropython")]
pub mod ui_firmware;

impl CommonUI for UICaesar {
    #[cfg(feature = "backlight")]
    fn fadein() {
        crate::ui::display::fade_backlight_duration(theme::backlight::get_backlight_normal(), 150);
    }

    #[cfg(feature = "backlight")]
    fn fadeout() {
        crate::ui::display::fade_backlight_duration(theme::backlight::get_backlight_dim(), 150);
    }

    #[cfg(feature = "backlight")]
    fn backlight_on() {
        crate::ui::display::set_backlight(theme::backlight::get_backlight_normal());
    }

    #[cfg(feature = "backlight")]
    fn get_backlight_none() -> u8 {
        theme::backlight::get_backlight_none()
    }

    #[cfg(feature = "backlight")]
    fn get_backlight_normal() -> u8 {
        theme::backlight::get_backlight_normal()
    }

    #[cfg(feature = "backlight")]
    fn get_backlight_low() -> u8 {
        theme::backlight::get_backlight_low()
    }

    #[cfg(feature = "backlight")]
    fn get_backlight_dim() -> u8 {
        theme::backlight::get_backlight_dim()
    }

    #[cfg(feature = "backlight")]
    fn get_backlight_max() -> u8 {
        theme::backlight::get_backlight_max()
    }

    const SCREEN: Rect = constant::SCREEN;

    fn screen_fatal_error(title: &str, msg: &str, footer: &str) {
        let mut frame = ErrorScreen::new(title.into(), msg.into(), footer.into());
        show(&mut frame, false);
    }

    fn screen_boot_stage_2(fade_in: bool) {
        let mut frame = WelcomeScreen::new(false);
        show(&mut frame, fade_in);
    }

    fn screen_update() {}

    #[cfg(feature = "ui_debug_overlay")]
    fn render_debug_overlay<'s>(target: &mut impl shape::Renderer<'s>) {
        const RECT_SIZE: i16 = constant::SCREEN.width() / 30;
        let r = Rect::from_top_left_and_size(
            Self::SCREEN.top_right() - Offset::x(RECT_SIZE),
            Offset::new(RECT_SIZE, RECT_SIZE),
        );
        shape::Bar::new(r)
            .with_bg(Color::rgb(0xff, 0, 0))
            .render(target);
    }

    #[cfg(feature = "ui_performance_overlay")]
    fn render_performance_overlay<'s>(
        _target: &mut impl shape::Renderer<'s>,
        _info: PerformanceOverlay,
    ) {
        // Not implemented
    }
}
