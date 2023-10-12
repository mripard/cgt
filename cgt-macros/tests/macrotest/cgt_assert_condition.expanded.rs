pub fn main() {
    if !(1 > 2) {
        return cgt_core::TestResult::Failure(
            cgt_core::TestError::ConditionUnmet("1 > 2".to_string()),
        );
    }
}
