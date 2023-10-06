pub fn main() {
    if !(1 > 2) {
        return Err(TestError::ConditionUnmet("1 > 2".to_string()));
    }
}
