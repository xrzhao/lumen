use super::*;

#[test]
fn with_number_atom_reference_function_port_pid_or_tuple_second_returns_second() {
    TestRunner::new(Config::with_source_file(file!()))
        .run(
            &strategy::process().prop_flat_map(|arc_process| {
                (
                    strategy::term::map(arc_process.clone()),
                    strategy::term::number_atom_reference_function_port_pid_or_tuple(arc_process),
                )
            }),
            |(first, second)| {
                prop_assert_eq!(erlang::min_2(first, second), second);

                Ok(())
            },
        )
        .unwrap();
}

#[test]
fn with_smaller_map_second_returns_second() {
    min(
        |_, process| {
            process
                .map_from_slice(&[(atom_unchecked("a"), process.integer(1).unwrap())])
                .unwrap()
        },
        Second,
    );
}

#[test]
fn with_same_size_map_with_lesser_keys_returns_second() {
    min(
        |_, process| {
            process
                .map_from_slice(&[
                    (atom_unchecked("a"), process.integer(2).unwrap()),
                    (atom_unchecked("b"), process.integer(3).unwrap()),
                ])
                .unwrap()
        },
        Second,
    );
}

#[test]
fn with_same_size_map_with_same_keys_with_lesser_values_returns_second() {
    min(
        |_, process| {
            process
                .map_from_slice(&[
                    (atom_unchecked("b"), process.integer(2).unwrap()),
                    (atom_unchecked("c"), process.integer(2).unwrap()),
                ])
                .unwrap()
        },
        Second,
    );
}

#[test]
fn with_same_map_returns_first() {
    min(|first, _| first, First);
}

#[test]
fn with_same_value_map_returns_first() {
    min(
        |_, process| {
            process
                .map_from_slice(&[
                    (atom_unchecked("b"), process.integer(2).unwrap()),
                    (atom_unchecked("c"), process.integer(3).unwrap()),
                ])
                .unwrap()
        },
        First,
    );
}

#[test]
fn with_same_size_map_with_same_keys_with_greater_values_returns_first() {
    min(
        |_, process| {
            process
                .map_from_slice(&[
                    (atom_unchecked("b"), process.integer(3).unwrap()),
                    (atom_unchecked("c"), process.integer(4).unwrap()),
                ])
                .unwrap()
        },
        First,
    );
}

#[test]
fn with_same_size_map_with_greater_keys_returns_first() {
    min(
        |_, process| {
            process
                .map_from_slice(&[
                    (atom_unchecked("c"), process.integer(2).unwrap()),
                    (atom_unchecked("d"), process.integer(3).unwrap()),
                ])
                .unwrap()
        },
        First,
    );
}

#[test]
fn with_greater_size_map_returns_first() {
    min(
        |_, process| {
            process
                .map_from_slice(&[
                    (atom_unchecked("a"), process.integer(1).unwrap()),
                    (atom_unchecked("b"), process.integer(2).unwrap()),
                    (atom_unchecked("c"), process.integer(3).unwrap()),
                ])
                .unwrap()
        },
        First,
    );
}

#[test]
fn with_list_or_bitstring_second_returns_first() {
    TestRunner::new(Config::with_source_file(file!()))
        .run(
            &strategy::process().prop_flat_map(|arc_process| {
                (
                    strategy::term::map(arc_process.clone()),
                    strategy::term::list_or_bitstring(arc_process.clone()),
                )
            }),
            |(first, second)| {
                prop_assert_eq!(erlang::min_2(first, second), first);

                Ok(())
            },
        )
        .unwrap();
}

fn min<R>(second: R, which: FirstSecond)
where
    R: FnOnce(Term, &ProcessControlBlock) -> Term,
{
    super::min(
        |process| {
            process
                .map_from_slice(&[
                    (atom_unchecked("b"), process.integer(2).unwrap()),
                    (atom_unchecked("c"), process.integer(3).unwrap()),
                ])
                .unwrap()
        },
        second,
        which,
    );
}
