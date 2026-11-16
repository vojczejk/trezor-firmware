//! PoC bridge for running the button-driven Caesar layout on a touch device.
//!
//! The bottom strip of the screen works as the two physical buttons that
//! Caesar expects. Touching the left/right half sends button press/release
//! events, so short taps, long holds and hold-to-confirm keep working.

use core::sync::atomic::{AtomicU8, Ordering};

use crate::ui::component::Event;
use crate::ui::constant;
use crate::ui::event::{ButtonEvent, PhysicalButton, TouchEvent};
use crate::ui::geometry::Point;

const NONE: u8 = 0;
const LEFT: u8 = 1;
const RIGHT: u8 = 2;

/// The visual Caesar button hints are tiny; use the bottom sixth of the
/// screen as the touch target, matching where the hints are drawn.
const STRIP_HEIGHT: i16 = constant::HEIGHT / 6;

static PRESSED: AtomicU8 = AtomicU8::new(NONE);

fn button_at(point: Point) -> Option<PhysicalButton> {
    if point.y < constant::HEIGHT - STRIP_HEIGHT {
        return None;
    }
    Some(if point.x < constant::WIDTH / 2 {
        PhysicalButton::Left
    } else {
        PhysicalButton::Right
    })
}

pub fn translate(event: TouchEvent) -> Event {
    match event {
        TouchEvent::TouchStart(point) => match button_at(point) {
            Some(button) => {
                PRESSED.store(
                    if button == PhysicalButton::Left {
                        LEFT
                    } else {
                        RIGHT
                    },
                    Ordering::Relaxed,
                );
                Event::Button(ButtonEvent::ButtonPressed(button))
            }
            None => Event::Touch(event),
        },
        TouchEvent::TouchEnd(_) => match PRESSED.swap(NONE, Ordering::Relaxed) {
            LEFT => Event::Button(ButtonEvent::ButtonReleased(PhysicalButton::Left)),
            RIGHT => Event::Button(ButtonEvent::ButtonReleased(PhysicalButton::Right)),
            _ => Event::Touch(event),
        },
        _ => Event::Touch(event),
    }
}
