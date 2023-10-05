use cgt_core::TestError;
use cgt_macros::{cgt_assert, cgt_assert_eq, cgt_assert_err, cgt_assert_ok};

#[test]
fn cgt_assert_bool_true() {
    fn test() -> Result<(), TestError> {
        cgt_assert!(true);
        Ok(())
    }

    assert_eq!(test(), Ok(()));
}

#[test]
fn cgt_assert_bool_false() {
    fn test() -> Result<(), TestError> {
        cgt_assert!(false);
        unreachable!()
    }

    assert_eq!(
        test(),
        Err(TestError::ConditionUnmet(String::from("false")))
    );
}

#[test]
fn cgt_assert_expr_true() {
    fn test() -> Result<(), TestError> {
        cgt_assert!(1 < 2);
        Ok(())
    }

    assert_eq!(test(), Ok(()));
}

#[test]
fn cgt_assert_expr_false() {
    fn test() -> Result<(), TestError> {
        cgt_assert!(1 > 2);
        unreachable!()
    }

    assert_eq!(
        test(),
        Err(TestError::ConditionUnmet(String::from("1 > 2")))
    );
}

#[test]
fn cgt_assert_eq_bool_true() {
    fn test() -> Result<(), TestError> {
        cgt_assert_eq!(true, true);
        Ok(())
    }

    assert_eq!(test(), Ok(()));
}

#[test]
fn cgt_assert_eq_bool_false() {
    fn test() -> Result<(), TestError> {
        cgt_assert_eq!(true, false);
        unreachable!()
    }

    assert_eq!(
        test(),
        Err(TestError::NotEqual(
            String::from("true"),
            String::from("false")
        ))
    );
}

#[test]
fn cgt_assert_eq_expr_true() {
    fn test() -> Result<(), TestError> {
        cgt_assert_eq!(1 + 1, 2);
        Ok(())
    }

    assert_eq!(test(), Ok(()));
}

#[test]
fn cgt_assert_eq_expr_false() {
    fn test() -> Result<(), TestError> {
        cgt_assert_eq!(1 + 1, 3);
        unreachable!()
    }

    assert_eq!(
        test(),
        Err(TestError::NotEqual(String::from("2"), String::from("3")))
    );
}

#[test]
fn cgt_assert_ok_true() {
    fn test() -> Result<(), TestError> {
        cgt_assert_ok!(Ok::<(), TestError>(()));
        Ok(())
    }

    assert_eq!(test(), Ok(()));
}

#[test]
fn cgt_assert_ok_false() {
    fn test() -> Result<(), TestError> {
        cgt_assert_ok!(Err::<(), TestError>(TestError::Unspecified));
        Ok(())
    }

    assert_eq!(
        test(),
        Err(TestError::ResultNotOk(String::from(
            "Err(\n    Unspecified,\n)"
        )))
    );
}

#[test]
fn cgt_assert_err_true() {
    fn test() -> Result<(), TestError> {
        cgt_assert_err!(Err::<(), TestError>(TestError::Unspecified));
        Ok(())
    }

    assert_eq!(test(), Ok(()));
}
#[test]
fn cgt_assert_err_false() {
    fn test() -> Result<(), TestError> {
        cgt_assert_err!(Ok::<(), TestError>(()));
        Ok(())
    }

    assert_eq!(
        test(),
        Err(TestError::ResultNotOk(String::from("Ok(\n    (),\n)")))
    );
}
