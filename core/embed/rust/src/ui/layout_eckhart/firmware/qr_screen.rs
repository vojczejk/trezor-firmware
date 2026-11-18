use super::super::constant::SCREEN;
use super::super::firmware::{theme, Header, HeaderMsg};
use crate::strutil::TString;
use crate::ui::component::swipe_detect::SwipeConfig;
use crate::ui::component::{Component, Event, EventCtx, Qr};
use crate::ui::flow::Swipable;
use crate::ui::geometry::{Offset, Rect};
use crate::ui::shape::Renderer;
use crate::ui::util::Pager;

pub enum QrMsg {
    Cancelled,
}

pub struct QrScreen {
    header: Header,
    qr: Qr,
}

impl QrScreen {
    pub fn new(title: TString<'static>, qr: Qr) -> Self {
        Self {
            header: Header::new(title).with_close_button(),
            qr,
        }
    }
}

impl Component for QrScreen {
    type Msg = QrMsg;

    fn place(&mut self, bounds: Rect) -> Rect {
        // assert full screen
        debug_assert_eq!(bounds.height(), SCREEN.height());
        debug_assert_eq!(bounds.width(), SCREEN.width());

        let (header_area, mut qr_area) = bounds.split_top(Header::HEADER_HEIGHT);
        qr_area = qr_area.inset(theme::SIDE_INSETS);
        // Fit the QR code into the remaining area instead of forcing it to the
        // width, which overflows on a square screen.
        let size = qr_area.width().min(qr_area.height());
        qr_area = Rect::from_center_and_size(qr_area.center(), Offset::new(size, size));

        self.header.place(header_area);
        self.qr.place(qr_area);

        bounds
    }

    fn event(&mut self, ctx: &mut EventCtx, event: Event) -> Option<Self::Msg> {
        if let Some(HeaderMsg::Cancelled) = self.header.event(ctx, event) {
            return Some(QrMsg::Cancelled);
        }

        None
    }

    fn render<'s>(&'s self, target: &mut impl Renderer<'s>) {
        self.header.render(target);
        self.qr.render(target);
    }
}

#[cfg(feature = "micropython")]
impl Swipable for QrScreen {
    fn get_swipe_config(&self) -> SwipeConfig {
        SwipeConfig::new()
    }

    fn get_pager(&self) -> Pager {
        Pager::single_page()
    }
}

#[cfg(feature = "ui_debug")]
impl crate::trace::Trace for QrScreen {
    fn trace(&self, t: &mut dyn crate::trace::Tracer) {
        t.component("QrScreen");
    }
}
