pub fn main() {
    if !(true) {
        return cgt_core::TestResult::Failure(
            cgt_core::TestError::ConditionUnmet("true".to_string()),
        );
    }
}
