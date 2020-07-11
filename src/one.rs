use crate::util;

pub fn one_a() -> i32 {
    let masses = util::parse_ints_from_file("src/inputs/1.txt");
    masses.iter().map(|x| fuel_for_module_one_step(*x)).sum()
}

pub fn one_b() -> i32 {
    let masses = util::parse_ints_from_file("src/inputs/1.txt");
    masses.iter().map(|x| fuel_for_module(*x)).sum()
}

/// Performs one step of the fuel calculation algorithm for a given mass.
///
/// "Fuel required to launch a given module is based on its mass. Specifically, to
/// find the fuel required for a module, take its mass, divide by three, round
/// down, and subtract 2."
fn fuel_for_module_one_step(mass: i32) -> i32 {
    let divided = mass as f32 / 3.0;
    divided.trunc() as i32 - 2
}

/// Calculates fuel for a given mass.
///
/// "Fuel itself requires fuel just like a module - take its mass, divide by
/// three, round down, and subtract 2. However, that fuel also requires fuel, and
/// that fuel requires fuel, and so on."
fn fuel_for_module(mass: i32) -> i32 {
    let step_output = fuel_for_module_one_step(mass);

    if step_output <= 0 {
        0
    } else {
        step_output + fuel_for_module(step_output)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_fuel_for_module_one_step() {
        assert_eq!(fuel_for_module_one_step(12), 2);
        assert_eq!(fuel_for_module_one_step(14), 2);
        assert_eq!(fuel_for_module_one_step(1969), 654);
        assert_eq!(fuel_for_module_one_step(100756), 33583);
    }

    #[test]
    fn test_fuel_for_module() {
        assert_eq!(fuel_for_module(14), 2);
        assert_eq!(fuel_for_module(1969), 966);
        assert_eq!(fuel_for_module(100756), 50346);
    }

    #[test]
    fn test_solutions() {
        assert_eq!(one_a(), 3334297);
        assert_eq!(one_b(), 4998565);
    }
}
