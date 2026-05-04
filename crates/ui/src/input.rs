/// karakter için kullanılır maske password girdi alanlar.
pub(super) const MASK_CHAR: char = '•';

mod blink_cursor;
mod change;
mod clear_button;
mod cursor;
mod display_map;
mod element;
mod indent;
mod input;
mod lsp;
mod mask_pattern;
mod mode;
mod movement;
mod number_input;
mod otp_input;
pub(crate) mod popovers;
mod rope_ext;
mod search;
mod selection;
mod state;

pub(crate) use clear_button::*;
pub use cursor::*;
#[cfg(target_family = "wasm")]
pub use display_map::folding::Agac;
pub use display_map::{BufferPoint, DisplayMap, DisplayPoint, FoldRange};
pub use indent::TabSize;
pub use input::*;
pub use lsp::*;
pub use lsp_types::Position;
pub use mask_pattern::MaskPattern;
pub use number_input::{NumberInput, NumberInputEvent, StepAction};
pub use otp_input::*;
pub use rope_ext::{InputEdit, Point, RopeExt, RopeLines};
pub use ropey::Rope;
pub use state::*;

pub type Girdi = input::Input;
pub type GirdiDurumu = state::InputState;
pub type GirdiOlayi = state::InputEvent;
pub type SekmeBoyutu = indent::TabSize;
pub type MaskeDeseni = mask_pattern::MaskPattern;
pub type SayiGirdisi = number_input::NumberInput;
pub type SayiGirdisiOlayi = number_input::NumberInputEvent;
pub type AdimAksiyonu = number_input::StepAction;
pub type GirdiDuzenlemesi = rope_ext::InputEdit;
pub type Halat = ropey::Rope;
pub type HalatSatirlari<'a> = rope_ext::RopeLines<'a>;
pub use rope_ext::RopeExt as HalatUzantisi;
