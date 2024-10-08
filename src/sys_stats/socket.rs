use std::fmt::Display;
use tokio::{
    fs::File,
    io::{AsyncBufReadExt, BufReader},
};

use crate::Measurements;

const SOCK_MEAS_PATH: &str = "/proc/net/sockstat";

#[derive(Default, Clone)]
pub struct SocketStatMeasurements {
    tcp_inuse: usize,
    udp_inuse: usize,
    udp_lite_inuse: usize,
}

impl SocketStatMeasurements {
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

impl Display for SocketStatMeasurements {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "TCP: \t\t{} \nUDP: \t\t{} \nUPD lite: \t{}",
            self.tcp_inuse, self.udp_inuse, self.udp_lite_inuse
        )
    }
}

impl Measurements for SocketStatMeasurements {
    fn print_info(&self) {
        println!("{}", self);
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
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
pub async fn net_socket_read() -> Result<SocketStatMeasurements, Box<dyn std::error::Error>> {
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

    let sock_stat = SocketStatMeasurements::new(tcp_inuse, udp_inuse, udp_lite_inuse);
    Ok(sock_stat)
}

fn extract_socket_stat(line: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let sock_stat_elemts = line
        .split(" ")
        .collect::<Vec<&str>>()
        .iter()
        .filter(|s| **s != "")
        .map(|s| s.to_string().clone())
        .collect::<Vec<String>>();
    Ok(sock_stat_elemts[2].parse::<usize>()?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_socket_stat() {
        let line = "TCP: inuse   8 orphan 0 tw 0   alloc 8 mem 4";
        let result = extract_socket_stat(line);

        assert!(result.is_ok());
        let socket_stat = result.unwrap();

        assert_eq!(socket_stat, 8);
    }
}
