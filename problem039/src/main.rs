fn main() {
    let mut p: u32 = 1;
    let mut max_p = 0;
    let mut max_solutions = 0;
    while p <= 1000 {
        let mut solutions: u32 = 0;
        let mut a: u32 = 1;
        let max_a = (p + (3 - (p % 3))) / 3;
        while a < max_a {
            let mut b = a;
            let max_b = p - a;
            while b <= max_b {
                let perfect_square = is_perfect_square(a, b, p);
                if perfect_square {
                    solutions += 1;
                }
                b += 1;
            }
            a += 1;
        }
        if solutions > max_solutions {
            max_solutions = solutions;
            max_p = p;
        }
        p += 1;
    }
    println!("Max solutions: {} achieved with p={}", max_solutions, max_p);
}

fn is_perfect_square(a: u32, b: u32, p: u32) -> bool {
    let c = p - a - b;
    if (a * a) + (b * b) != (c * c) { return false; }
    return true;
}