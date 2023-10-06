pub fn main() {
    if !(true) {
        return Err(TestError::ConditionUnmet("true".to_string()));
    }
}
