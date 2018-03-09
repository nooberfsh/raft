use std::time::Duration;
use std::sync::mpsc;

use futures::Future;
use tokio_core::reactor::{Handle, Timeout};
use worker::Stopped;
use worker::future::{BoxFuture, Runner, Scheduler, Worker};

use TimeoutTerm;
use Error;
use Message;

pub enum Request {
    Follower,
    Candidate,
}

pub struct Task {
    timeout: Duration,
    term: TimeoutTerm,
    req: Request,
}

pub fn new_timeout_task(term: TimeoutTerm, timeout: Duration, req: Request) -> Task {
    Task {
        timeout: timeout,
        term: term,
        req: req,
    }
}

pub struct Timer {
    tx: mpsc::Sender<Message>,
    worker: Worker<Task>,
}

pub struct TimerHandle {
    handle: Scheduler<Task>,
}

impl TimerHandle {
    pub fn schedule(&self, task: Task) -> Result<(), Stopped> {
        self.handle.schedule(task)
    }
}

struct TimerRunner {
    tx: mpsc::Sender<Message>,
}

impl Runner<Task> for TimerRunner {
    fn future(&self, task: Task, handle: &Handle) -> BoxFuture {
        unimplemented!()
    }
}
