use std::io::Stdout;
use std::{
    io::{stdin, stdout, Read, Write},
    time,
};

use crossterm::cursor::{Hide, MoveTo, MoveToNextLine};
use crossterm::style::{Attribute, Print, SetAttribute};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType, SetSize};
use crossterm::{execute, queue, Result};
use rand::prelude::Rng;

pub struct SpeedLetter {
    letters: Vec<char>,
    stdout: Stdout,
}

impl SpeedLetter {
    pub fn new(letters: Vec<char>) -> Self {
        let stdout = stdout();
        SpeedLetter { letters, stdout }
    }

    pub fn run(&mut self) -> Result<()> {
        let (cols, rows) = size()?;
        execute!(
            self.stdout,
            SetSize(cols, rows),
            Clear(ClearType::All),
            Hide
        )?;

        self.print_menu()?;

        enable_raw_mode()?;
        let mut buffer = [0; 1];
        stdin().read(&mut buffer)?;
        disable_raw_mode()?;

        let start = time::Instant::now();
        let deadline = 3;
        let mut old_remain = 0;
        loop {
            let elapsed = start.elapsed().as_secs();
            if deadline <= elapsed {
                break;
            }

            let new_remain = deadline - elapsed;
            if old_remain != new_remain {
                execute!(
                    self.stdout,
                    Clear(ClearType::CurrentLine),
                    MoveTo(25, 15),
                    Print(format!("{}", new_remain))
                )?;
                old_remain = new_remain;
            }
        }

        let mut rng = rand::thread_rng();
        let random_index = rng.gen_range(0..self.letters.len());
        let secret_letter = self.letters[random_index];
        let start = time::Instant::now();

        execute!(
            self.stdout,
            Clear(ClearType::CurrentLine),
            MoveTo(25, 15),
            Print(format!("{}", secret_letter))
        )?;

        enable_raw_mode()?;

        let mut buffer = [0; 1];
        while let Ok(_) = stdin().read(&mut buffer) {
            disable_raw_mode()?;
            let input = buffer[0] as char;

            if input == secret_letter {
                execute!(
                    self.stdout,
                    Clear(ClearType::CurrentLine),
                    MoveTo(25, 15),
                    Print("▒"),
                    MoveTo(22, 17),
                    Print("CORRECT!"),
                    MoveTo(13, 18),
                    Print(format!("Time taken: {} sec", start.elapsed().as_secs_f64())),
                    MoveToNextLine(5),
                    Print(""),
                )?;
                break;
            } else {
                execute!(
                    self.stdout,
                    Clear(ClearType::CurrentLine),
                    MoveTo(25, 15),
                    Print("X"),
                    MoveTo(21, 17),
                    Print("INCORRECT!"),
                    MoveTo(15, 18),
                    Print("Better luck next time!"),
                    MoveToNextLine(2),
                    Print(""),
                )?;
                break;
            }
        }
        Ok(())
    }

    fn print_menu(&mut self) -> Result<()> {
        queue!(
            self.stdout,
            MoveTo(14, 5),
            Print("----- SPEED LETTER -----"),
            MoveTo(3, 7),
            Print("Goal of this game is to, as fast as you can, type"),
            MoveTo(3, 8),
            Print("the random letter between a-z that will appear"),
            MoveTo(3, 9),
            Print("on the screen."),
            MoveTo(8, 11),
            Print("The faster you are, the cooler you are."),
            MoveTo(14, 13),
            Print("------ GOOD LUCK! ------"),
            MoveTo(7, 15),
            SetAttribute(Attribute::SlowBlink),
            Print("Press a key to start the 3 sec countdown"),
            SetAttribute(Attribute::Reset)
        )?;

        self.stdout.flush()?;

        Ok(())
    }
}
