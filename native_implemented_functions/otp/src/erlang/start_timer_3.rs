// wasm32 proptest cannot be compiled at the same time as non-wasm32 proptest, so disable tests that
// use proptest completely for wasm32
//
// See https://github.com/rust-lang/cargo/issues/4866
#[cfg(all(not(target_arch = "wasm32"), test))]
mod test;

use std::sync::Arc;

use liblumen_alloc::erts::exception;
use liblumen_alloc::erts::process::Process;
use liblumen_alloc::erts::term::prelude::Term;

use native_implemented_function::native_implemented_function;

use crate::erlang::start_timer;
use crate::runtime::timer::Timeout;

#[native_implemented_function(start_timer/3)]
pub fn result(
    arc_process: Arc<Process>,
    time: Term,
    destination: Term,
    message: Term,
) -> exception::Result<Term> {
    start_timer(
        time,
        destination,
        Timeout::TimeoutTuple,
        message,
        Default::default(),
        arc_process,
    )
    .map_err(From::from)
}
