pub mod puts_1;

use liblumen_alloc::erts::term::prelude::Atom;

fn module() -> Atom {
    Atom::try_from_str("Elixir.IO").unwrap()
}
