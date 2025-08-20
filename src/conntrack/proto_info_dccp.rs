// SPDX-License-Identifier: MIT

use netlink_packet_utils::{
	nla::{DefaultNla, Nla, NlaBuffer},
	DecodeError, Parseable,
};

// enum ctattr_protoinfo_dccp {
// 	CTA_PROTOINFO_DCCP_UNSPEC,
// 	CTA_PROTOINFO_DCCP_STATE,
// 	CTA_PROTOINFO_DCCP_ROLE,
// 	CTA_PROTOINFO_DCCP_HANDSHAKE_SEQ,
// 	CTA_PROTOINFO_DCCP_PAD,
// 	__CTA_PROTOINFO_DCCP_MAX,
// };

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ProtoInfoDccpNla {
	// removed as just for not set/invalid 
	// Unspec,
	State,
	Role,
	HandshakeSeq,
    // not for user applications and might get removed 
	// Pad, 
	Other(DefaultNla),
}

impl Nla for ProtoInfoDccpNla {
	fn value_len(&self) -> usize { todo!("protoinfo dccp nla value_len") }
	fn kind(&self) -> u16 { todo!("protoinfo dccp nla kind") }
	fn emit_value(&self, _buffer: &mut [u8]) { todo!("protoinfo dccp nla emit_value") }
}

impl<'a, T: AsRef<[u8]> + ?Sized> Parseable<NlaBuffer<&'a T>> for ProtoInfoDccpNla {
	fn parse(buf: &NlaBuffer<&'a T>) -> Result<Self, DecodeError> {
		Ok(ProtoInfoDccpNla::Other(DefaultNla::parse(buf)?))
	}
}
