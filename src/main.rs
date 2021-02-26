use crate::cake_factory::{CakeFactory, StepBalancer, StepDefinition};

mod cake_factory;

#[cfg(test)]
mod tests;

fn main() {
    println!("Application Started !");
    let mut factory = CakeFactory::new(std::time::Duration::new(60, 0));

    let preparing = StepBalancer::new(3, "On Preparing", StepDefinition::new(5, 8));
    let cook = StepBalancer::new(5, "On Cook", StepDefinition::new(10, 10));
    let package = StepBalancer::new(2, "On Package", StepDefinition::new(2, 2));

    factory.add_balancer(preparing);
    factory.add_balancer(cook);
    factory.add_balancer(package);

    factory.run();
    println!("Application Stopped !");
}
