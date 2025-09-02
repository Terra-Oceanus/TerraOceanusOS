//! Globally Unique Identifier

use crate::io::text::Output;

#[repr(C, packed)]
#[derive(PartialEq, Eq)]
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
    pub const UNUSED_PARTITION: Self = Self::from_str("00000000-0000-0000-0000-000000000000");
    pub const LEGACY_MBR_PARTITION: Self = Self::from_str("024DEE41-33E7-11D3-9D69-0008C781F39F");
    pub const EFI_SYSTEM_PARTITION: Self = Self::from_str("C12A7328-F81F-11D2-BA4B-00A0C93EC93B");
    pub const WINDOWS_BASIC_DATA_PARTITION: Self =
        Self::from_str("EBD0A0A2-B9E5-4433-87C0-68B6B72699C7");

    const fn from_str(guid: &'static str) -> Self {
        const fn chars_to_byte(low: u8, high: u8) -> u8 {
            const fn char_to_nibble(c: u8) -> u8 {
                match c {
                    b'a'..=b'f' => c - b'a' + 10,
                    b'A'..=b'F' => c - b'A' + 10,
                    _ => c - b'0',
                }
            }
            (char_to_nibble(high) << 4) | char_to_nibble(low)
        }

        let bytes = guid.as_bytes();
        Self {
            time_low: ((chars_to_byte(bytes[1], bytes[0]) as u32) << 24)
                | ((chars_to_byte(bytes[3], bytes[2]) as u32) << 16)
                | ((chars_to_byte(bytes[5], bytes[4]) as u32) << 8)
                | (chars_to_byte(bytes[7], bytes[6]) as u32),
            time_mid: ((chars_to_byte(bytes[10], bytes[9]) as u16) << 8)
                | (chars_to_byte(bytes[12], bytes[11]) as u16),
            time_high_and_version: ((chars_to_byte(bytes[15], bytes[14]) as u16) << 8)
                | (chars_to_byte(bytes[17], bytes[16]) as u16),
            clock_seq_high_and_reserved: chars_to_byte(bytes[20], bytes[19]),
            clock_seq_low: chars_to_byte(bytes[22], bytes[21]),
            node: [
                chars_to_byte(bytes[25], bytes[24]),
                chars_to_byte(bytes[27], bytes[26]),
                chars_to_byte(bytes[29], bytes[28]),
                chars_to_byte(bytes[31], bytes[30]),
                chars_to_byte(bytes[33], bytes[32]),
                chars_to_byte(bytes[35], bytes[34]),
            ],
        }
    }
}
