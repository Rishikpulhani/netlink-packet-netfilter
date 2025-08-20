// SPDX-License-Identifier: MIT

use netlink_packet_utils::{
	nla::{DefaultNla, Nla, NlaBuffer},
	DecodeError, Parseable,
};

// enum ctattr_protoinfo_tcp {
// 	CTA_PROTOINFO_TCP_UNSPEC,
// 	CTA_PROTOINFO_TCP_STATE,
// 	CTA_PROTOINFO_TCP_WSCALE_ORIGINAL,
// 	CTA_PROTOINFO_TCP_WSCALE_REPLY,
// 	CTA_PROTOINFO_TCP_FLAGS_ORIGINAL,
// 	CTA_PROTOINFO_TCP_FLAGS_REPLY,
// 	__CTA_PROTOINFO_TCP_MAX
// };

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ProtoInfoTcpNla {
	// removed as just for not set/invalid 
	// Unspec,
	State,
	WScaleOriginal,
	WScaleReply,
	FlagsOriginal,
	FlagsReply,
	Other(DefaultNla),
}

impl Nla for ProtoInfoTcpNla {
	fn value_len(&self) -> usize { todo!("protoinfo tcp nla value_len") }
	fn kind(&self) -> u16 { todo!("protoinfo tcp nla kind") }
	fn emit_value(&self, _buffer: &mut [u8]) { todo!("protoinfo tcp nla emit_value") }
}

impl<'a, T: AsRef<[u8]> + ?Sized> Parseable<NlaBuffer<&'a T>> for ProtoInfoTcpNla {
	fn parse(buf: &NlaBuffer<&'a T>) -> Result<Self, DecodeError> {
		Ok(ProtoInfoTcpNla::Other(DefaultNla::parse(buf)?))
	}
}
