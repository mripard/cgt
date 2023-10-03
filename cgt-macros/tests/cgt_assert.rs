use cgt_core::TestError;
use cgt_macros::cgt_assert;

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
