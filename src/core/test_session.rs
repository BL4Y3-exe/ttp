#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum SessionStatus {
    Waiting,
    Running,
    Finished,
    Aborted,
}
