use proptest::prop_assert_eq;
use proptest::test_runner::{Config, TestRunner};

use crate::erlang::is_integer_1::native;
use crate::test::strategy;
use crate::test::with_process_arc;

#[test]
fn without_integer_returns_false() {
    run!(
        |arc_process| strategy::term::is_not_integer(arc_process.clone()),
        |term| {
            prop_assert_eq!(native(term), false.into());

            Ok(())
        },
    );
}

#[test]
fn with_integer_returns_true() {
    with_process_arc(|arc_process| {
        TestRunner::new(Config::with_source_file(file!()))
            .run(&strategy::term::is_integer(arc_process.clone()), |term| {
                prop_assert_eq!(native(term), true.into());

                Ok(())
            })
            .unwrap();
    });
}
