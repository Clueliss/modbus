use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("IO Error: {0}")]
    Io(#[from] tokio::io::Error),

    #[error("Modbus Error: {0}")]
    Modbus(#[from] ModbusError),

    #[error("unsable to deserialize server response {0}")]
    Deserialize(#[from] bincode::Error),
}

#[derive(Error, Debug)]
pub enum ModbusError {
    #[error("server responded with mismatched function code")]
    MismatchedFunctionCode,

    #[error("server responded with mismatched packet header")]
    MismatchedHeader,

    #[error("modbus exception: {0}")]
    Exception(ModbusException),

    #[error("invalid modbus exception code received: {0}")]
    InvalidException(u8),

    #[error("server did not execute write request")]
    FailedWrite,
}

#[derive(Error, Debug)]
pub enum ModbusException {
    #[error("illegal function")]
    IllegalFunction,
    #[error("illegal data address")]
    IllegalDataAddress,
    #[error("illegal data value")]
    IllegalDataValue,
    #[error("server encountered an unrecoverable error")]
    ServerDeviceFailure,
    #[error("acknowledged")]
    Acknowledge,
    #[error("server is processing long running command")]
    ServerDeviceBusy,
    #[error("memory parity error")]
    MemoryParityError,
    #[error("gateway was not able to allocate a communication path")]
    GatewayPathUnavailable,
    #[error("gateway did not receive response from target")]
    GatewayTargetDeviceFailedToRespond,
}

impl TryFrom<u8> for ModbusException {
    type Error = ();

    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        use ModbusException::*;

        match value {
            0x01 => Ok(IllegalFunction),
            0x02 => Ok(IllegalDataAddress),
            0x03 => Ok(IllegalDataValue),
            0x04 => Ok(ServerDeviceFailure),
            0x05 => Ok(Acknowledge),
            0x06 => Ok(ServerDeviceBusy),
            0x08 => Ok(MemoryParityError),
            0x0A => Ok(GatewayPathUnavailable),
            0x0B => Ok(GatewayTargetDeviceFailedToRespond),
            _ => Err(()),
        }
    }
}
