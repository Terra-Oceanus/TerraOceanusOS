//! Globally Unique Identifier

use crate::io::text::Output;

#[repr(C, packed)]
pub struct GUID {
    time_low: u32,
    time_mid: u16,
    time_high_and_version: u16,
    clock_seq_high_and_reserved: u8,
    clock_seq_low: u8,
    node: [u8; 6],
}
impl Output for GUID {
    fn out(&self) {
        Self::byte_to_hex_str((self.time_low >> 24) as u8);
        Self::byte_to_hex_str((self.time_low >> 16) as u8);
        Self::byte_to_hex_str((self.time_low >> 8) as u8);
        Self::byte_to_hex_str(self.time_low as u8);
        '-'.out();
        Self::byte_to_hex_str((self.time_mid >> 8) as u8);
        Self::byte_to_hex_str(self.time_mid as u8);
        '-'.out();
        Self::byte_to_hex_str((self.time_high_and_version >> 8) as u8);
        Self::byte_to_hex_str(self.time_high_and_version as u8);
        '-'.out();
        Self::byte_to_hex_str(self.clock_seq_high_and_reserved);
        Self::byte_to_hex_str(self.clock_seq_low);
        '-'.out();
        Self::byte_to_hex_str(self.node[0]);
        Self::byte_to_hex_str(self.node[1]);
        Self::byte_to_hex_str(self.node[2]);
        Self::byte_to_hex_str(self.node[3]);
        Self::byte_to_hex_str(self.node[4]);
        Self::byte_to_hex_str(self.node[5]);
    }
}
impl GUID {
    pub fn is_null(&self) -> bool {
        self.time_low == 0
            && self.time_mid == 0
            && self.time_high_and_version == 0
            && self.clock_seq_high_and_reserved == 0
            && self.clock_seq_low == 0
            && self.node == [0; 6]
    }
}
