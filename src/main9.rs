fn main() {
    let language = "Kotlin";
    let purpose = match language {
        "PHP" => "Web",
        "Kotlin" => "Android",
        "Python" => "Data Science",
        _ => "Unknown"
    };

    println!("The purpose of {} is {}.", language, purpose)
}