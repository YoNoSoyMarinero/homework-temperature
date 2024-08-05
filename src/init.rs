use hal::{
    clocks::Clocks,
    iwdg,
};

use crate::setup;

pub fn run(){
    let clock_cfg = Clocks::default();
    clock_cfg.setup().unwrap();
    setup::setup_pins();
    iwdg::setup(0.1);
}
