use cliva_io::input::{checkbox_selection, inline_selection, vertical_selection};

// Tests inline selection with left and right arrow keys.
fn test_inline() -> std::io::Result<()> {
    let framework = inline_selection("Choose framework", vec!["Next.js", "React", "Vue"], Some(0))?;

    println!("Selected: {}", framework);

    Ok(())
}

// Tests vertical selection with up and down arrow keys.
fn test_vertical() -> std::io::Result<()> {
    let framework = vertical_selection(
        "Choose framework",
        vec!["Next.js", "React", "Vue"],
        Some(1),
        None,
    )?;

    println!("Selected: {}", framework);

    Ok(())
}

// Tests checkbox selection with arrow keys and the spacebar.
fn test_checkbox() -> std::io::Result<()> {
    let frameworks = checkbox_selection(
        "Choose frameworks",
        vec!["Next.js", "React", "Vue"],
        Some(vec![0]),
        None,
    )?;

    println!("Selected frameworks:");

    for framework in frameworks {
        println!("- {}", framework);
    }

    Ok(())
}

// Runs all three selection examples one after another.
fn main() -> std::io::Result<()> {
    test_inline()?;
    test_vertical()?;
    test_checkbox()?;

    Ok(())
}
