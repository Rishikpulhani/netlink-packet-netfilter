// SPDX-License-Identifier: MIT

use netlink_packet_utils::{
	nla::{DefaultNla, Nla, NlaBuffer},
	DecodeError, Parseable,
};

// enum ctattr_protoinfo_sctp {
// 	CTA_PROTOINFO_SCTP_UNSPEC,
// 	CTA_PROTOINFO_SCTP_STATE,
// 	CTA_PROTOINFO_SCTP_VTAG_ORIGINAL,
// 	CTA_PROTOINFO_SCTP_VTAG_REPLY,
// 	__CTA_PROTOINFO_SCTP_MAX
// };

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ProtoInfoSctpNla {
	// removed as just for not set/invalid 
	// Unspec,
	State,
	VTagOriginal,
	VTagReply,
	Other(DefaultNla),
}

impl Nla for ProtoInfoSctpNla {
	fn value_len(&self) -> usize { todo!("protoinfo sctp nla value_len") }
	fn kind(&self) -> u16 { todo!("protoinfo sctp nla kind") }
	fn emit_value(&self, _buffer: &mut [u8]) { todo!("protoinfo sctp nla emit_value") }
}

impl<'a, T: AsRef<[u8]> + ?Sized> Parseable<NlaBuffer<&'a T>> for ProtoInfoSctpNla {
	fn parse(buf: &NlaBuffer<&'a T>) -> Result<Self, DecodeError> {
		Ok(ProtoInfoSctpNla::Other(DefaultNla::parse(buf)?))
	}
}
