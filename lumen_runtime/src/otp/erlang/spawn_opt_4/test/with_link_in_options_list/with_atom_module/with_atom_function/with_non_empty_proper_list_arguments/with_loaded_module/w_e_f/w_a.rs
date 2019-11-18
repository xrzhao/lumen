use super::*;

use crate::otp::erlang;

#[test]
fn with_valid_arguments_when_run_exits_normal_and_parent_does_not_exit() {
    let parent_arc_process = process::test_init();
    let arc_scheduler = Scheduler::current();

    let priority = Priority::Normal;
    let run_queue_length_before = arc_scheduler.run_queue_len(priority);

    erlang::number_or_badarith_1::export();

    let module_atom = erlang::module();
    let module: Term = module_atom.encode().unwrap();

    let function_atom = erlang::number_or_badarith_1::function();
    let function: Term = function_atom.encode().unwrap();

    let number = parent_arc_process.integer(0).unwrap();
    let arguments = parent_arc_process.cons(number, Term::NIL).unwrap();

    let result = native(
        &parent_arc_process,
        module,
        function,
        arguments,
        options(&parent_arc_process),
    );

    assert!(result.is_ok());

    let child_pid = result.unwrap();
    let child_pid_result_pid: Result<Pid, _> = child_pid.try_into();

    assert!(child_pid_result_pid.is_ok());

    let child_pid_pid = child_pid_result_pid.unwrap();

    let run_queue_length_after = arc_scheduler.run_queue_len(priority);

    assert_eq!(run_queue_length_after, run_queue_length_before + 1);

    let arc_process = pid_to_process(&child_pid_pid).unwrap();

    assert!(arc_scheduler.run_through(&arc_process));
    assert!(!arc_scheduler.run_through(&arc_process));

    assert_eq!(arc_process.code_stack_len(), 0);
    assert_eq!(arc_process.current_module_function_arity(), None);

    match *arc_process.status.read() {
        Status::Exiting(ref runtime_exception) => {
            assert_eq!(runtime_exception, &exit!(atom!("normal")));
        }
        ref status => panic!("Process status ({:?}) is not exiting.", status),
    };
}

#[test]
fn without_valid_arguments_when_run_exits_and_parent_exits() {
    let parent_arc_process = process::test_init();
    let arc_scheduler = Scheduler::current();

    let priority = Priority::Normal;
    let run_queue_length_before = arc_scheduler.run_queue_len(priority);

    erlang::number_or_badarith_1::export();

    let module_atom = erlang::module();
    let module: Term = module_atom.encode().unwrap();

    let function_atom = erlang::number_or_badarith_1::function();
    let function: Term = function_atom.encode().unwrap();

    // not a number
    let number = atom!("zero");
    let arguments = parent_arc_process.cons(number, Term::NIL).unwrap();

    let result = native(
        &parent_arc_process,
        module,
        function,
        arguments,
        options(&parent_arc_process),
    );

    assert!(result.is_ok());

    let child_pid = result.unwrap();
    let child_pid_result_pid: Result<Pid, _> = child_pid.try_into();

    assert!(child_pid_result_pid.is_ok());

    let child_pid_pid = child_pid_result_pid.unwrap();

    let run_queue_length_after = arc_scheduler.run_queue_len(priority);

    assert_eq!(run_queue_length_after, run_queue_length_before + 1);

    let child_arc_process = pid_to_process(&child_pid_pid).unwrap();

    assert!(arc_scheduler.run_through(&child_arc_process));
    assert!(!arc_scheduler.run_through(&child_arc_process));

    assert_eq!(child_arc_process.code_stack_len(), 1);
    assert_eq!(
        child_arc_process.current_module_function_arity(),
        Some(Arc::new(ModuleFunctionArity {
            module: atom_from!(module),
            function: atom_from!(function),
            arity: 1
        }))
    );

    match *child_arc_process.status.read() {
        Status::Exiting(ref runtime_exception) => {
            assert_eq!(runtime_exception, &badarith!());
        }
        ref status => panic!("Process status ({:?}) is not exiting.", status),
    };

    assert!(parent_arc_process.is_exiting())
}
