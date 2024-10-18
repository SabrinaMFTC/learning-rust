fn main() {
    repeticoes();
}

fn repeticoes() {
    let multiplier: u8 = 5;

    // while
    let mut count: u8 = 0;
    while count < 10 {
        count += 1;
        if count == 5 { continue; }
        println!("{} x {} = {}", multiplier, count, multiplier * count);
    }

    // loop
    count = 0;
    loop {
        count += 1;
        println!("{} x {} = {}", multiplier, count, multiplier * count);
        if count == 10 { break; } 
    }

    // for
    for i in 1..=10 {       // other option: for i in 1..11 
        println!("{} x {} = {}", multiplier, i, multiplier * i);
    }
}