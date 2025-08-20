// SPDX-License-Identifier: MIT

use netlink_packet_utils::{
	nla::{DefaultNla, Nla, NlaBuffer},
	DecodeError, Parseable,
};
use core::fmt;
use crate::conntrack::conntrack_proto_nat::ConntrackProtoNatNla;

// enum ctattr_nat {
// 	CTA_NAT_UNSPEC,
// 	CTA_NAT_V4_MINIP,
// 	#define CTA_NAT_MINIP CTA_NAT_V4_MINIP
// 	CTA_NAT_V4_MAXIP,
// 	#define CTA_NAT_MAXIP CTA_NAT_V4_MAXIP
// 	CTA_NAT_PROTO,
// 	CTA_NAT_V6_MINIP,
// 	CTA_NAT_V6_MAXIP,
// 	__CTA_NAT_MAX
// };

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ConntrackNatNla {
	// removed as just for not set/invalid
	// Unspec,
	V4MinIp([u8; 4]),
	V4MaxIp([u8; 4]),
	// Nested protonat attributes (port range)
	Proto(Vec<ConntrackProtoNatNla>),
	V6MinIp([u8; 16]),
	V6MaxIp([u8; 16]),
	Other(DefaultNla),
}

impl ConntrackNatNla {
	fn is_v4(&self) -> bool {
		matches!(self, Self::V4MinIp(_) | Self::V4MaxIp(_))
	}
	fn is_v6(&self) -> bool {
		matches!(self, Self::V6MinIp(_) | Self::V6MaxIp(_))
	}
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConntrackNatAttrs {
	inner: Vec<ConntrackNatNla>,
}

impl ConntrackNatAttrs {
	pub fn new(inner: Vec<ConntrackNatNla>) -> Result<Self, ConntrackNatError> {
		validate_nat_family(&inner)?;
		Ok(Self { inner })
	}
	pub fn as_slice(&self) -> &[ConntrackNatNla] { &self.inner }
	pub fn into_vec(self) -> Vec<ConntrackNatNla> { self.inner }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConntrackNatError {
	MixedFamilies,
	IncompleteV4Pair,
	IncompleteV6Pair,
}

impl fmt::Display for ConntrackNatError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			ConntrackNatError::MixedFamilies => write!(f, "mixed IPv4 and IPv6 NAT attributes in the same nest"),
			ConntrackNatError::IncompleteV4Pair => write!(f, "only one of IPv4 min/max present; both CTA_NAT_V4_MINIP and CTA_NAT_V4_MAXIP required"),
			ConntrackNatError::IncompleteV6Pair => write!(f, "only one of IPv6 min/max present; both CTA_NAT_V6_MINIP and CTA_NAT_V6_MAXIP required"),
		}
	}
}

impl std::error::Error for ConntrackNatError {}

fn validate_nat_family(nlas: &[ConntrackNatNla]) -> Result<(), ConntrackNatError> {
	let has_v4_min = nlas.iter().any(|n| matches!(n, ConntrackNatNla::V4MinIp(_)));
	let has_v4_max = nlas.iter().any(|n| matches!(n, ConntrackNatNla::V4MaxIp(_)));
	let has_v6_min = nlas.iter().any(|n| matches!(n, ConntrackNatNla::V6MinIp(_)));
	let has_v6_max = nlas.iter().any(|n| matches!(n, ConntrackNatNla::V6MaxIp(_)));

	let any_v4 = has_v4_min || has_v4_max;
	let any_v6 = has_v6_min || has_v6_max;
	if any_v4 && any_v6 { return Err(ConntrackNatError::MixedFamilies); }
	if any_v4 && !(has_v4_min && has_v4_max) { return Err(ConntrackNatError::IncompleteV4Pair); }
	if any_v6 && !(has_v6_min && has_v6_max) { return Err(ConntrackNatError::IncompleteV6Pair); }
	Ok(())
}

impl Nla for ConntrackNatNla {
	fn value_len(&self) -> usize { todo!("conntrack nat nla value_len") }
	fn kind(&self) -> u16 { todo!("conntrack nat nla kind") }
	fn emit_value(&self, _buffer: &mut [u8]) { todo!("conntrack nat nla emit_value") }
}

impl<'a, T: AsRef<[u8]> + ?Sized> Parseable<NlaBuffer<&'a T>> for ConntrackNatNla {
	fn parse(buf: &NlaBuffer<&'a T>) -> Result<Self, DecodeError> {
		Ok(ConntrackNatNla::Other(DefaultNla::parse(buf)?))
	}
}
