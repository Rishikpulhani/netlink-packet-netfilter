// SPDX-License-Identifier: MIT

use netlink_packet_utils::{
	nla::{DefaultNla, Nla, NlaBuffer},
	DecodeError, Parseable,
};

// enum ctattr_counters {
// 	CTA_COUNTERS_UNSPEC,
// 	CTA_COUNTERS_PACKETS,		/* 64bit counters */
// 	CTA_COUNTERS_BYTES,		/* 64bit counters */
// 	CTA_COUNTERS32_PACKETS,		/* old 32bit counters, unused */
// 	CTA_COUNTERS32_BYTES,		/* old 32bit counters, unused */
// 	CTA_COUNTERS_PAD,
// 	__CTA_COUNTERS_MAX
// };

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ConntrackCountersNla {
	// removed as just for not set/invalid
	// Unspec,
	Packets(u64),
	Bytes(u64),
	Packets32(u32), // this is legacy and might   be removed and not used 
	Bytes32(u32), // this is legacy and might   be removed and not used
	// not for user applications and might get removed 
	// Pad, // no payload
	Other(DefaultNla),
}

impl Nla for ConntrackCountersNla {
	fn value_len(&self) -> usize { todo!("conntrack counters nla value_len") }
	fn kind(&self) -> u16 { todo!("conntrack counters nla kind") }
	fn emit_value(&self, _buffer: &mut [u8]) { todo!("conntrack counters nla emit_value") }
}

impl<'a, T: AsRef<[u8]> + ?Sized> Parseable<NlaBuffer<&'a T>> for ConntrackCountersNla {
	fn parse(buf: &NlaBuffer<&'a T>) -> Result<Self, DecodeError> {
		Ok(ConntrackCountersNla::Other(DefaultNla::parse(buf)?))
	}
}
