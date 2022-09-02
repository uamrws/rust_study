use anyhow::Result;
use std::fs::read_to_string;
fn main() -> Result<()> {
    let html = render()?;
    println!("{}", html);
    Ok(())
}
fn render() -> Result<String> {
    let file = std::env::var("MARKDOWN")?;
    let source = read_to_string(file)?;
    Ok(source)
}
