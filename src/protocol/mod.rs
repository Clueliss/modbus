pub mod request;
pub mod response;

use bincode::{
    config::{BigEndian, FixintEncoding, WithOtherEndian, WithOtherIntEncoding},
    DefaultOptions, Options,
};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::ops::Range;

pub type Address = u16;
pub type Register = Range<Address>;

pub mod function_codes {
    pub const READ_COILS: u8 = 0x01;
    pub const READ_DISCRETE_INPUTS: u8 = 0x02;
    pub const READ_HOLDING_REGISTERS: u8 = 0x03;
    pub const READ_INPUT_REGISTERS: u8 = 0x04;
    pub const WRITE_SINGLE_COIL: u8 = 0x05;
    pub const WRITE_SINGLE_REGISTER: u8 = 0x06;
    pub const WRITE_MULTIPLE_COILS: u8 = 0x0F;
    pub const WRITE_MULTIPLE_REGISTERS: u8 = 0x10;
    pub const READ_FILE_RECORD: u8 = 0x14;
    pub const WRITE_FILE_RECORD: u8 = 0x15;
    pub const MASK_WRITE_REGISTER: u8 = 0x16;
    pub const READ_WRITE_MULTIPLE_REGISTERS: u8 = 0x17;
    pub const READ_FIFO_QUEUE: u8 = 0x18;
    pub const ENCAPSULATED_INTERFACE_TRANSPORT: u8 = 0x2B;

    pub const ERROR_FUNCTION_CODE_OFFSET: u8 = 0x80;
}

type BincodeOptions = WithOtherIntEncoding<WithOtherEndian<DefaultOptions, BigEndian>, FixintEncoding>;

lazy_static! {
    pub static ref BINCODE_OPTS: BincodeOptions = bincode::options().with_big_endian().with_fixint_encoding();
}

#[derive(Copy, Clone, Serialize, Deserialize)]
#[repr(C, packed)]
pub struct PacketHeader {
    pub transaction_id: u16,
    pub protocol_identifier: u16,
    pub length: u16,
    pub unit_identifier: u8,
}

impl PacketHeader {
    pub fn header_eq(&self, other: &Self) -> bool {
        self.transaction_id == other.transaction_id
            && self.protocol_identifier == other.protocol_identifier
            && self.unit_identifier == other.unit_identifier
    }
}
