use std::time::Duration;
use std::net::SocketAddr;
use std::collections::HashMap;
use std::thread::{self, JoinHandle};
use std::sync::mpsc;

use futures::Future;
use worker::future::{BoxFuture, Runner, Scheduler, Worker};

use raftpb::*;
use raftpb_grpc::*;
use rpc_client::{self, RpcClientHandle};
use timer::{self, TimerHandle};
use persistent::Log;
use state_machine::StateMachine;
use Result;
use Term;
use TimeoutTerm;
use LogIndex;
use ServerId;
use Error;
use Message;

enum State {
    Leader(Leader),
    Follower(Follower),
    Candidate(Candidate),
}

struct Leader {
    next_index: HashMap<ServerId, LogIndex>,
    match_index: HashMap<ServerId, LogIndex>,
}

struct Follower {
    term: TimeoutTerm,
    timeout: Duration,
}

#[derive(Default)]
struct Candidate {
    term: TimeoutTerm,
    timeout: Duration,
    votes: Vec<ServerId>,
}

fn new_follower() -> Follower {
    unimplemented!()
}
fn new_candidate() -> Candidate {
    unimplemented!()
}
fn new_leader() -> Leader {
    unimplemented!()
}

pub struct Consensus<L: Log, S: StateMachine> {
    id: ServerId,
    state: State,
    commit_index: LogIndex,
    last_applied: LogIndex,

    log: L,
    state_machine: S,

    peers: HashMap<ServerId, Peer>,
    next_index: HashMap<ServerId, LogIndex>,
    match_index: HashMap<ServerId, LogIndex>,

    rx: mpsc::Receiver<Message>,
    rpc_client: RpcClientHandle,
    timer: TimerHandle,
}

pub struct Peer {
    id: ServerId,
    addr: SocketAddr,
}

impl<L: Log, S: StateMachine> Consensus<L, S> {
    pub fn new(
        id: ServerId,
        peers: Vec<Peer>,
        log: L,
        st: S,
        rx: mpsc::Receiver<Message>,
        rpc_client: RpcClientHandle,
        timer: TimerHandle,
    ) -> Self {
        let peers: HashMap<_, _> = peers.into_iter().map(|p| (p.id, p)).collect();
        let next_index = peers.keys().map(|k| (*k, 1)).collect();
        let match_index = peers.keys().map(|k| (*k, 0)).collect();
        Consensus {
            id: id,
            state: State::Follower(new_follower()),
            commit_index: 0,
            last_applied: 0,
            log: log,
            state_machine: st,
            peers: peers,
            next_index: next_index,
            match_index: match_index,
            rx: rx,
            rpc_client: rpc_client,
            timer: timer,
        }
    }

    pub fn run(&mut self) {
        self.follower_count_down();
        loop {
            match self.rx.recv().unwrap() {
                Message::FollowerTimeout(term) => {
                    if let Some(c_term) = self.get_follower_timeout_term() {
                        if c_term == term {
                            self.follower_timeout();
                            continue;
                        }
                    }
                    info!("stale follower timeout message");
                }
                Message::CandidateTimeout(term) => {
                    if let Some(c_term) = self.get_candidate_timeout_term() {
                        if c_term == term {
                            self.candidate_timeout();
                            continue;
                        }
                    }
                    info!("stale candidate timeout message");
                }
                Message::AppendEntriesRequest(req) => let _ = self.append_entries_request(req),
                Message::RequestVoteRequest(req) => let _ = self.request_vote_request(req),
                Message::AppendEntriesRequestResult(term, res) => {
                    if term < self.term() {
                        warn!("stale append entries request result message");
                        return;
                    }
                    self.append_entries_request_result(res);
                }
                Message::RequestVoteRequestResult(term, res) => {
                    if term < self.term() {
                        warn!("stale request vote request result message");
                        return;
                    }
                    self.request_vote_request_result(res);
                }
                Message::Stop => break,
            }
        }
        info!("consensus finished");
    }

    fn follower_count_down(&mut self) {
        assert!(self.state.is_follower());
        let term = self.get_follower_timeout_term().unwrap();
        let timeout = self.get_follower_timeout().unwrap();
        let task = timer::new_timeout_task(term, timeout, timer::Request::Follower);
        if self.timer.schedule(task).is_err() {
            warn!("timer was dropped");
        }
    }

    fn candidate_count_down(&mut self) {
        assert!(self.state.is_candidate());
        let term = self.get_candidate_timeout_term().unwrap();
        let timeout = self.get_candidate_timeout().unwrap();
        let task = timer::new_timeout_task(term, timeout, timer::Request::Candidate);
        if self.timer.schedule(task).is_err() {
            warn!("timer was dropped");
        }
    }

    fn follower_timeout(&mut self) {
        assert!(self.state.is_follower());
        self.elect();
    }

    fn candidate_timeout(&mut self) {
        assert!(self.state.is_candidate());
        self.elect();
    }

    fn elect(&mut self) {
        let mut candidate = new_candidate();
        candidate.votes.push(self.id);
        self.state = State::Candidate(candidate);
        self.inc_term();
        self.vote_self();

        let mut req = RequestVoteRequest::new();
        req.term = self.term();
        req.candidate_id = self.id;
        req.last_log_index = self.last_log_index();
        req.last_log_term = self.last_log_term();
        for peer in self.peers.values() {
            let task = rpc_client::new_rpc_task(
                peer.addr,
                req.term,
                rpc_client::Request::RequestVote(req.clone()),
            );
            if self.rpc_client.schedule(task).is_err() {
                warn!("rpc client was dropped");
                break;
            }
        }
        self.candidate_count_down();
    }

    fn append_entries_request(&mut self, req: AppendEntriesRequest) -> AppendEntriesResponse {
        unimplemented!()
    }

    fn request_vote_request(&mut self, req: RequestVoteRequest) -> RequestVoteResponse {
        unimplemented!()
    }

    fn convert_to_follower(&mut self) {
        self.state = State::Follower(new_follower());
        self.follower_count_down();
    }

    fn is_up_to_date(&self, candidate_term: Term, candidate_log_index: LogIndex) -> bool {
        unimplemented!()
    }

    fn append_entries_request_result(&mut self, res: Result<AppendEntriesResponse>) {}

    fn request_vote_request_result(&mut self, res: Result<RequestVoteResponse>) {}

    fn get_follower_timeout_term(&self) -> Option<TimeoutTerm> {
        match self.state {
            State::Follower(ref fo) => Some(fo.term),
            _ => None,
        }
    }

    fn get_follower_timeout(&self) -> Option<Duration> {
        match self.state {
            State::Follower(ref fo) => Some(fo.timeout),
            _ => None,
        }
    }

    fn get_candidate_timeout_term(&self) -> Option<TimeoutTerm> {
        match self.state {
            State::Candidate(ref can) => Some(can.term),
            _ => None,
        }
    }

    fn get_candidate_timeout(&self) -> Option<Duration> {
        match self.state {
            State::Candidate(ref can) => Some(can.timeout),
            _ => None,
        }
    }

    fn term(&self) -> Term {
        unimplemented!()
    }

    fn set_term(&mut self, term: Term) {
        unimplemented!()
    }

    fn last_log_index(&self) -> LogIndex {
        unimplemented!()
    }

    fn last_log_term(&self) -> Term {
        unimplemented!()
    }

    fn inc_term(&mut self) {
        unimplemented!()
    }

    fn vote_self(&mut self) {
        unimplemented!()
    }

    fn voted_for(&self) -> Option<ServerId> {
        unimplemented!()
    }

    fn entry(&self, i: LogIndex) -> Option<Entry> {
        unimplemented!()
    }

    fn append_entries(&mut self, from: LogIndex, entries: Vec<Entry>) {
        unimplemented!()
    }

    fn remove_entries(&mut self, from: LogIndex) {
        unimplemented!()
    }
}

impl Peer {
    pub fn new(id: ServerId, addr: SocketAddr) -> Self {
        Peer { id: id, addr: addr }
    }
}

impl State {
    fn is_leader(&self) -> bool {
        match *self {
            State::Leader(_) => true,
            _ => false,
        }
    }

    fn is_follower(&self) -> bool {
        match *self {
            State::Follower(_) => true,
            _ => false,
        }
    }

    fn is_candidate(&self) -> bool {
        match *self {
            State::Candidate(_) => true,
            _ => false,
        }
    }
}
