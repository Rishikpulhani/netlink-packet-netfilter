// SPDX-License-Identifier: MIT

use netlink_packet_utils::{
    nla::{DefaultNla, Nla, NlaBuffer},
    DecodeError, Parseable,
};

// enum ctattr_ip {
// 	CTA_IP_UNSPEC,
// 	CTA_IP_V4_SRC,
// 	CTA_IP_V4_DST,
// 	CTA_IP_V6_SRC,
// 	CTA_IP_V6_DST,
// 	__CTA_IP_MAX
// };

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ConntrackTupleIpNla {
    Unspec,
    V4Src,
    V4Dst,
    V6Src,
    V6Dst,
    Other(DefaultNla),
}

impl Nla for ConntrackTupleIpNla {
    fn value_len(&self) -> usize { todo!("conntrack tuple ip nla value_len") }
    fn kind(&self) -> u16 { todo!("conntrack tuple ip nla kind") }
    fn emit_value(&self, _buffer: &mut [u8]) { todo!("conntrack tuple ip nla emit_value") }
}

impl<'a, T: AsRef<[u8]> + ?Sized> Parseable<NlaBuffer<&'a T>> for ConntrackTupleIpNla {
    fn parse(buf: &NlaBuffer<&'a T>) -> Result<Self, DecodeError> {
        Ok(ConntrackTupleIpNla::Other(DefaultNla::parse(buf)?))
    }
}