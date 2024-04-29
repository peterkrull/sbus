#![no_std]

use core::array::from_fn;

use reader::Reader;

mod reader;

// Important bytes for correctnes checks
const FLAG_MASK: u8 = 0b11110000;
const HEAD_BYTE: u8 = 0b00001111;
const FOOT_BYTE: u8 = 0b00000000;

// Number of bytes in SBUS message
const PACKET_SIZE: usize = 25;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
pub struct SBusPacket {
    pub channels: [u16; 16],
    pub d1: bool,
    pub d2: bool,
    pub failsafe: bool,
    pub frame_lost: bool,
}

enum State {
    SeekHeader,
    Reading,
}

pub struct SBusPacketParser {
    state: State,
    buf: [u8; PACKET_SIZE],
    len: usize,
}

impl SBusPacketParser {
    pub fn new() -> SBusPacketParser {
        SBusPacketParser {
            state: State::SeekHeader,
            buf: [0; PACKET_SIZE],
            len: 0,
        }
    }

    /// Push array of `u8` bytes into buffer.
    pub fn push_bytes<'b>(&mut self, bytes: &'b [u8]) -> (Option<SBusPacket>, &'b [u8]) {
        let mut reader = Reader::new(bytes);
        let result = 'state_machine: loop {
            match self.state {
                State::SeekHeader => {
                    while let Some(byte) = reader.next() {
                        if byte == HEAD_BYTE {
                            self.buf[0] = byte;
                            self.len = 1;
                            self.state = State::Reading;
                            continue 'state_machine;
                        }
                    }

                    if reader.is_empty() {
                        break None;
                    }
                }
                State::Reading => {
                    let take = reader.next_n(PACKET_SIZE - self.len);
                    let n = take.len();
                    self.buf[self.len..self.len + n].copy_from_slice(take);
                    self.len += n;
                    if self.len == PACKET_SIZE {
                        self.state = State::SeekHeader;
                        break self.try_parse();
                    }
                }
            }

            break None;
        };

        (result, reader.remaining())
    }

    /// Attempts to parse a valid SBUS packet from the buffer
    pub fn try_parse(&mut self) -> Option<SBusPacket> {
        // Check if entire frame is valid
        if !self.valid_frame() {
            return None;
        }

        // Convert buffer to u16 array to allow for bit shifting
        let data: [u16; 24] = from_fn(|i| self.buf[i] as u16);

        // Initialize channels with 11-bit mask
        let mut ch: [u16; 16] = [0x07FF; 16];

        // Trust me bro
        ch[0] &= data[1] | data[2] << 8;
        ch[1] &= data[2] >> 3 | data[3] << 5;
        ch[2] &= data[3] >> 6 | data[4] << 2 | data[5] << 10;
        ch[3] &= data[5] >> 1 | data[6] << 7;
        ch[4] &= data[6] >> 4 | data[7] << 4;
        ch[5] &= data[7] >> 7 | data[8] << 1 | data[9] << 9;
        ch[6] &= data[9] >> 2 | data[10] << 6;
        ch[7] &= data[10] >> 5 | data[11] << 3;

        ch[8] &= data[12] | data[13] << 8;
        ch[9] &= data[13] >> 3 | data[14] << 5;
        ch[10] &= data[14] >> 6 | data[15] << 2 | data[16] << 10;
        ch[11] &= data[16] >> 1 | data[17] << 7;
        ch[12] &= data[17] >> 4 | data[18] << 4;
        ch[13] &= data[18] >> 7 | data[19] << 1 | data[20] << 9;
        ch[14] &= data[20] >> 2 | data[21] << 6;
        ch[15] &= data[21] >> 5 | data[22] << 3;

        let flag_byte = data[23] as u8;

        return Some(SBusPacket {
            channels: ch,
            d1: is_flag_set(flag_byte, 0),
            d2: is_flag_set(flag_byte, 1),
            frame_lost: is_flag_set(flag_byte, 2),
            failsafe: is_flag_set(flag_byte, 3),
        });
    }

    pub fn iter_packets<'a, 'b>(&'a mut self, buf: &'b [u8]) -> IterPackets<'a, 'b> {
        IterPackets { parser: self, buf }
    }

    /// Returns `true` if the first part of the buffer contains a valid SBUS frame
    fn valid_frame(&self) -> bool {
        self.len == PACKET_SIZE
            && self.buf[0] == HEAD_BYTE
            && self.buf[24] == FOOT_BYTE
            && self.buf[23] & FLAG_MASK == 0
    }
}

#[inline(always)]
fn is_flag_set(flag_byte: u8, shift_by: u8) -> bool {
    (flag_byte >> shift_by) & 1 == 1
}

pub struct IterPackets<'a, 'b> {
    parser: &'a mut SBusPacketParser,
    buf: &'b [u8],
}

impl<'a, 'b> Iterator for IterPackets<'a, 'b> {
    type Item = SBusPacket;

    fn next(&mut self) -> Option<Self::Item> {
        let packet;
        (packet, self.buf) = self.parser.push_bytes(self.buf);
        packet
    }
}
