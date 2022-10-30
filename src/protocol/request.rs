use crate::{
    protocol::{function_codes, PacketHeader, BINCODE_OPTS},
    Address, Register,
};
use bincode::Options;
use serde::Serialize;

pub fn serialize_request<R>(
    header: PacketHeader,
    req: R,
) -> [u8; std::mem::size_of::<PacketHeader>() + std::mem::size_of::<R>()]
where
    R: Serialize,
    [(); std::mem::size_of::<PacketHeader>() + std::mem::size_of::<R>()]: Sized,
{
    let mut out = [0; std::mem::size_of::<PacketHeader>() + std::mem::size_of::<R>()];

    BINCODE_OPTS
        .serialize_into(&mut out[..std::mem::size_of::<PacketHeader>()], &header)
        .unwrap();

    BINCODE_OPTS
        .serialize_into(&mut out[std::mem::size_of::<PacketHeader>()..], &req)
        .unwrap();

    out
}

#[derive(Copy, Clone, Serialize)]
#[repr(C, packed)]
pub struct ReadRequest {
    pub function_code: u8,
    pub base_address: Address,
    pub len: u16,
}

impl ReadRequest {
    fn from_parts(function_code: u8, reg: Register) -> Self {
        Self { function_code, base_address: reg.start, len: reg.end - reg.start }
    }

    pub fn read_coils(reg: Register) -> Self {
        Self::from_parts(function_codes::READ_COILS, reg)
    }

    pub fn read_discrete_inputs(reg: Register) -> Self {
        Self::from_parts(function_codes::READ_DISCRETE_INPUTS, reg)
    }

    pub fn read_holding_registers(reg: Register) -> Self {
        Self::from_parts(function_codes::READ_HOLDING_REGISTERS, reg)
    }

    pub fn read_input_registers(reg: Register) -> Self {
        Self::from_parts(function_codes::READ_INPUT_REGISTERS, reg)
    }
}

#[derive(Copy, Clone, Serialize)]
#[repr(C, packed)]
pub struct WriteSingleRequest {
    pub function_code: u8,
    pub output_address: Address,
    pub output_value: u16,
}

impl WriteSingleRequest {
    pub fn write_single_coil(output_address: Address, output_value: bool) -> Self {
        let output_value = if output_value { 0xFF00 } else { 0x0000 };
        Self {
            function_code: function_codes::WRITE_SINGLE_COIL,
            output_address,
            output_value,
        }
    }

    pub fn write_single_register(output_address: Address, output_value: u16) -> Self {
        Self {
            function_code: function_codes::WRITE_SINGLE_REGISTER,
            output_address,
            output_value,
        }
    }
}
