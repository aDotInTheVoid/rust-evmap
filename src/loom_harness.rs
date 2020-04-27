#[cfg(loom)]
pub(crate) use loom::{sync, thread};
#[cfg(not(loom))]
pub(crate) use std::{sync, thread};

//https://github.com/tokio-rs/loom/blob/2e030653e78d83a8fb46d808397e693494e3bdcc/src/rt/atomic.rs#L142-L145
#[cfg(not(loom))]
pub(crate) fn fense_seq_cst() {
    crate::atomic::fence(crate::sync::atomic::Ordering::SeqCst);
}
#[cfg(loom)]
pub(crate) fn fense_seq_cst() {
    crate::atomic::fence(crate::atomic::Ordering::Acquire);
}
