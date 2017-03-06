pub use self::combined::*;
pub use self::direct::*;

use std::net::{Ipv4Addr,Ipv6Addr};

mod combined;
mod direct;
mod from_str;

#[cfg(test)]
mod tests;

/// `Cidr` type representing an IPv4 network
#[derive(Clone,PartialEq,Eq,Hash)]
pub struct Ipv4Cidr {
	address: Ipv4Addr,
	network_length: u8,
}

/// `Cidr` type representing an IPv6 network
#[derive(Clone,PartialEq,Eq,Hash)]
pub struct Ipv6Cidr {
	address: Ipv6Addr,
	network_length: u8,
}

/// `Cidr` type representing either an IPv4 or an IPv6 network
#[derive(Clone,PartialEq,Eq,Hash,Debug)]
pub enum IpCidr {
	/// IPv4 network
	V4(Ipv4Cidr),
	/// IPv6 network
	V6(Ipv6Cidr),
}
