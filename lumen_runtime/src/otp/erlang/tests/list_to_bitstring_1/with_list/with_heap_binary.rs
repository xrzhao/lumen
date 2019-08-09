use super::*;

#[test]
fn without_byte_bitstring_or_list_element_errors_badarg() {
    with_process_arc(|arc_process| {
        TestRunner::new(Config::with_source_file(file!()))
            .run(
                &(
                    strategy::term::binary::heap(arc_process.clone()),
                    is_not_byte_bitstring_nor_list(arc_process.clone()),
                )
                    .prop_map(|(head, tail)| arc_process.cons(head, tail).unwrap()),
                |list| {
                    prop_assert_eq!(
                        erlang::list_to_bitstring_1(list, &arc_process),
                        Err(badarg!().into())
                    );

                    Ok(())
                },
            )
            .unwrap();
    });
}

#[test]
fn with_empty_list_returns_binary() {
    with(|head, process| {
        let tail = Term::NIL;
        let iolist = process.cons(head, tail).unwrap();

        assert_eq!(
            erlang::list_to_bitstring_1(iolist, &process),
            Ok(process.binary_from_bytes(&[0, 1]).unwrap())
        );
    })
}

#[test]
fn with_improper_list_returns_binary() {
    with_tail_errors_badarg(|process| {
        let tail_head = process.integer(1).unwrap();
        let tail_tail = process.integer(2).unwrap();

        process.cons(tail_head, tail_tail).unwrap()
    });
}

#[test]
fn with_proper_list_returns_binary() {
    with(|head, process| {
        let tail_head_byte = 2;
        let tail_head = process.integer(tail_head_byte).unwrap();
        let tail_tail = Term::NIL;
        let tail = process.cons(tail_head, tail_tail).unwrap();

        let iolist = process.cons(head, tail).unwrap();

        assert_eq!(
            erlang::list_to_bitstring_1(iolist, &process),
            Ok(process.binary_from_bytes(&[0, 1, 2]).unwrap())
        );
    });
}

#[test]
fn with_heap_binary_returns_binary() {
    with(|head, process| {
        let tail = process.binary_from_bytes(&[2, 3]).unwrap();

        let iolist = process.cons(head, tail).unwrap();

        assert_eq!(
            erlang::list_to_bitstring_1(iolist, &process),
            Ok(process.binary_from_bytes(&[0, 1, 2, 3]).unwrap())
        );
    })
}

#[test]
fn with_subbinary_with_bit_count_0_returns_binary() {
    with(|head, process| {
        let original = process
            .binary_from_bytes(&[0b1111_1111, 0b0000_0000])
            .unwrap();
        let tail = process
            .subbinary_from_original(original, 0, 0, 1, 0)
            .unwrap();

        let iolist = process.cons(head, tail).unwrap();

        assert_eq!(
            erlang::list_to_bitstring_1(iolist, &process),
            Ok(process.binary_from_bytes(&[0, 1, 255]).unwrap())
        );
    });
}

#[test]
fn with_subbinary_with_bit_count_1_returns_subbinary() {
    with(|head, process| {
        let tail = bitstring!(0b1010_1010, 0b1 :: 1, &process);
        let iolist = process.cons(head, tail).unwrap();

        assert_eq!(
            erlang::list_to_bitstring_1(iolist, &process),
            Ok(bitstring!(0, 1, 0b1010_1010, 0b1 :: 1, &process))
        );
    });
}

#[test]
fn with_subbinary_with_bit_count_2_returns_subbinary() {
    with(|head, process| {
        let tail = bitstring!(0b0101_0101, 0b01 :: 2, &process);
        let iolist = process.cons(head, tail).unwrap();

        assert_eq!(
            erlang::list_to_bitstring_1(iolist, &process),
            Ok(bitstring!(0, 1, 0b0101_0101, 0b01 :: 2, &process))
        );
    });
}

#[test]
fn with_subbinary_with_bit_count_3_returns_subbinary() {
    with(|head, process| {
        let tail = bitstring!(0b1010_1010, 0b101 :: 3, &process);
        let iolist = process.cons(head, tail).unwrap();

        assert_eq!(
            erlang::list_to_bitstring_1(iolist, &process),
            Ok(bitstring!(0, 1, 0b1010_1010, 0b101 :: 3, &process))
        );
    });
}

#[test]
fn with_subbinary_with_bit_count_4_returns_subbinary() {
    with(|head, process| {
        let tail = bitstring!(0b0101_0101, 0b0101 :: 4, &process);
        let iolist = process.cons(head, tail).unwrap();

        assert_eq!(
            erlang::list_to_bitstring_1(iolist, &process),
            Ok(bitstring!(0, 1, 0b0101_0101, 0b0101 :: 4, &process))
        );
    });
}

#[test]
fn with_subbinary_with_bit_count_5_returns_subbinary() {
    with(|head, process| {
        let tail = bitstring!(0b1010_1010, 0b1010_1 :: 5, &process);
        let iolist = process.cons(head, tail).unwrap();

        assert_eq!(
            erlang::list_to_bitstring_1(iolist, &process),
            Ok(bitstring!(0, 1, 0b1010_1010, 0b1010_1 :: 5, &process))
        );
    });
}

#[test]
fn with_subbinary_with_bit_count_6_returns_subbinary() {
    with(|head, process| {
        let tail = bitstring!(0b0101_0101, 0b0101_0100 :: 6, &process);
        let iolist = process.cons(head, tail).unwrap();

        assert_eq!(
            erlang::list_to_bitstring_1(iolist, &process),
            Ok(bitstring!(0, 1, 0b0101_0101, 0b0101_0100 :: 6, &process))
        );
    });
}

#[test]
fn with_subbinary_with_bit_count_7_returns_subbinary() {
    with(|head, process| {
        let tail = bitstring!(0b1010_1010, 0b1010_101 :: 7, &process);
        let iolist = process.cons(head, tail).unwrap();

        assert_eq!(
            erlang::list_to_bitstring_1(iolist, &process),
            Ok(bitstring!(0, 1, 0b1010_1010, 0b1010_101 :: 7, &process))
        );
    });
}

fn with_tail_errors_badarg<T>(tail: T)
where
    T: FnOnce(&ProcessControlBlock) -> Term,
{
    with(|head, process| {
        let iolist = process.cons(head, tail(&process)).unwrap();

        assert_badarg!(erlang::list_to_bitstring_1(iolist, &process));
    });
}

fn with<F>(f: F)
where
    F: FnOnce(Term, &ProcessControlBlock) -> (),
{
    with_process(|process| {
        let head = process.binary_from_bytes(&[0, 1]).unwrap();

        f(head, &process);
    })
}
