use std::time::Instant;

use project_euler::*;

fn main() {
    let problems: Vec<(&dyn Fn() -> Whole128, Whole128, &str)> = vec![
        define_problem!(problem027, -59231),
        define_problem!(problem029, 9183),
        define_problem!(problem030, 443839),
        define_problem!(problem031, 73682),
        // define_problem!(problem033, 100),
        define_problem!(problem034, 40730),
        define_problem!(problem035, 55),
        define_problem!(problem036, 872187),
        define_problem!(problem037, 748317),
        define_problem!(problem038, 932718654),
        define_problem!(problem039, 840),
        define_problem!(problem040, 210),
        define_problem!(problem041, 7652413),
        define_problem!(problem042, 162),
        define_problem!(problem043, 16695334890u128),
        define_problem!(problem044, 5482660),
        define_problem!(problem045, 1533776805),
        define_problem!(problem046, 5777),
        define_problem!(problem047, 134043),
        define_problem!(problem048, 9110846700u128),
        define_problem!(problem049, 296962999629u128),
        define_problem!(problem050, 997651),
        // define_problem!(problem051, 121313),
        define_problem!(problem052, 142857),
        define_problem!(problem053, 4075),
        define_problem!(problem054, 376),
        define_problem!(problem055, 249),
        define_problem!(problem056, 972),
        define_problem!(problem057, 153),
    ];

    problems.iter().for_each(check_problem)
}

fn check_problem(problem: &(&dyn Fn() -> Whole128, Whole128, &str)) {
    let problem_function = problem.0;
    let expected = &problem.1;
    let problem_name = problem.2;

    let now = Instant::now();

    let actual_result = problem_function();

    let duration = now.elapsed().as_secs_f32();
    println!("Problem {problem_name} took {duration} seconds");

    match (actual_result, expected) {
        (Whole128::Signed(actual), Whole128::Signed(expected)) => assert_eq!(actual, *expected),
        (Whole128::Unsigned(actual), Whole128::Unsigned(expected)) => assert_eq!(actual, *expected),
        _ => panic!("Expected return type does not match actual return type"),
    }
}
