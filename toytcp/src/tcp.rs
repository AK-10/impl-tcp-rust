use crate::packet::TCPPacket;
use crate::socket::{SockID, Socket, TcpStatus};
use crate::tcpflags;
use anyhow::{Context, Result};
use pnet::packet::{ip::IpNextHeaderProtocols, tcp::TcpPacket, Packet};
use pnet::transport::{self, TransportChannelType};
use rand::{rngs::ThreadRng, Rng};
use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr};
use std::process::Command;
use std::sync::{Arc, Condvar, Mutex, RwLock, RwLockWriteGuard};
use std::time::{Duration, SystemTime};
use std::{cmp, ops::Range, str, thread};

const UNDETERMINED_IP_ADDR: std::net::Ipv4Addr = Ipv4Addr::new(0, 0, 0, 0);
const UNDETERMINED_PORT: u16 = 0;
const MAX_TRANSMITTION: u8 = 5;
const RETRANSMITTION_TIMEOUT: u64 = 3;
const MSS: usize = 1460;
const PORT_RANGE: Range<u16> = 40000..60000;

pub struct TCP {
    sockets: HashMap<SockID, Socket>,
}

impl TCP {
    pub fn new() -> Self {
        let sockets = HashMap::new();
        let tcp = Self { sockets };

        tcp
    }

    fn select_unused_port(&self, rng: &mut ThreadRng) -> Result<u16> {
        Ok(33445)
    }

    pub fn connect(&self, addr: Ipv4Addr, port: u16) -> Result<SockID> {
        let mut rng = rand::thread_rng();
        let mut socket = Socket::new(
            get_source_addr_to(addr)?,
            addr,
            self.select_unused_port(&mut rng)?,
            port,
        )?;

        // tsharkでのpacketの確認
        // TCP SYNセグメントが33445 -> 40000に対して送信されていることがわかる
        // TCPヘッダの設定が不足しているため、不正なセグメントとみなされている
        // (bogus TCP header lengthと表示されている)
        // そのためnetcatから返信がない
        // ```
        // 6 51.413941268     10.0.0.1 → 10.0.1.1     TCP 54 33445 → 40000 [SYN] Seq=0 Win=0, bogus TCP header length (0, must be at least 20)
        // ```
        socket.send_tcp_packet(tcpflags::SYN, &[])?;

        let sock_id = socket.get_sock_id();
        Ok(sock_id)
    }
}

fn get_source_addr_to(addr: Ipv4Addr) -> Result<Ipv4Addr> {
    Ok("10.0.0.1".parse().unwrap())
}
