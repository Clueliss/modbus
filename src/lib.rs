#![feature(generic_const_exprs)]

mod protocol;
pub mod result;

use bincode::Options;
use result::*;
use serde::Serialize;
use std::net::SocketAddr;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

pub use protocol::{Address, Register};
pub use result::{Error, Result};

pub struct Modbus {
    addr: SocketAddr,
    transaction_id: u16,
}

impl Modbus {
    async fn send_request<R>(&mut self, stream: &mut TcpStream, req: R) -> Result<Vec<u8>>
    where
        R: Serialize,
        [(); std::mem::size_of::<protocol::PacketHeader>() + std::mem::size_of::<R>()]: Sized,
    {
        let req_header = protocol::PacketHeader {
            transaction_id: self.transaction_id,
            protocol_identifier: 0,
            length: (std::mem::size_of::<protocol::request::ReadRequest>() + 1) as u16,
            unit_identifier: 0,
        };

        self.transaction_id = self.transaction_id.wrapping_add(1);

        stream
            .write_all(&protocol::request::serialize_request(req_header, req))
            .await?;

        let mut resp_header = [0; std::mem::size_of::<protocol::PacketHeader>()];
        stream.read_exact(&mut resp_header).await?;

        let resp_header: protocol::PacketHeader = protocol::BINCODE_OPTS.deserialize(&resp_header).unwrap();

        if resp_header.header_eq(&req_header) {
            let mut resp_body = vec![0; (resp_header.length - 1) as usize];
            stream.read_exact(&mut resp_body).await?;

            Ok(resp_body)
        } else {
            Err(ModbusError::MismatchedHeader.into())
        }
    }

    async fn read_registers(&mut self, req: protocol::request::ReadRequest) -> Result<Vec<u16>> {
        let mut stream = TcpStream::connect(self.addr).await?;
        let response_data = self.send_request(&mut stream, req).await?;

        let response = protocol::response::ReadResponse::parse(&response_data, req.function_code)?;
        let data = protocol::BINCODE_OPTS.deserialize_seed(
            protocol::response::VarLenVec::<u16>::new(response.data.len() / std::mem::size_of::<u16>()),
            &response.data,
        )?;

        Ok(data)
    }

    async fn read_bits(&mut self, req: protocol::request::ReadRequest) -> Result<Vec<bool>> {
        let mut stream = TcpStream::connect(self.addr).await?;
        let response_data = self.send_request(&mut stream, req).await?;

        let response = protocol::response::ReadResponse::parse(&response_data, req.function_code)?;
        let mut bits: Vec<_> = response
            .data
            .into_iter()
            .flat_map(|byte| (0..7).into_iter().map(move |shift| (byte >> shift) & 1 > 0))
            .collect();

        let remain = (req.len % 8) as usize;
        if remain > 0 {
            bits.truncate(bits.len() - (8 - remain));
        }

        Ok(bits)
    }

    async fn write_single(&mut self, req: protocol::request::WriteSingleRequest) -> Result<()> {
        let mut stream = TcpStream::connect(self.addr).await?;
        let response_data = self.send_request(&mut stream, req).await?;

        let response = protocol::response::WriteSingleResponse::parse(&response_data, req.function_code)?;
        if response.output_address == req.output_address && response.output_value == req.output_value {
            Ok(())
        } else {
            Err(ModbusError::FailedWrite.into())
        }
    }
}

impl Modbus {
    pub fn new<A: Into<SocketAddr>>(addr: A) -> Self {
        Self { addr: addr.into(), transaction_id: 0 }
    }

    pub async fn read_input_registers(&mut self, reg: Register) -> Result<Vec<u16>> {
        let req = protocol::request::ReadRequest::read_input_registers(reg);
        self.read_registers(req).await
    }

    pub async fn read_holding_registers(&mut self, reg: Register) -> Result<Vec<u16>> {
        let req = protocol::request::ReadRequest::read_holding_registers(reg);
        self.read_registers(req).await
    }

    pub async fn read_coils(&mut self, reg: Register) -> Result<Vec<bool>> {
        let req = protocol::request::ReadRequest::read_coils(reg);
        self.read_bits(req).await
    }

    pub async fn read_discrete_inputs(&mut self, reg: Register) -> Result<Vec<bool>> {
        let req = protocol::request::ReadRequest::read_discrete_inputs(reg);
        self.read_bits(req).await
    }

    pub async fn write_single_coil(&mut self, addr: Address, value: bool) -> Result<()> {
        let req = protocol::request::WriteSingleRequest::write_single_coil(addr, value);
        self.write_single(req).await
    }

    pub async fn write_single_register(&mut self, addr: Address, value: u16) -> Result<()> {
        let req = protocol::request::WriteSingleRequest::write_single_register(addr, value);
        self.write_single(req).await
    }
}

#[cfg(test)]
mod tests {
    use crate::Modbus;
    use std::net::IpAddr;

    #[tokio::test]
    async fn test() {
        let mut modbus = Modbus::new((IpAddr::from([192, 168, 0, 52]), 502));

        let reg = 1065..1072;
        dbg!(modbus.read_input_registers(reg.clone()).await);
        dbg!(modbus.read_input_registers(reg.clone()).await);

        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

        dbg!(modbus.read_input_registers(reg).await);
    }
}
