use super::StepDefinition;

pub struct StepBalancer {
    pub max_concurrent_working: i32,
    pub balancing_action_name: String,
    pub step_definition: StepDefinition,
    pub on_element_complete: Box<()>,
}

impl StepBalancer {
    pub fn new(
        max_concurrent_working: i32,
        balancing_action_name: &str,
        step_definition: StepDefinition,
    ) -> StepBalancer {
        return StepBalancer {
            max_concurrent_working,
            balancing_action_name: balancing_action_name.to_string(),
            step_definition,
            on_element_complete: Box::new(()),
        };
    }

    pub fn set_element_complete(&mut self, new_function: Box<()>) {
        self.on_element_complete = new_function;
    }

    pub fn produce(&self) {
        println!("produce {}", self.balancing_action_name);
    }
}
