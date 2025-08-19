// SPDX-License-Identifier: MIT

use netlink_packet_utils::{DecodeError, Emitable, Parseable, ParseableParametrized};

use crate::{
	buffer::NetfilterBuffer,
	constants::{
		NFNL_SUBSYS_CTNETLINK, IPCTNL_MSG_CT_NEW, IPCTNL_MSG_CT_GET,
		IPCTNL_MSG_CT_DELETE, IPCTNL_MSG_CT_GET_CTRZERO, IPCTNL_MSG_CT_GET_STATS_CPU,
		IPCTNL_MSG_CT_GET_STATS, IPCTNL_MSG_CT_GET_DYING, IPCTNL_MSG_CT_GET_UNCONFIRMED,
	},
	conntrack::nlas::ConntrackNla,
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ConntrackMessage {
	New(Vec<ConntrackNla>),
	Get(Vec<ConntrackNla>),
	Delete(Vec<ConntrackNla>),
	GetCtrZero(Vec<ConntrackNla>),
	GetStatsCpu(Vec<ConntrackNla>),
	GetStats(Vec<ConntrackNla>),
	GetDying(Vec<ConntrackNla>),
	GetUnconfirmed(Vec<ConntrackNla>),
	Other { message_type: u8, nlas: Vec<ConntrackNla> }, // everything is covered but just for new additions
}

impl ConntrackMessage {
	pub const SUBSYS: u8 = NFNL_SUBSYS_CTNETLINK;

	pub fn message_type(&self) -> u8 {
		match self {
			ConntrackMessage::New(_) => IPCTNL_MSG_CT_NEW,
			ConntrackMessage::Get(_) => IPCTNL_MSG_CT_GET,
			ConntrackMessage::Delete(_) => IPCTNL_MSG_CT_DELETE,
			ConntrackMessage::GetCtrZero(_) => IPCTNL_MSG_CT_GET_CTRZERO,
			ConntrackMessage::GetStatsCpu(_) => IPCTNL_MSG_CT_GET_STATS_CPU,
			ConntrackMessage::GetStats(_) => IPCTNL_MSG_CT_GET_STATS,
			ConntrackMessage::GetDying(_) => IPCTNL_MSG_CT_GET_DYING,
			ConntrackMessage::GetUnconfirmed(_) => IPCTNL_MSG_CT_GET_UNCONFIRMED,
			ConntrackMessage::Other { message_type, .. } => *message_type,
		}
	}
}

impl Emitable for ConntrackMessage {
	fn buffer_len(&self) -> usize { todo!("implement buffer length for conntrack messages") }
	fn emit(&self, _buffer: &mut [u8]) { todo!("emit conntrack message nlas") }
}

impl<'a, T: AsRef<[u8]> + ?Sized> ParseableParametrized<NetfilterBuffer<&'a T>, u8> for ConntrackMessage {
	fn parse_with_param(buf: &NetfilterBuffer<&'a T>, message_type: u8) -> Result<Self, DecodeError> {
		todo!("implement conntrack message parsing")
	}
}

