use crossterm::{terminal, ExecutableCommand, QueueableCommand, cursor,
                style::{self, Colorize}, Result,
                event::{self, poll, read, Event, KeyCode, KeyEvent}};
use std::io::{stdout, Write};
use std::time::Duration;
use realestatebg_ml::display::screen::Screen;
use realestatebg_ml::display::layout::SingleMenuLayout;

fn main() -> Result<()> {
    let mut stdout = stdout();
    stdout.execute(
        terminal::EnterAlternateScreen
    )?;

    terminal::enable_raw_mode()?;

    stdout.queue(cursor::Hide)?;
    stdout.queue(cursor::MoveTo(1, 1))?;

    let layout = Box::new(SingleMenuLayout { });
    let mut screen = Screen::new(layout);

    loop {
        let size = terminal::size();

        stdout.flush()?;

        poll_events(&mut screen);
    }

    stdout.execute(cursor::Show);
    stdout.execute(terminal::LeaveAlternateScreen);

    Ok(())
}

fn poll_events(screen: &mut Screen) -> crossterm::Result<()> {
    loop {
        if poll(Duration::from_millis(500))? {
            match read()? {
                Event::Key(event) => (),
                Event::Mouse(_event) => (),
                Event::Resize(width, height) => screen.resize_terminal(width, height),
            }
        }
    }
    Ok(())
}