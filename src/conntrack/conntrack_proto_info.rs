// SPDX-License-Identifier: MIT

use netlink_packet_utils::{
	nla::{DefaultNla, Nla, NlaBuffer},
	DecodeError, Parseable,
};

use crate::conntrack::{proto_info_tcp::ProtoInfoTcpNla, proto_info_dccp::ProtoInfoDccpNla, proto_info_sctp::ProtoInfoSctpNla};

// enum ctattr_protoinfo {
// 	CTA_PROTOINFO_UNSPEC,
// 	CTA_PROTOINFO_TCP,
// 	CTA_PROTOINFO_DCCP,
// 	CTA_PROTOINFO_SCTP,
// 	__CTA_PROTOINFO_MAX
// };

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ConntrackProtoInfoNla {
	// removed as just for not set/invalid 
	// Unspec,
	Tcp(Vec<ProtoInfoTcpNla>),
	Dccp(Vec<ProtoInfoDccpNla>),
	Sctp(Vec<ProtoInfoSctpNla>),
	Other(DefaultNla),
}

impl Nla for ConntrackProtoInfoNla {
	fn value_len(&self) -> usize { todo!("conntrack protoinfo nla value_len") }
	fn kind(&self) -> u16 { todo!("conntrack protoinfo nla kind") }
	fn emit_value(&self, _buffer: &mut [u8]) { todo!("conntrack protoinfo nla emit_value") }
}

impl<'a, T: AsRef<[u8]> + ?Sized> Parseable<NlaBuffer<&'a T>> for ConntrackProtoInfoNla {
	fn parse(buf: &NlaBuffer<&'a T>) -> Result<Self, DecodeError> {
		Ok(ConntrackProtoInfoNla::Other(DefaultNla::parse(buf)?))
	}
}
