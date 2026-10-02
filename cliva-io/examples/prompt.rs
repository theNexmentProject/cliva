use cliva_io::input::{InputType, InputValue, confirm, prompt};

fn main() -> std::io::Result<()> {
    let name = prompt("Enter your name", InputType::Str, Some("Shisui"))?;

    let age = prompt("Enter your age", InputType::Int, Some("16"))?;

    let score = prompt("Enter your score", InputType::Float, Some("9.5"))?;

    if let InputValue::Str(value) = name {
        println!("Name: {value}");
    }

    if let InputValue::Int(value) = age {
        println!("Age: {value}");
    }

    if let InputValue::Float(value) = score {
        println!("Score: {value}");
    }

    let _accepted = confirm("Continue", Some(true))?;
    let _overwrite = confirm("Overwrite existing file", Some(false))?;
    let _proceed = confirm("Continue installation", None)?;

    Ok(())
}
