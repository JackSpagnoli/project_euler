use std::fs;

pub fn ans() -> u128 {
    let contents = fs::read_to_string("resources/problem042/words.txt").unwrap();
    let words = contents.split(",");
    let mut num_triangle_words:u16 = 0;
    for word in words {
        if is_triangle_word(word) { num_triangle_words+=1; }
    }
    return num_triangle_words as u128;
}
fn is_triangle_word(word:&str)->bool{
    let word_value = word_value(word);
    let discriminant = 1+(8*word_value);

    //Tests satisfactory conditions for word_value = (1/2)n(n+1) being solved by natural n
    if discriminant < 9 { return false; }
    let (is_square_root, root) = is_square(discriminant);
    if !is_square_root { return false; }

    // println!("Found {} is a triangle word, with value {}", word, word_value);

    return (root-1)%2 == 0;
}
fn word_value(word:&str)->u16{
    let mut chars = word.chars();
    let mut current = chars.next();
    let mut value:u16=0;
    while current.is_some() {
        if current.unwrap()!='\"' {
            value += current.unwrap() as u16 - 64;
        }
        current = chars.next();
    }
    return value;
}
fn is_square(a:u16)->(bool,u16){
    let mut guess:u16 = 1;
    while guess*guess*4 < a {
        guess*=2;
    }
    while guess*guess<=a{
        if guess*guess==a { return (true,guess); }
        guess+=1;
    }
    return (false,0);
}