use std::result;

use Term;
use LogIndex;
use ServerId;
use raftpb::Entry;

/// A store of persistent Raft state.
pub trait Log: Send + 'static {
    type Error: Send + 'static;

    /// Returns the latest known term.
    fn current_term(&self) -> result::Result<Term, Self::Error>;

    /// Sets the current term to the provided value. The provided term must be greater than
    /// the current term. The `voted_for` value will be reset`.
    fn set_current_term(&mut self, term: Term) -> result::Result<(), Self::Error>;

    /// Increment the current term. The `voted_for` value will be reset.
    fn inc_current_term(&mut self) -> result::Result<Term, Self::Error>;

    /// Returns the candidate id of the candidate voted for in the current term (or none).
    fn voted_for(&self) -> result::Result<Option<ServerId>, Self::Error>;

    /// Sets the candidate id voted for in the current term.
    fn set_voted_for(&mut self, server: ServerId) -> result::Result<(), Self::Error>;

    /// Returns the index of the latest persisted log entry (0 if the log is empty).
    fn latest_log_index(&self) -> result::Result<LogIndex, Self::Error>;

    /// Returns the term of the latest persisted log entry (0 if the log is empty).
    fn latest_log_term(&self) -> result::Result<Term, Self::Error>;

    /// Returns the entry at the provided log index.
    fn entry(&self, index: LogIndex) -> result::Result<Option<Entry>, Self::Error>;

    /// Returns the given range of entries (excluding the right endpoint).
    fn entries(&self, lo: LogIndex, hi: LogIndex) -> result::Result<Vec<Entry>, Self::Error> {
        let mut ret = vec![];
        for i in lo..hi {
            if let Some(e) = self.entry(i)? {
                ret.push(e);
            }
        }
        Ok(ret)
    }

    /// Appends the provided entries to the log beginning at the given index.
    fn append_entries(
        &mut self,
        from: LogIndex,
        entries: Vec<Entry>,
    ) -> result::Result<(), Self::Error>;
}
