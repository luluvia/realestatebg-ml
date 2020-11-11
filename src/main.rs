use crossterm::{terminal, ExecutableCommand, QueueableCommand, cursor,
                style::{self, Colorize}, Result,
                event::{self, poll, read, Event, KeyCode, KeyEvent}};
use std::io::{stdout, Write};
use display::Screen;
use std::time::Duration;

fn main() -> Result<()> {
    let mut stdout = stdout();
    stdout.execute(
        terminal::EnterAlternateScreen
    )?;

    terminal::enable_raw_mode()?;

    stdout.queue(cursor::Hide)?;
    stdout.queue(cursor::MoveTo(1, 1))?;

    let mut screen = Screen::new(LAYOUT);

    loop {
        let duration = start.elapsed();

        let size = terminal::size();

        stdout.flush()?;

        poll_events();
    }

    stdout.execute(cursor::Show);
    stdout.execute(terminal::LeaveAlternateScreen);

    Ok(())
}

fn poll_events() -> crossterm::Result<()> {
    loop {
        if poll(Duration::from_millis(500))? {
            match read()? {
                Event::Key(event) => (),
                Event::Mouse(_event) => (),
                Event::Resize(width, height) => Screen::resize_terminal(width, height),
            }
        }
    }
    Ok(())
}