pub struct StepDefinition {
    pub minimal_time_processing: i32,
    pub maximal_time_processing: i32,
}

impl StepDefinition {
    pub fn new(minimal_time_processing: i32, maximal_time_processing: i32) -> StepDefinition {
        return StepDefinition {
            minimal_time_processing,
            maximal_time_processing,
        };
    }
}
