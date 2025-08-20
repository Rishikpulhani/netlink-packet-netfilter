// SPDX-License-Identifier: MIT

use netlink_packet_utils::{
	nla::{DefaultNla, Nla, NlaBuffer},
	DecodeError, Parseable,
};

// enum ctattr_tstamp {
// 	CTA_TIMESTAMP_UNSPEC,
// 	CTA_TIMESTAMP_START,
// 	CTA_TIMESTAMP_STOP,
// 	CTA_TIMESTAMP_PAD,
// 	__CTA_TIMESTAMP_MAX
// };

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ConntrackTimestampNla {
	// removed as just for not set/invalid
	// Unspec,
	Start(u64),
	Stop(u64),
	// removed: padding attribute not for user applications
	// Pad,
	Other(DefaultNla),
}

impl Nla for ConntrackTimestampNla {
	fn value_len(&self) -> usize { todo!("conntrack timestamp nla value_len") }
	fn kind(&self) -> u16 { todo!("conntrack timestamp nla kind") }
	fn emit_value(&self, _buffer: &mut [u8]) { todo!("conntrack timestamp nla emit_value") }
}

impl<'a, T: AsRef<[u8]> + ?Sized> Parseable<NlaBuffer<&'a T>> for ConntrackTimestampNla {
	fn parse(buf: &NlaBuffer<&'a T>) -> Result<Self, DecodeError> {
		Ok(ConntrackTimestampNla::Other(DefaultNla::parse(buf)?))
	}
}
