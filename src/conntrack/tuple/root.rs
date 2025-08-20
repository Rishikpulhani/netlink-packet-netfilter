// SPDX-License-Identifier: MIT

use netlink_packet_utils::{
	nla::{DefaultNla, Nla, NlaBuffer},
	DecodeError, Parseable,
};

use crate::conntrack::tuple::{ip::ConntrackTupleIpNla, proto::ConntrackTupleProtoNla};

// ctattr_tuple enum (nested inside certain top-level conntrack tuple attributes)
// enum ctattr_tuple {
// 	CTA_TUPLE_UNSPEC,
// 	CTA_TUPLE_IP,
// 	CTA_TUPLE_PROTO,
// 	CTA_TUPLE_ZONE,
// 	__CTA_TUPLE_MAX
// };

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ConntrackTupleNla {
	// removed as just for not set/invalid
	// Unspec, 
	// Nested IP attribute set (at most one)
	Ip(ConntrackTupleIpNla),
	// Nested L4 protocol attribute set (at most one)
	Proto(ConntrackTupleProtoNla),
	Zone,
	Other(DefaultNla),
}

impl Nla for ConntrackTupleNla {
	fn value_len(&self) -> usize { todo!("conntrack tuple nla value_len") }
	fn kind(&self) -> u16 { todo!("conntrack tuple nla kind") }
	fn emit_value(&self, _buffer: &mut [u8]) { todo!("conntrack tuple nla emit_value") }
}

impl<'a, T: AsRef<[u8]> + ?Sized> Parseable<NlaBuffer<&'a T>> for ConntrackTupleNla {
	fn parse(buf: &NlaBuffer<&'a T>) -> Result<Self, DecodeError> {
		Ok(ConntrackTupleNla::Other(DefaultNla::parse(buf)?))
	}
}
