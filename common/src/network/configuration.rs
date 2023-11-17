use std::net::{IpAddr, Ipv4Addr, SocketAddr};

pub const LOCAL_ADDRESS: IpAddr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
pub const SERVER_ADDRESS: IpAddr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
pub const SERVER_TO_CLIENT_PORT: u16 = 0;
pub const CLIENT_TO_SERVER_PORT: u16 = 43735;

pub const SERVER_SOCKET_ADDRESS: SocketAddr = SocketAddr::new(LOCAL_ADDRESS, CLIENT_TO_SERVER_PORT);
pub const CLIENT_SOCKET_ADDRESS: SocketAddr = SocketAddr::new(LOCAL_ADDRESS, SERVER_TO_CLIENT_PORT);

pub const PROTOCOL_ID: u64 = crate::version_num() as u64;
