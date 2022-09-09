fn main() {
    let mut pentagonal_numbers:Vec<u128> = vec![1,5,12,22,35,51,70,92,117,145];
    generate_pentagonals(&mut pentagonal_numbers, 500_000_000);
    println!("Generated initial pentagonal numbers, {} found", pentagonal_numbers.len());
    let mut d:u128 = 1_000_000_000;
    let mut i:usize=0;
    let max = pentagonal_numbers.len();
    let limit = pentagonal_numbers[max-1] - pentagonal_numbers[max-2];
    while i < max-2{
        println!("i={}", i);
        let mut j:usize = i+1;
        while j < max-1 {
            let diff = pentagonal_numbers[j] - pentagonal_numbers[i];
            if diff < d {
                if pentagonal_numbers.contains(&diff) {
                    let sum = pentagonal_numbers[j] + pentagonal_numbers[i];
                    generate_pentagonals(&mut pentagonal_numbers, sum);
                    if pentagonal_numbers.contains(&sum) {
                        d = diff;
                        if limit > d {
                            println!("Possible limit found.");
                            j=max-2;
                            i=max-3;
                        }
                    }
                }
            }
            j+=1;
        }
        i+=1;
    }
    println!("{}", d);
}
fn generate_pentagonals(pentagonal_numbers:&mut Vec<u128>, limit:u128){
    while pentagonal_numbers[pentagonal_numbers.len()-1] < limit {
        let n:u128 = pentagonal_numbers.len() as u128;
        pentagonal_numbers.push((n*((3*n)-1))/2);
    }
}