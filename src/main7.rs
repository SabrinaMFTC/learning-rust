fn main() {
    let age: u8 = 16;
    let authorized = true;
    let majority = age >= 18;

    if majority {
        println!("You can enter the club.");
    } else if age >= 16 && authorized {
        println!("You can enter the club, since you are at least 16 and authorized by your legal guardian.")
    } else {
        println!("You can't enter the club.");
    }

    let condition = if majority { "adult" } else { "minor" };

    println!("You are a(n) {}.", condition);
}