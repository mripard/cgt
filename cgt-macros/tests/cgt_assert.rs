use cgt_core::{TestError, TestResult};
use cgt_macros::{cgt_assert, cgt_assert_eq, cgt_assert_err, cgt_assert_ok};

#[test]
fn cgt_assert_bool_true() {
    fn test() -> TestResult {
        cgt_assert!(true);
        TestResult::Success
    }

    assert_eq!(test(), TestResult::Success);
}

#[test]
fn cgt_assert_bool_false() {
    fn test() -> TestResult {
        cgt_assert!(false);
        unreachable!()
    }

    assert_eq!(
        test(),
        TestResult::Failure(TestError::ConditionUnmet(String::from("false")))
    );
}

#[test]
fn cgt_assert_expr_true() {
    fn test() -> TestResult {
        cgt_assert!(1 < 2);
        TestResult::Success
    }

    assert_eq!(test(), TestResult::Success);
}

#[test]
fn cgt_assert_expr_false() {
    fn test() -> TestResult {
        cgt_assert!(1 > 2);
        unreachable!()
    }

    assert_eq!(
        test(),
        TestResult::Failure(TestError::ConditionUnmet(String::from("1 > 2")))
    );
}

#[test]
fn cgt_assert_eq_bool_true() {
    fn test() -> TestResult {
        cgt_assert_eq!(true, true);
        TestResult::Success
    }

    assert_eq!(test(), TestResult::Success);
}

#[test]
fn cgt_assert_eq_bool_false() {
    fn test() -> TestResult {
        cgt_assert_eq!(true, false);
        unreachable!()
    }

    assert_eq!(
        test(),
        TestResult::Failure(TestError::NotEqual(
            String::from("true"),
            String::from("false")
        ))
    );
}

#[test]
fn cgt_assert_eq_expr_true() {
    fn test() -> TestResult {
        cgt_assert_eq!(1 + 1, 2);
        TestResult::Success
    }

    assert_eq!(test(), TestResult::Success);
}

#[test]
fn cgt_assert_eq_expr_false() {
    fn test() -> TestResult {
        cgt_assert_eq!(1 + 1, 3);
        unreachable!()
    }

    assert_eq!(
        test(),
        TestResult::Failure(TestError::NotEqual(String::from("2"), String::from("3")))
    );
}

#[test]
fn cgt_assert_ok_true() {
    fn test() -> TestResult {
        cgt_assert_ok!(Ok::<(), TestError>(()));
        TestResult::Success
    }

    assert_eq!(test(), TestResult::Success);
}

#[test]
fn cgt_assert_ok_false() {
    fn test() -> TestResult {
        cgt_assert_ok!(Err::<(), TestError>(TestError::Unspecified));
        TestResult::Success
    }

    assert_eq!(
        test(),
        TestResult::Failure(TestError::ResultNotOk(String::from(
            "Err(\n    Unspecified,\n)"
        )))
    );
}

#[test]
fn cgt_assert_err_true() {
    fn test() -> TestResult {
        cgt_assert_err!(Err::<(), TestError>(TestError::Unspecified));
        TestResult::Success
    }

    assert_eq!(test(), TestResult::Success);
}
#[test]
fn cgt_assert_err_false() {
    fn test() -> TestResult {
        cgt_assert_err!(Ok::<(), TestError>(()));
        TestResult::Success
    }

    assert_eq!(
        test(),
        TestResult::Failure(TestError::ResultNotOk(String::from("Ok(\n    (),\n)")))
    );
}
