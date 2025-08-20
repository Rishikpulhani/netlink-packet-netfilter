// SPDX-License-Identifier: MIT

use netlink_packet_utils::{
	nla::{DefaultNla, Nla, NlaBuffer},
	DecodeError, Parseable,
};

// enum ctattr_l4proto {
// 	CTA_PROTO_UNSPEC,
// 	CTA_PROTO_NUM,
// 	CTA_PROTO_SRC_PORT,
// 	CTA_PROTO_DST_PORT,
// 	CTA_PROTO_ICMP_ID,
// 	CTA_PROTO_ICMP_TYPE,
// 	CTA_PROTO_ICMP_CODE,
// 	CTA_PROTO_ICMPV6_ID,
// 	CTA_PROTO_ICMPV6_TYPE,
// 	CTA_PROTO_ICMPV6_CODE,
// 	__CTA_PROTO_MAX
// };

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ConntrackTupleProtoNla {
	// removed as just for not set/invalid 
	// Unspec,
	Num,
	SrcPort,
	DstPort,
	IcmpId,
	IcmpType,
	IcmpCode,
	Icmpv6Id,
	Icmpv6Type,
	Icmpv6Code,
	Other(DefaultNla),
}

impl Nla for ConntrackTupleProtoNla {
	fn value_len(&self) -> usize { todo!("conntrack tuple proto nla value_len") }
	fn kind(&self) -> u16 { todo!("conntrack tuple proto nla kind") }
	fn emit_value(&self, _buffer: &mut [u8]) { todo!("conntrack tuple proto nla emit_value") }
}

impl<'a, T: AsRef<[u8]> + ?Sized> Parseable<NlaBuffer<&'a T>> for ConntrackTupleProtoNla {
	fn parse(buf: &NlaBuffer<&'a T>) -> Result<Self, DecodeError> {
		Ok(ConntrackTupleProtoNla::Other(DefaultNla::parse(buf)?))
	}
}
