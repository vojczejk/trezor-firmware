//! No-op fuel gauge used on models without a power manager.
//!
//! The Eckhart layout was written for T3W1, which has a battery and a power
//! manager. T3T1 has neither, so the visual layer must still compile against
//! the same API while never drawing a battery indicator.

use crate::ui::component::{Component, Event, EventCtx, Never};
use crate::ui::geometry::Rect;
use crate::ui::shape::Renderer;

pub struct FuelGauge;

impl FuelGauge {
    pub const fn always() -> Self {
        Self
    }

    pub const fn always_icon_only() -> Self {
        Self
    }

    pub const fn charging_icon_only() -> Self {
        Self
    }

    pub const fn on_charging_change() -> Self {
        Self
    }

    pub fn update_pm_state(&mut self) {}

    pub fn should_be_shown(&self) -> bool {
        false
    }

    pub fn content_width(&self) -> i16 {
        0
    }
}

impl Component for FuelGauge {
    type Msg = Never;

    fn place(&mut self, bounds: Rect) -> Rect {
        bounds
    }

    fn event(&mut self, _ctx: &mut EventCtx, _event: Event) -> Option<Self::Msg> {
        None
    }

    fn render<'s>(&'s self, _target: &mut impl Renderer<'s>) {}
}

#[cfg(feature = "ui_debug")]
impl crate::trace::Trace for FuelGauge {
    fn trace(&self, t: &mut dyn crate::trace::Tracer) {
        t.component("FuelGauge");
    }
}
