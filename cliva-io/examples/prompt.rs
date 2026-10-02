use cliva_io::input::{confirm, password, prompt, InputType, InputValue};

fn main() -> std::io::Result<()> {
    // Ask for a name with a default text value.
    let name = prompt("Enter your name :", InputType::Str, Some("Shisui"))?;
    if let InputValue::Str(value) = name {
        println!("Name: {value}");
    }

    // Ask for an age with a default integer value.
    let age = prompt("Enter your age :", InputType::Int, Some("16"))?;
    if let InputValue::Int(value) = age {
        println!("Age: {value}");
    }

    // Ask for a score with a default floating-point value.
    let score = prompt("Enter your score :", InputType::Float, Some("9.5"))?;
    if let InputValue::Float(value) = score {
        println!("Score: {value}");
    }

    // Ask the user to confirm with yes or no.
    let accepted = confirm("Do you want to continue", Some(true))?;
    println!("Continue: {accepted}");

    // Ask for confirmation without providing a default.
    let overwrite = confirm("Overwrite existing file", None)?;
    println!("Overwrite: {overwrite}");

    // Ask for a password while masking typed characters.
    let secret = password("Enter your password :", Some("ansh123"))?;
    println!("Password: {secret}");

    Ok(())
}
