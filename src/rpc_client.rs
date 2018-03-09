use std::net::SocketAddr;
use std::sync::mpsc;

use worker::future::{BoxFuture, Runner, Scheduler, Worker};
use worker::Stopped;
use tokio_core::reactor::{Core, Handle};

use raftpb::*;
use raftpb_grpc::*;

use Error;
use Message;
use ServerId;
use Term;

pub enum Request {
    AppendEntries(AppendEntriesRequest),
    RequestVote(RequestVoteRequest),
}

pub struct Task {
    addr: SocketAddr,
    term: Term,
    req: Request,
}

pub fn new_rpc_task(addr: SocketAddr, term: Term, req: Request) -> Task {
    Task {
        addr: addr,
        term: term,
        req: req,
    }
}

pub struct RpcClient {
    tx: mpsc::Sender<Message>,
    worker: Worker<Task>,
}

pub struct RpcClientHandle {
    handle: Scheduler<Task>,
}

impl RpcClientHandle {
    pub fn schedule(&self, task: Task) -> Result<(), Stopped> {
        self.handle.schedule(task)
    }
}

struct ClientRunner {
    tx: mpsc::Sender<Message>,
}

impl Runner<Task> for ClientRunner {
    fn future(&self, task: Task, handle: &Handle) -> BoxFuture {
        unimplemented!()
    }
}
