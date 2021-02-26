use super::StepBalancer;
use chrono::Local;

pub struct CakeFactory {
    pub created_cake_count: i32,
    interval_duration_in_seconds: u64,
    balancers: Vec<StepBalancer>,
}

impl CakeFactory {
    pub fn new(interval_duration_in_seconds: std::time::Duration) -> CakeFactory {
        return CakeFactory {
            created_cake_count: 0,
            interval_duration_in_seconds: interval_duration_in_seconds.as_secs(),
            balancers: Vec::new(),
        };
    }

    fn private_get_status_message(created_cake_count: i32) -> String {
        let mut result = "".to_owned();
        result = format!(
            "--- Actual Status {} ---\n",
            Local::now().format("%m/%d/%Y %H:%M:%S")
        );
        result = format!("{}Finished cakes: {}\n", result, created_cake_count);
        // loop balancers //
        result += "-----------------------------------------";
        return result.to_owned();
    }

    pub fn get_status_message(&self) -> String {
        return CakeFactory::private_get_status_message(self.created_cake_count);
    }

    pub fn add_balancer(&mut self, balancer: StepBalancer) {
        if self.balancers.len() >= 1 {
            let len = self.balancers.len() - 1;
            let callback = Box::new(balancer.produce());
            self.balancers[len].set_element_complete(callback)
        }

        self.balancers.push(balancer);
    }

    pub fn run(&self) {}
}
