/// This trait is meant to be implemented such that the commands issued to it via `apply()` will
/// be reflected in your consuming application. Commands sent via `apply()` have been committed
/// in the cluser. Unlike `store`, your application should consume data produced by this and
/// accept it as truth.
///
/// Note that you are responsible for **not crashing** the state machine. Your production
/// implementation should not use `.unwrap()`, `.expect()` or anything else that likes to `panic!()`
pub trait StateMachine: Send + 'static {
    /// Applies a command to the state machine.
    /// Returns an application-specific result value.
    fn apply(&mut self, command: &[u8]) -> Vec<u8>;

    /// Queries a value of the state machine. Does not go through the durable log, or mutate the
    /// state machine.
    /// Returns an application-specific result value.
    fn query(&self, query: &[u8]) -> Vec<u8>;

    /// Take a snapshot of the state machine.
    fn snapshot(&self) -> Vec<u8>;

    /// Restore a snapshot of the state machine.
    fn restore_snapshot(&mut self, snapshot: Vec<u8>) -> ();
}
