use socks5_protocol::{Address, Error};
use std::io::{self, ErrorKind, Result};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub fn map_err(e: Error) -> rd_interface::Error {
    match e {
        Error::Io(io) => rd_interface::Error::IO(io),
        e => rd_interface::Error::Other(e.into()),
    }
}

pub async fn parse_udp(buf: &[u8]) -> Result<(Address, &[u8])> {
    let mut cursor = std::io::Cursor::new(buf);
    let mut header = [0u8; 3];
    cursor.read_exact(&mut header).await?;
    let addr = match header[0..3] {
        // TODO: support fragment sequence or at least give another error
        [0x00, 0x00, 0x00] => Address::read(&mut cursor).await.map_err(map_err)?,
        _ => {
            return Err(io::Error::new(
                ErrorKind::InvalidData,
                format!(
                    "server response wrong RSV {} RSV {} FRAG {}",
                    header[0], header[1], header[2]
                ),
            )
            .into())
        }
    };

    let pos = cursor.position() as usize;

    Ok((addr, &cursor.into_inner()[pos..]))
}

pub async fn pack_udp(addr: Address, buf: &[u8]) -> Result<Vec<u8>> {
    let addr: Address = addr.into();
    let mut cursor = std::io::Cursor::new(Vec::new());
    cursor.write_all(&[0x00, 0x00, 0x00]).await?;
    addr.write(&mut cursor).await.map_err(map_err)?;
    cursor.write_all(buf).await?;

    let bytes = cursor.into_inner();

    Ok(bytes)
}

pub fn sa2ra(addr: socks5_protocol::Address) -> rd_interface::Address {
    match addr {
        socks5_protocol::Address::Domain(d, p) => rd_interface::Address::Domain(d, p),
        socks5_protocol::Address::SocketAddr(s) => rd_interface::Address::SocketAddr(s),
    }
}
pub fn ra2sa(addr: rd_interface::Address) -> socks5_protocol::Address {
    match addr {
        rd_interface::Address::Domain(d, p) => socks5_protocol::Address::Domain(d, p),
        rd_interface::Address::SocketAddr(s) => socks5_protocol::Address::SocketAddr(s),
    }
}
