mod address;
mod error;
mod interface;
mod registry;

pub use address::{Address, IntoAddress};
pub use error::{Error, Result, NOT_IMPLEMENTED};
pub use interface::*;
pub use registry::Registry;
pub mod config {
    pub use serde_json::{self, from_value, Error, Value};
}

pub struct NoopNet;

#[async_trait]
impl INet for NoopNet {
    async fn tcp_connect(&self, _addr: Address) -> Result<TcpStream> {
        Err(NOT_IMPLEMENTED)
    }

    async fn tcp_bind(&self, _addr: Address) -> Result<TcpListener> {
        Err(NOT_IMPLEMENTED)
    }

    async fn udp_bind(&self, _addr: Address) -> Result<UdpSocket> {
        Err(NOT_IMPLEMENTED)
    }
}

pub struct CombineNet {
    pub tcp_connect: Net,
    pub tcp_bind: Net,
    pub udp_bind: Net,
}

#[async_trait]
impl INet for CombineNet {
    async fn tcp_connect(&self, addr: Address) -> Result<TcpStream> {
        self.tcp_connect.tcp_connect(addr).await
    }

    async fn tcp_bind(&self, addr: Address) -> Result<TcpListener> {
        self.tcp_bind.tcp_bind(addr).await
    }

    async fn udp_bind(&self, addr: Address) -> Result<UdpSocket> {
        self.udp_bind.udp_bind(addr).await
    }
}
