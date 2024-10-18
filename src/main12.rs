fn main() {
    ownership();
}

fn ownership() {
    let _string = String::from("Vinicius");
    steal(&_string);

    println!("{}", _string);
}

fn steal(string: &String) {
    println!("{}", string);
}
