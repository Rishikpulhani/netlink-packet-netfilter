// SPDX-License-Identifier: MIT

use netlink_packet_utils::{
	nla::{DefaultNla, Nla, NlaBuffer},
	DecodeError, Parseable,
};

// enum ctattr_protonat {
// 	CTA_PROTONAT_UNSPEC,
// 	CTA_PROTONAT_PORT_MIN,
// 	CTA_PROTONAT_PORT_MAX,
// 	__CTA_PROTONAT_MAX
// };

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ConntrackProtoNatNla {
	// removed as just for not set/invalid
	// Unspec,
	PortMin(u16),
	PortMax(u16),
	Other(DefaultNla),
}

impl Nla for ConntrackProtoNatNla {
	fn value_len(&self) -> usize { todo!("conntrack protonat nla value_len") }
	fn kind(&self) -> u16 { todo!("conntrack protonat nla kind") }
	fn emit_value(&self, _buffer: &mut [u8]) { todo!("conntrack protonat nla emit_value") }
}

impl<'a, T: AsRef<[u8]> + ?Sized> Parseable<NlaBuffer<&'a T>> for ConntrackProtoNatNla {
	fn parse(buf: &NlaBuffer<&'a T>) -> Result<Self, DecodeError> {
		Ok(ConntrackProtoNatNla::Other(DefaultNla::parse(buf)?))
	}
}