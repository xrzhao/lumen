use core::sync::atomic::{self, AtomicUsize};

use lazy_static::lazy_static;

/// Generates the next `ID`.  `ID`s are not reused for the lifetime of the VM.
pub fn next() -> ID {
    let raw = ID_COUNTER.fetch_add(1, atomic::Ordering::AcqRel);

    ID(raw)
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct ID(usize);

lazy_static! {
    static ref ID_COUNTER: AtomicUsize = AtomicUsize::new(0);
}
