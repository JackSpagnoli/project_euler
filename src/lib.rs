#![feature(iter_next_chunk)]

pub enum Whole128 {
    Signed(i128),
    Unsigned(u128),
}

#[macro_export()]
macro_rules! define_problem {
    ($module:ident, $expected:expr) => {{
        let problem_number = &stringify!($module)[7..10];
        use $module::ans as ans_func;
        let expected_value = $expected;
        if expected_value < 0 {
            let return_function: &dyn Fn() -> Whole128 = &|| Whole128::Signed(ans_func() as i128);
            (
                return_function,
                Whole128::Signed(expected_value as i128),
                problem_number,
            )
        } else {
            let return_function: &dyn Fn() -> Whole128 = &|| Whole128::Unsigned(ans_func() as u128);
            (
                return_function,
                Whole128::Unsigned(expected_value as u128),
                problem_number,
            )
        }
    }};
}

pub mod number_utils;
pub mod primes;
pub mod permutations;

pub mod problem027;
pub mod problem029;
pub mod problem030;
pub mod problem031;
pub mod problem033;
pub mod problem034;
pub mod problem035;
pub mod problem036;
pub mod problem037;
pub mod problem038;
pub mod problem039;
pub mod problem040;
pub mod problem041;
pub mod problem042;
pub mod problem043;
pub mod problem044;
pub mod problem045;
pub mod problem046;
pub mod problem047;
pub mod problem048;
pub mod problem049;
pub mod problem050;
// pub mod problem051;
pub mod problem052;
pub mod problem053;
pub mod problem054;
pub mod problem055;
pub mod problem056;
pub mod problem057;
pub mod problem058;
pub mod problem425;
