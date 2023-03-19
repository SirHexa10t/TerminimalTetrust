mod animation;
mod params;
mod session;

use animation::AnimationMechanics;
use params::AnimationParams;
use crossterm::terminal::size;
use crate::params::Constraints;


fn approval_terminal_size() -> Result<(), String> {
    let params = Constraints::default();
    let (width, height) = size().map_err(|e| format!("Failed to get terminal size: {}", e))?;

    if width < params.required_width || height < params.required_height {
        println!(
            "Terminal is too small! Expected height X width of at-least {}x{}, but got {}x{}.",
            params.required_width, params.required_height, width, height
        );
        std::process::exit(1);

    }

    Ok(())
}


fn main() -> crossterm::Result<()> {
    approval_terminal_size().expect("Failed terminal size check");

    let params = AnimationParams::default();
    let mut animation = AnimationMechanics::new(&params)?;
    animation.run(&params)?;

    Ok(())
}