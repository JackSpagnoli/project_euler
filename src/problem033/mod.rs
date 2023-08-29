pub fn ans() -> u128 {
    let mut fractions: Vec<(u8, u8)> = vec![(0, 0)];

    let mut c: u8 = 1;
    while c < 10 {
        let mut d: u8 = 0;
        while d < 10 {
            let mut a: u8 = 1;
            while a <= c {
                let mut b: u8 = 0;
                while (10 * a) + b < (10 * c) + d {
                    if b == c
                        && d > 0
                        && a as u16 * (10 * c as u16 + d as u16)
                            == d as u16 * (10 * a as u16 + b as u16)
                    {
                        fractions.push(((10 * a) + b, (10 * c) + d));
                    }
                    b += 1;
                }
                a += 1;
            }
            d += 1;
        }
        c += 1;
    }

    for fraction in fractions {
        println!("{} / {}", fraction.0, fraction.1);
    }
    //TODO return
    0
}
