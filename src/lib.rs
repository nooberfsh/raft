#![feature(conservative_impl_trait)]

extern crate futures;
extern crate grpcio;
#[macro_use]
extern crate log;
extern crate protobuf;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate tokio_core;
extern crate worker;

pub mod consensus;
pub mod persistent;
pub mod state_machine;
pub mod raftpb;
pub mod raftpb_grpc;
pub mod rpc_client;
pub mod timer;

use raftpb::*;

#[derive(Debug)]
pub enum Error {

}

pub type Result<T> = std::result::Result<T, Error>;

pub type Term = u64;
pub type TimeoutTerm = u64;
pub type LogIndex = u64;
pub type ServerId = u64;

pub enum Message {
    FollowerTimeout(TimeoutTerm),
    CandidateTimeout(TimeoutTerm),
    AppendEntriesRequest(AppendEntriesRequest),
    RequestVoteRequest(RequestVoteRequest),
    AppendEntriesRequestResult(Term, Result<AppendEntriesResponse>),
    RequestVoteRequestResult(Term, Result<RequestVoteResponse>),
    Stop,
}
