fn main() {
    ownership();
}

fn ownership() {
    let _string = String::from("Vinicius");
    steal(_string);

    /* println!("{}", _string); --> this will result in the following error: 
    
    error[E0382]: borrow of moved value: `_string`
      --> main10.rs:10:20
       |
       |
    6  |     let _string = String::from("Vinicius");
       |         ------- move occurs because `_string` has type `String`, which does not implement the `Copy` trait
    7  |
    8  |     steal(_string);
       |           ------- value moved here
    9  |
    10 |     println!("{}", _string);
       |                    ^^^^^^^ value borrowed here after move
       |
    note: consider changing this parameter type in function `steal` to borrow instead if owning the value isn't necessary
      --> main10.rs:13:18
       |
    13 | fn steal(string: String) {
       |    -----         ^^^^^^ this parameter takes ownership of the value
       |    |
       |    in this function
       = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
    help: consider cloning the value if the performance cost is acceptable
       |
    8  |     steal(_string.clone());
       |                  ++++++++
       */
}

fn steal(string: String) {
    println!("{}", string);
}