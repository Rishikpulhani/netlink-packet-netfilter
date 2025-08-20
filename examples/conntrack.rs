// SPDX-License-Identifier: MIT

// Example: craft and send a conntrack delete request. This is a draft showing
// how the API will look once Conntrack NLAs and parsing/serialization are
// implemented. Many pieces are still todo!() in the conntrack module, so this
// example focuses on building the Netlink envelope similarly to the nflog
// example.
//
// To run (after implementing ConntrackMessage Emitable + NLAs):
//   1) Build: cargo build --example conntrack
//   2) Run as root: sudo ./target/debug/examples/conntrack
//   3) Adjust the tuple matching NLAs once those enums are implemented.

use netlink_packet_core::{NetlinkHeader, NetlinkMessage, NetlinkPayload, NLM_F_ACK, NLM_F_REQUEST};
use netlink_packet_netfilter::{
	constants::{AF_INET, NFNETLINK_V0},
	conntrack::ConntrackMessage,
	NetfilterHeader, NetfilterMessage,
};
use netlink_sys::{constants::NETLINK_NETFILTER, Socket};

fn delete_request(nlas: Vec<netlink_packet_netfilter::conntrack::nlas::ConntrackNla>) -> NetlinkMessage<NetfilterMessage> {
	// For deletes, the res_id often encodes a conntrack table family (AF_INET/AF_INET6) and zone.
	// Here we set family=AF_INET, version=NFNETLINK_V0, res_id=0.
	let nf_hdr = NetfilterHeader::new(AF_INET, NFNETLINK_V0, 0);

	let inner = ConntrackMessage::Delete(nlas); // message_type will map to IPCTNL_MSG_CT_DELETE

	let mut nl_hdr = NetlinkHeader::default();
	nl_hdr.flags = NLM_F_REQUEST | NLM_F_ACK; // we want an ACK / error response

	let mut msg = NetlinkMessage::new(
		nl_hdr,
		NetlinkPayload::from(NetfilterMessage::new(nf_hdr, inner)),
	);
	msg.finalize();
	msg
}

fn main() {
	// Placeholder: build the attribute list identifying the conntrack entry to delete.
	// Typically you'd include:
	//   - CTA_TUPLE_ORIG with nested IP/PROTO (src/dst IP + ports + L4 proto)
	//   - Optionally CTA_ZONE if using zones
	// Since NLAs are not yet implemented/serialized, we use an empty vec.
	let delete_nlas = vec![]; // TODO: populate with ConntrackNla::TupleOrig([...]) etc.

	let packet = delete_request(delete_nlas);
	println!(">>> (DELETE request draft) {:?}", packet);

	// Open netfilter netlink socket
	let mut socket = Socket::new(NETLINK_NETFILTER).expect("socket");
	socket.bind_auto().expect("bind");

	// Serialize (will panic until ConntrackMessage Emitable is implemented)
	let mut buf = vec![0; packet.header.length as usize];
	// NOTE: This will call todo!() today for conntrack emit; once implemented it works.
	// Remove this expect once implementation is done.
	use std::panic::{catch_unwind, AssertUnwindSafe};
	let serialize_result = catch_unwind(AssertUnwindSafe(|| packet.serialize(&mut buf)));
	if serialize_result.is_err() {
		eprintln!("Conntrack message serialization not yet implemented (expected for now)");
		return;
	}

	// Send the delete request
	socket.send(&buf, 0).expect("send");
}
