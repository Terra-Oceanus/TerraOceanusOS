//! Interrupt Controller Structure

pub mod type0;
pub mod type1;
pub mod type2;
pub mod type4;

use super::FromAddr;

#[repr(C, packed)]
pub struct Header {
    pub type_: u8,

    pub length: u8,
}

fn polarity_to_str(polarity: u8) -> &'static str {
    match polarity {
        0b00 => "Bus default",
        0b01 => "Active high",
        0b11 => "Active low",
        _ => "Reserved",
    }
}

fn trigger_mode_to_str(trigger_mode: u8) -> &'static str {
    match trigger_mode {
        0b00 => "Bus default",
        0b01 => "Edge-triggered",
        0b11 => "Level-triggered",
        _ => "Reserved",
    }
}
