pub mod varlenvec;

use crate::{
    protocol::{function_codes, BINCODE_OPTS},
    result::{ModbusError, Result},
    Address,
};
use bincode::Options;

pub use varlenvec::VarLenVec;

fn parse_response(cursor: &mut &[u8], sent_function_code: u8) -> Result<()> {
    let function_code: u8 = BINCODE_OPTS.deserialize_from(cursor.clone())?;

    if function_code == sent_function_code {
        Ok(())
    } else if function_code == sent_function_code + function_codes::ERROR_FUNCTION_CODE_OFFSET {
        let exception_code: u8 = BINCODE_OPTS.deserialize_from(cursor)?;
        let exception_code = exception_code
            .try_into()
            .map_err(|_| ModbusError::InvalidException(exception_code))?;

        Err(ModbusError::Exception(exception_code).into())
    } else {
        Err(ModbusError::MismatchedFunctionCode.into())
    }
}

#[derive(Clone)]
pub struct ReadResponse {
    pub function_code: u8,
    pub data: Vec<u8>,
}

impl ReadResponse {
    pub fn parse(mut data: &[u8], sent_function_code: u8) -> Result<ReadResponse> {
        parse_response(&mut data, sent_function_code)?;

        let byte_count: u8 = BINCODE_OPTS.deserialize_from(&mut data)?;
        let data_len = byte_count as usize;
        let data = BINCODE_OPTS.deserialize_seed(varlenvec::VarLenVec::<u8>::new(data_len), &mut data)?;
        Ok(ReadResponse { function_code: sent_function_code, data })
    }
}

#[derive(Copy, Clone)]
pub struct WriteSingleResponse {
    pub function_code: u8,
    pub output_address: Address,
    pub output_value: u16,
}

impl WriteSingleResponse {
    pub fn parse(mut data: &[u8], sent_function_code: u8) -> Result<WriteSingleResponse> {
        parse_response(&mut data, sent_function_code)?;

        let output_address: u16 = BINCODE_OPTS.deserialize_from(&mut data)?;
        let output_value: u16 = BINCODE_OPTS.deserialize_from(&mut data)?;

        Ok(WriteSingleResponse { function_code: sent_function_code, output_address, output_value })
    }
}
