use bytes::{BufMut, BytesMut};
use log::*;
use std::io;
use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::lookup_host;
use tokio::net::TcpStream;
use tokio::net::ToSocketAddrs;
use url::Url;

pub trait ToSocketAddrsExt: ToSocketAddrs {
    fn to_string2(&self) -> String;
}

impl ToSocketAddrsExt for SocketAddr {
    fn to_string2(&self) -> String {
        self.to_string()
    }
}

impl ToSocketAddrsExt for str {
    fn to_string2(&self) -> String {
        self.to_string()
    }
}

impl ToSocketAddrsExt for &str {
    fn to_string2(&self) -> String {
        self.to_string()
    }
}

impl ToSocketAddrsExt for String {
    fn to_string2(&self) -> String {
        self.to_string()
    }
}

impl ToSocketAddrsExt for (&'_ str, u16) {
    fn to_string2(&self) -> String {
        format!("{}:{}", self.0, self.1)
    }
}

impl ToSocketAddrsExt for (IpAddr, u16) {
    fn to_string2(&self) -> String {
        format!("{}:{}", self.0, self.1)
    }
}

const ADDR_TYPE_IPV4ADDR: u8 = 0x01;
const ADDR_TYPE_DOMAINNAME: u8 = 0x03;
const ADDR_TYPE_IPV6ADDR: u8 = 0x04;

fn put_ip(ip: &IpAddr, buf: &mut BytesMut) {
    match ip {
        IpAddr::V4(ip) => {
            buf.put_u8(ADDR_TYPE_IPV4ADDR);
            buf.put(&ip.octets()[..]);
        }
        IpAddr::V6(ip) => {
            buf.put_u8(ADDR_TYPE_IPV6ADDR);
            buf.put(&ip.octets()[..]);
        }
    };
}

pub async fn connect<A: ToSocketAddrsExt, T: AsRef<str>>(
    addr: A,
    proxy_url: T,
) -> io::Result<TcpStream> {
    let addr2 = addr.to_string2();
    let proxy_url = match Url::parse(proxy_url.as_ref()) {
        Ok(url) => url,
        Err(err) => {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!("invalid socks5 url: {:?}", err),
            ));
        }
    };

    let mut addrs = Vec::new();
    let max_tries = match proxy_url.scheme() {
        "socks5" => {
            addrs = lookup_host(addr).await?.collect();
            addrs.len()
        }
        "socks5h" => 1,
        _ => {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "invalid socks5 url: unsupported scheme",
            ));
        }
    };

    let mut buf = BytesMut::with_capacity(512);
    let proxy_addr = (
        proxy_url.host_str().ok_or(io::Error::new(
            io::ErrorKind::Other,
            "invalid socks5 url, no host",
        ))?,
        proxy_url.port().ok_or(io::Error::new(
            io::ErrorKind::Other,
            "invalid socks5 url, no port",
        ))?,
    );

    for addr_idx in 0..max_tries {
        trace!("connect socks5 server: {:?}", proxy_addr);
        let mut s = TcpStream::connect(&proxy_addr).await?;

        s.write_all(b"\x05\x01\x00").await?;
        buf.resize(2, 0);
        s.read_exact(&mut buf).await?;
        if buf[1] != 0 {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "unsupported authentication method",
            ));
        }

        buf.clear();
        buf.put(&b"\x05\x01\x00"[..]);
        match proxy_url.scheme() {
            "socks5" => {
                let addr = addrs[addr_idx];
                trace!("connect {}", addr);
                let ip = addr.ip();
                put_ip(&ip, &mut buf);
                buf.put(addr.port().to_be_bytes().as_ref());
            }
            "socks5h" => {
                let addr = addr2.clone();
                trace!("connect {}", addr);
                let idx = addr.rfind(':').unwrap();
                let host = String::from(&addr[..idx]);
                let port = addr[idx + 1..].parse::<u16>().unwrap();
                match IpAddr::from_str(&host) {
                    Ok(ip) => {
                        put_ip(&ip, &mut buf);
                    }
                    Err(_) => {
                        if host.len() > 255 {
                            return Err(io::Error::new(io::ErrorKind::Other, "FQDN too long"));
                        }
                        buf.put_u8(ADDR_TYPE_DOMAINNAME);
                        buf.put_u8(host.len() as u8);
                        buf.put(host.as_bytes());
                    }
                }
                buf.put(port.to_be_bytes().as_ref());
            }
            _ => {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    "invalid socks5 url: unsupported scheme",
                ));
            }
        }

        s.write_all(&buf[..]).await?;

        buf.resize(4, 0);
        let mut len = s.read_exact(&mut buf).await?;

        let reply = buf[1];
        if reply != 0 {
            error!("socks5 connect command failed: reply={}", reply);
            continue;
        }

        let addr_len = match buf[3] {
            ADDR_TYPE_IPV4ADDR => 4,
            ADDR_TYPE_IPV6ADDR => 16,
            ADDR_TYPE_DOMAINNAME => {
                buf.resize(5, 0);
                len += s.read_exact(&mut buf[4..]).await?;
                (buf[4] + 1) as usize
            }
            _ => {
                return Err(io::Error::new(io::ErrorKind::Other, "unknown address type"));
            }
        };
        let should_len = 4 + addr_len + 2;
        buf.resize(should_len, 0);
        s.read_exact(&mut buf[len..]).await?;

        return Ok(s);
    }

    Err(io::Error::new(io::ErrorKind::Other, "failed to connect"))
}
