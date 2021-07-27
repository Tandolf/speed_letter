use crossterm::Result;
use speed_letter::SpeedLetter;

mod speed_letter;

fn main() -> Result<()> {
    let letters = vec![
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'x', 'y', 'z',
    ];
    let mut game = SpeedLetter::new(letters);
    game.run()?;
    Ok(())
}
