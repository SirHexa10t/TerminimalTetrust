use std::{thread, time};
use std::io::Stdout;
use std::sync::atomic::Ordering;
use crossterm::{execute, cursor, terminal, Result};
use crossterm::style::{Color, Print, ResetColor, SetForegroundColor};
use std::time::Instant;
use crate::{AnimationParams, session};


pub struct AnimationMechanics {
    y: u16,
    direction: i16,
    stdout: Stdout,
    iter_duration: time::Duration,
}

impl AnimationMechanics {
    pub fn new(params: &AnimationParams) -> Result<Self> {
        let mut stdout = std::io::stdout();
        execute!(stdout, terminal::EnterAlternateScreen, cursor::Hide)?;
        Ok(Self {
            y: params.starting_y_pos,
            direction: params.direction,
            stdout,
            iter_duration: time::Duration::from_secs_f32(1.0 / params.frame_rate as f32)
        })
    }

    pub fn run(&mut self, params: &AnimationParams) -> Result<()> {
        let inputs = session::UserInput::default();

        loop {
            if inputs.sigterm.load(Ordering::SeqCst) {
                println!("received sigterm!");
                execute!(self.stdout, terminal::LeaveAlternateScreen)?;
                std::process::exit(0);
            }

            let start_time = Instant::now();

            self.animate_frame(params)?;

            self.sleep_remaining_time(start_time)
        }
    }


    fn sleep_remaining_time(&mut self, start_time: Instant) {
        // Wait before rendering the next frame. Make each iteration take the same amount of time
        let elapsed_time = start_time.elapsed();
        if elapsed_time < self.iter_duration {
            let remaining = self.iter_duration - elapsed_time;
            thread::sleep(remaining);
        }
    }

    fn animate_frame(&mut self, _params: &AnimationParams) -> Result<()> {
        // Clear the screen
        execute!(self.stdout, terminal::Clear(terminal::ClearType::All))?;

        // Print the "X" character at the current position
        execute!(self.stdout, cursor::MoveTo(0, self.y), SetForegroundColor(Color::Red), Print("X"), ResetColor)?;

        // Update the position for the next frame
        self.y = (self.y as i16 + self.direction) as u16;
        if self.y == 0 || self.y == 9 {
            self.direction *= -1;
        }

        Ok(())
    }
}
