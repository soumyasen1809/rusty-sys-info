use std::fmt::Display;

use tokio::{
    fs::File,
    io::{AsyncBufReadExt, BufReader},
};

const SOCK_MEAS_PATH: &str = "/proc/net/sockstat";

pub struct SockStat {
    tcp_inuse: usize,
    udp_inuse: usize,
    udp_lite_inuse: usize,
}

impl SockStat {
    pub fn new(tcp_inuse: usize, udp_inuse: usize, udp_lite_inuse: usize) -> Self {
        Self {
            tcp_inuse,
            udp_inuse,
            udp_lite_inuse,
        }
    }

    pub fn tcp_inuse(&self) -> usize {
        self.tcp_inuse
    }

    pub fn udp_inuse(&self) -> usize {
        self.udp_inuse
    }

    pub fn udp_lite_inuse(&self) -> usize {
        self.udp_lite_inuse
    }
}

impl Display for SockStat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "TCP: \t\t{} \nUDP: \t\t{} \nUPD lite: \t{}",
            self.tcp_inuse, self.udp_inuse, self.udp_lite_inuse
        )
    }
}

/// The `/proc/net/sockstat6` file in Linux provides statistics about IPv6 sockets.
/// This file is part of the proc filesystem, which is a pseudo-filesystem used to
/// access kernel information.
///
/// TCP: Shows the number of TCP sockets in use.
/// UDP: Displays the number of UDP sockets in use.
/// UDPLITE: Indicates the number of UDP-Lite sockets in use.
/// RAW: Lists the number of raw sockets in use.
/// FRAG: Provides information about the number of IP fragments currently in use.
pub async fn net_socket_read() -> Result<SockStat, Box<dyn std::error::Error>> {
    let net_sock_file = File::open(SOCK_MEAS_PATH).await?;
    let net_sock_contents = BufReader::new(net_sock_file);
    let mut line = net_sock_contents.lines();

    let mut tcp_inuse: usize = usize::default();
    let mut udp_inuse: usize = usize::default();
    let mut udp_lite_inuse: usize = usize::default();

    while let Some(l) = line.next_line().await? {
        if l.contains("TCP: inuse") {
            tcp_inuse = extract_socket_stat(&l)?;
        }
        if l.contains("UDP: inuse") {
            udp_inuse = extract_socket_stat(&l)?;
        }
        if l.contains("UDPLITE: inuse") {
            udp_lite_inuse = extract_socket_stat(&l)?;
        }
    }

    let sock_stat = SockStat::new(tcp_inuse, udp_inuse, udp_lite_inuse);
    Ok(sock_stat)
}

fn extract_socket_stat(line: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let sock_stat_elemts = line.split(" ").collect::<Vec<&str>>();
    Ok(sock_stat_elemts[2].parse::<usize>()?)
}
