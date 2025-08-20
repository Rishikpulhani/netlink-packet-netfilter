// SPDX-License-Identifier: MIT

use netlink_packet_utils::{
	nla::{DefaultNla, Nla, NlaBuffer},
	DecodeError, Parseable,
};

use crate::conntrack::{
	tuple::ConntrackTupleNla,
	proto_info::ConntrackProtoInfoNla,
	counters::ConntrackCountersNla,
	timestamp::ConntrackTimestampNla,
	nat::ConntrackNatAttrs,
	seq::ConntrackSeqAdjNla,
};

/// Conntrack netlink attributes placeholder.
///
/// This mirrors the style of the nflog `ConfigNla` enum, but will be filled
/// with the concrete conntrack attribute variants later. For now we only keep
/// an `Other` variant wrapping a `DefaultNla` so the plumbing compiles.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ConntrackNla {
	// CTA_UNSPEC
    // removed as just for not set/invalid 
	// Unspec, 
	// CTA_TUPLE_ORIG
	TupleOrig(Vec<ConntrackTupleNla>),
	// CTA_TUPLE_REPLY
	TupleReply(Vec<ConntrackTupleNla>),
	// CTA_STATUS
	Status,
	// CTA_PROTOINFO
	ProtoInfo(ConntrackProtoInfoNla),
	// CTA_HELP
	Help,
	// CTA_NAT_SRC (aka CTA_NAT for backwards compatibility)
	NatSrc(ConntrackNatAttrs), // as this is a vector of ConntrackNatNlas
	// CTA_TIMEOUT
	Timeout,
	// CTA_MARK
	Mark,
	// CTA_COUNTERS_ORIG
	CountersOrig(Vec<ConntrackCountersNla>),
	// CTA_COUNTERS_REPLY
	CountersReply(Vec<ConntrackCountersNla>),
	// CTA_USE
	Use,
	// CTA_ID
	Id,
	// CTA_NAT_DST
	NatDst(ConntrackNatAttrs), // as this is a vector of ConntrackNatNlas
	// CTA_TUPLE_MASTER
	TupleMaster(Vec<ConntrackTupleNla>), // used in exp may be removed later 
	// CTA_SEQ_ADJ_ORIG (CTA_NAT_SEQ_ADJ_ORIG is alias)
	SeqAdjOrig(Vec<ConntrackSeqAdjNla>),
	// CTA_SEQ_ADJ_REPLY (CTA_NAT_SEQ_ADJ_REPLY is alias)
	SeqAdjReply(Vec<ConntrackSeqAdjNla>),
	// CTA_SECMARK (obsolete)
	SecMark,
	// CTA_ZONE
	Zone,
	// CTA_SECCTX
	SecCtx,
	// CTA_TIMESTAMP
	Timestamp(Vec<ConntrackTimestampNla>),
	// CTA_MARK_MASK
	MarkMask,
	// CTA_LABELS
	Labels,
	// CTA_LABELS_MASK
	LabelsMask,
	// CTA_SYNPROXY
    // not implemented for now 
	// SynProxy, 
	// CTA_FILTER
	Filter,
	// CTA_STATUS_MASK
	StatusMask,
	// CTA_TIMESTAMP_EVENT
	TimestampEvent,
	/// Fallback / unknown or not yet modeled attribute.
	Other(DefaultNla),
}

impl Nla for ConntrackNla {
	fn value_len(&self) -> usize { todo!("conntrack nla value_len to be implemented") }
	fn kind(&self) -> u16 { todo!("conntrack nla kind to be implemented") }
	fn emit_value(&self, _buffer: &mut [u8]) { todo!("conntrack nla emit_value to be implemented") }
}

impl<'a, T: AsRef<[u8]> + ?Sized> Parseable<NlaBuffer<&'a T>> for ConntrackNla {
	fn parse(buf: &NlaBuffer<&'a T>) -> Result<Self, DecodeError> {
		// For now just fall back to DefaultNla so we can build end-to-end.
		Ok(ConntrackNla::Other(DefaultNla::parse(buf)?))
	}
}
