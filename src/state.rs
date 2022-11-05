use crate::{brightness::Brightness, temperature::Temperature};

pub struct GlowState {
    pub on: Option<bool>,
    pub brightness: Option<Brightness>,
    pub temperature: Option<Temperature>,
}

impl GlowState {
    pub fn write_to_hiddev() {

    }
}
