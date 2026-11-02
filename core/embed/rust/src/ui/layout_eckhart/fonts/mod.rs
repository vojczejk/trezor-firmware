//! Font definitions for the Eckhart layout
//! Allowed only in Bootloader:
//!     - FONT_SATOSHI_EXTRALIGHT_72_NUMS_ONLY
//! Allowed in Bootloader and Firmware:
//!     - FONT_SATOSHI_MEDIUM_26
//!     - FONT_SATOSHI_REGULAR_38
//! Allowed only in Firmware:
//!     - FONT_MONO_LIGHT_30
//!     - FONT_MONO_MEDIUM_38
//!     - FONT_SATOSHI_EXTRALIGHT_46
//!     - FONT_SATOSHI_EXTRALIGHT_72
//!     - FONT_SATOSHI_REGULAR_22

#[cfg(any(feature = "prodtest", feature = "micropython"))]
mod font_robotomono_light_30;
mod font_robotomono_medium_21;
#[cfg(any(feature = "prodtest", feature = "micropython"))]
mod font_robotomono_medium_38;
mod font_ttsatoshi_demibold_18;
mod font_ttsatoshi_demibold_21;
mod font_ttsatoshi_demibold_42;
#[cfg(any(feature = "prodtest", feature = "micropython"))]
mod font_ttsatoshi_extralight_46;
mod font_ttsatoshi_extralight_72;
mod font_ttsatoshi_medium_26;
#[cfg(any(feature = "prodtest", feature = "micropython"))]
mod font_ttsatoshi_regular_22;
mod font_ttsatoshi_regular_38;

use font_robotomono_medium_21::Font_RobotoMono_Medium_21_info;
use font_ttsatoshi_demibold_18::Font_TTSatoshi_DemiBold_18_info;
use font_ttsatoshi_demibold_21::Font_TTSatoshi_DemiBold_21_info;
use font_ttsatoshi_demibold_42::Font_TTSatoshi_DemiBold_42_info;
#[cfg(feature = "bootloader")]
use font_ttsatoshi_extralight_72::Font_TTSatoshi_ExtraLight_72_NumsOnly_info;

// The 240x240 display of T3T1 needs the same typographic hierarchy at a
// smaller physical size. For the spike, the Eckhart roles are mapped onto the
// already-proven Delizia faces.
#[cfg(any(feature = "prodtest", feature = "micropython"))]
pub const FONT_MONO_LIGHT_30: crate::ui::display::Font = &Font_RobotoMono_Medium_21_info;
#[cfg(any(feature = "prodtest", feature = "micropython"))]
pub const FONT_MONO_MEDIUM_38: crate::ui::display::Font = &Font_RobotoMono_Medium_21_info;
#[cfg(any(feature = "prodtest", feature = "micropython"))]
pub const FONT_SATOSHI_EXTRALIGHT_46: crate::ui::display::Font = &Font_TTSatoshi_DemiBold_42_info;
#[cfg(any(feature = "prodtest", feature = "micropython"))]
pub const FONT_SATOSHI_EXTRALIGHT_72: crate::ui::display::Font = &Font_TTSatoshi_DemiBold_42_info;
#[cfg(feature = "bootloader")]
pub const FONT_SATOSHI_EXTRALIGHT_72_NUMS_ONLY: crate::ui::display::Font =
    &Font_TTSatoshi_ExtraLight_72_NumsOnly_info;
pub const FONT_SATOSHI_MEDIUM_26: crate::ui::display::Font = &Font_TTSatoshi_DemiBold_21_info;
#[cfg(any(feature = "prodtest", feature = "micropython"))]
pub const FONT_SATOSHI_REGULAR_22: crate::ui::display::Font = &Font_TTSatoshi_DemiBold_18_info;
pub const FONT_SATOSHI_REGULAR_38: crate::ui::display::Font = &Font_TTSatoshi_DemiBold_21_info;
