// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

const METHOD_RAFT_APPEND_ENTRIES: ::grpcio::Method<super::raftpb::AppendEntriesRequest, super::raftpb::AppendEntriesResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/raftpb.Raft/AppendEntries",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_RAFT_RQUEST_VOTE: ::grpcio::Method<super::raftpb::RquestVoteRequest, super::raftpb::RquestVoteResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/raftpb.Raft/RquestVote",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

pub struct RaftClient {
    client: ::grpcio::Client,
}

impl RaftClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        RaftClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn append_entries_opt(&self, req: &super::raftpb::AppendEntriesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::raftpb::AppendEntriesResponse> {
        self.client.unary_call(&METHOD_RAFT_APPEND_ENTRIES, req, opt)
    }

    pub fn append_entries(&self, req: &super::raftpb::AppendEntriesRequest) -> ::grpcio::Result<super::raftpb::AppendEntriesResponse> {
        self.append_entries_opt(req, ::grpcio::CallOption::default())
    }

    pub fn append_entries_async_opt(&self, req: &super::raftpb::AppendEntriesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::raftpb::AppendEntriesResponse>> {
        self.client.unary_call_async(&METHOD_RAFT_APPEND_ENTRIES, req, opt)
    }

    pub fn append_entries_async(&self, req: &super::raftpb::AppendEntriesRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::raftpb::AppendEntriesResponse>> {
        self.append_entries_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn rquest_vote_opt(&self, req: &super::raftpb::RquestVoteRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::raftpb::RquestVoteResponse> {
        self.client.unary_call(&METHOD_RAFT_RQUEST_VOTE, req, opt)
    }

    pub fn rquest_vote(&self, req: &super::raftpb::RquestVoteRequest) -> ::grpcio::Result<super::raftpb::RquestVoteResponse> {
        self.rquest_vote_opt(req, ::grpcio::CallOption::default())
    }

    pub fn rquest_vote_async_opt(&self, req: &super::raftpb::RquestVoteRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::raftpb::RquestVoteResponse>> {
        self.client.unary_call_async(&METHOD_RAFT_RQUEST_VOTE, req, opt)
    }

    pub fn rquest_vote_async(&self, req: &super::raftpb::RquestVoteRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::raftpb::RquestVoteResponse>> {
        self.rquest_vote_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait Raft {
    fn append_entries(&self, ctx: ::grpcio::RpcContext, req: super::raftpb::AppendEntriesRequest, sink: ::grpcio::UnarySink<super::raftpb::AppendEntriesResponse>);
    fn rquest_vote(&self, ctx: ::grpcio::RpcContext, req: super::raftpb::RquestVoteRequest, sink: ::grpcio::UnarySink<super::raftpb::RquestVoteResponse>);
}

pub fn create_raft<S: Raft + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_RAFT_APPEND_ENTRIES, move |ctx, req, resp| {
        instance.append_entries(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_RAFT_RQUEST_VOTE, move |ctx, req, resp| {
        instance.rquest_vote(ctx, req, resp)
    });
    builder.build()
}
