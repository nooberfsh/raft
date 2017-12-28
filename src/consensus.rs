use std::thread::{self, JoinHandle};

enum State {
    Leader(Leader),
    Follower(Follower),
    Candidate(Candidate),
}

struct Leader {

}

struct Follower {
}

struct Candidate {
}

pub struct Consensus {
    state: State,
    thread_handle: JoinHandle<()>,
}

impl  Consensus {

}


