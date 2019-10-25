use super::*;

use std::convert::TryInto;

use liblumen_alloc::erts::process::alloc::heap_alloc::HeapAlloc;
use liblumen_alloc::erts::process::Process;
use liblumen_alloc::erts::term::prelude::*;

use crate::process;
use crate::scheduler::Spawned;

#[test]
fn without_heap_available_errors_alloc() {
    let init_arc_process = process::test_init();
    let Spawned { arc_process, .. } = crate::test::process(&init_arc_process, Default::default());
    let key = Atom::str_to_term("key");
    let value = Atom::str_to_term("value");

    arc_process.put(key, value).unwrap();

    fill_heap(&arc_process);

    assert_eq!(arc_process.get_value_from_key(key), value);

    assert_eq!(native(&arc_process), Err(liblumen_alloc::alloc!().into()));
}

#[test]
fn with_heap_available_returns_entries_as_list() {
    let init_arc_process = process::test_init();
    let Spawned { arc_process, .. } = crate::test::process(&init_arc_process, Default::default());
    let key = Atom::str_to_term("key");
    let value = Atom::str_to_term("value");

    arc_process.put(key, value).unwrap();

    assert_eq!(arc_process.get_value_from_key(key), value);

    let result = native(&arc_process);

    assert!(result.is_ok());

    let list = result.unwrap();

    assert!(list.is_list());

    let boxed_cons: Boxed<Cons> = list.try_into().unwrap();

    let head = boxed_cons.head;

    assert!(head.is_tuple());

    let head_boxed_tuple: Boxed<Tuple> = head.try_into().unwrap();

    assert_eq!(head_boxed_tuple.len(), 2);

    assert_eq!(head_boxed_tuple[0], key);
    assert_eq!(head_boxed_tuple[1], value);

    assert_eq!(boxed_cons.tail, Term::NIL);
}

fn fill_heap(process: &Process) {
    {
        let mut heap = process.acquire_heap();

        while let Ok(_) = heap.cons(Atom::str_to_term("hd"), Atom::str_to_term("tl")) {}
    }
}
