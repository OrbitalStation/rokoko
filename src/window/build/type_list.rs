/// Terminator
pub struct Empty;

/// Connector
pub struct With <T, N> {
    pub data: T,
    pub next: N
}
