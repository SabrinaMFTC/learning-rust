fn main() {
    ownership();
}

fn ownership() {
    let string1 = String::from("Vinicius");
    let string2 = steal(string1);

    println!("{}", string2);
}

fn steal(string: String) -> String {
    println!("{}", string);
    
    string
}