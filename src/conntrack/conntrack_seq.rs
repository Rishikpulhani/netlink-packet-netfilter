// SPDX-License-Identifier: MIT

use netlink_packet_utils::{
	nla::{DefaultNla, Nla, NlaBuffer},
	DecodeError, Parseable,
};

// enum ctattr_seqadj {
// 	CTA_SEQADJ_UNSPEC,
// 	CTA_SEQADJ_CORRECTION_POS,
// 	CTA_SEQADJ_OFFSET_BEFORE,
// 	CTA_SEQADJ_OFFSET_AFTER,
// 	__CTA_SEQADJ_MAX
// };

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ConntrackSeqAdjNla {
	// removed as just for not set/invalid
	// Unspec,
    // inner values may be wrong for now - put as an example payload 
	CorrectionPos(u32),
	OffsetBefore(u32),
	OffsetAfter(u32),
	Other(DefaultNla),
}

impl Nla for ConntrackSeqAdjNla {
	fn value_len(&self) -> usize { todo!("conntrack seqadj nla value_len") }
	fn kind(&self) -> u16 { todo!("conntrack seqadj nla kind") }
	fn emit_value(&self, _buffer: &mut [u8]) { todo!("conntrack seqadj nla emit_value") }
}

impl<'a, T: AsRef<[u8]> + ?Sized> Parseable<NlaBuffer<&'a T>> for ConntrackSeqAdjNla {
	fn parse(buf: &NlaBuffer<&'a T>) -> Result<Self, DecodeError> {
		Ok(ConntrackSeqAdjNla::Other(DefaultNla::parse(buf)?))
	}
}
