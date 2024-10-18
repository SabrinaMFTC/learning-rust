fn main() {
    ownership();
}

fn ownership() {
    let mut _string = String::from("Vinicius");
    steal(&mut _string);

    println!("{}", _string);
}

fn steal(string: &mut String) {
    string.push_str(" Dias");
    println!("{}", string);
}
