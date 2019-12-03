use std::io::{self, BufRead};

fn main() {
    let mut total_module_mass: i64 = 0;
    let mut total_module_fuel_mass: i64 = 0;
    let mut total_module_fuel_fuel_mass: i64 = 0;
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
      let module_mass = line.unwrap().parse::<i64>().unwrap();
      total_module_mass += module_mass;
      let module_fuel_mass = required_fuel(module_mass);
      total_module_fuel_mass += module_fuel_mass;
      let module_fuel_fuel_mass = required_fuel_with_fuel(module_mass);
      total_module_fuel_fuel_mass += module_fuel_fuel_mass;
    }
    println!("Total module mass: {}", total_module_mass);
    println!("Fuel required for modules: {}", total_module_fuel_mass);
    println!("Fuel required for modules and fuel: {}", total_module_fuel_fuel_mass);
}

pub fn required_fuel(payload_mass: i64) -> i64 {
    match (payload_mass / 3) - 2 {
      f if f <= 0 => 0,
      f => f,
    }
}

pub fn required_fuel_with_fuel(payload_mass: i64) -> i64 {
    match required_fuel(payload_mass) {
        0 => 0,
        fuel_mass => fuel_mass + required_fuel_with_fuel(fuel_mass),
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_required_fuel() {
        assert_eq!(required_fuel(12), 2);
        assert_eq!(required_fuel(14), 2);
        assert_eq!(required_fuel(1969), 654);
        assert_eq!(required_fuel(100756), 33583);
    }


    #[test]
    fn test_required_fuel_with_fuel() {
        assert_eq!(required_fuel_with_fuel(14), 2);
        assert_eq!(required_fuel_with_fuel(1969), 966);
        assert_eq!(required_fuel_with_fuel(100756), 50346);
    }
}

