pub fn ans() -> u128 {
    return find_combinations(200) as u128;
}

fn find_combinations(n: u32) -> u32 {
    let coin_value: u32 = 200;
    if n == 0 { return 1; }
    let mut combinations: u32 = 0;
    let mut temp: u32 = n;
    while temp >= coin_value {
        combinations += find_combinations1(temp);
        temp -= coin_value;
    }
    combinations += find_combinations1(temp);
    return combinations;
}

fn find_combinations1(n: u32) -> u32 {
    let coin_value: u32 = 100;
    if n == 0 { return 1; }
    let mut combinations: u32 = 0;
    let mut temp: u32 = n;
    while temp >= coin_value {
        combinations += find_combinations2(temp);
        temp -= coin_value;
    }
    combinations += find_combinations2(temp);
    return combinations;
}

fn find_combinations2(n: u32) -> u32 {
    let coin_value: u32 = 50;
    if n == 0 { return 1; }
    let mut combinations: u32 = 0;
    let mut temp: u32 = n;
    while temp >= coin_value {
        combinations += find_combinations3(temp);
        temp -= coin_value;
    }
    combinations += find_combinations3(temp);
    return combinations;
}

fn find_combinations3(n: u32) -> u32 {
    let coin_value: u32 = 20;
    if n == 0 { return 1; }
    let mut combinations: u32 = 0;
    let mut temp: u32 = n;
    while temp >= coin_value {
        combinations += find_combinations4(temp);
        temp -= coin_value;
    }
    combinations += find_combinations4(temp);
    return combinations;
}

fn find_combinations4(n: u32) -> u32 {
    let coin_value: u32 = 10;
    if n == 0 { return 1; }
    let mut combinations: u32 = 0;
    let mut temp: u32 = n;
    while temp >= coin_value {
        combinations += find_combinations5(temp);
        temp -= coin_value;
    }
    combinations += find_combinations5(temp);
    return combinations;
}

fn find_combinations5(n: u32) -> u32 {
    let coin_value: u32 = 5;
    if n == 0 { return 1; }
    let mut combinations: u32 = 0;
    let mut temp: u32 = n;
    while temp >= coin_value {
        combinations += find_combinations6(temp);
        temp -= coin_value;
    }
    combinations += find_combinations6(temp);
    return combinations;
}

fn find_combinations6(n: u32) -> u32 {
    let coin_value: u32 = 2;
    if n == 0 { return 1; }
    let mut combinations: u32 = 0;
    let mut temp: u32 = n;
    while temp >= coin_value {
        combinations += 1;
        temp -= coin_value;
    }
    combinations += 1;
    return combinations;
}