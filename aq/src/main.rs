#[cfg(target_os = "macos")]
mod macos;

#[cfg(target_os = "macos")]
pub fn main() -> Result<(), i32> {
    macos::main()
}

#[cfg(not(target_os = "macos"))]
pub fn main() -> Result<(), i32> {
    println!("This program is only supported on macOS.");
    Ok(())
}
