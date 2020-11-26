use crossterm::{terminal, ExecutableCommand, QueueableCommand, cursor,
                style::{self, style, Color, Colorize}, Result,
                event::{self, poll, read, Event, KeyCode, KeyEvent}};
use std::io::{stdout, Write, BufWriter};
use std::time::Duration;
use realestatebg_ml::display::screen::Screen;
use realestatebg_ml::display::layout::{SingleMenuLayout, GameLayout};
use realestatebg_ml::display::writer::{Writer, Justify};
use spin_sleep::LoopHelper;

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
    let mut writer = Writer::new(Box::new(&stdout));
    let mut loop_helper = LoopHelper::builder()
        .report_interval(Duration::from_secs_f64(1f64 / 60f64))
        .build_with_target_rate(30.0);

    loop {
        let delta = loop_helper.loop_start();
        let viewport = screen.areas.viewport;
        let command = screen.areas.command.unwrap();

        poll_events(&mut screen);

        loop_helper.loop_sleep();
        if poll_events(&mut screen).unwrap() == false {
            break;
        }
    }

    Ok(())
}

fn poll_events(screen: &mut Screen) -> crossterm::Result<bool> {
    if poll(Duration::from_millis(1))? {
        let res = match read()? {
            Event::Key(event) => {
                match event.code {
                    KeyCode::Esc => false,
                    _ => true
                }
            }
            Event::Mouse(_event) => true,
            Event::Resize(width, height) => {
                screen.resize_terminal(width, height);
                true
            }
        };
        Ok(res)
    } else {
        Ok(true)
    }
}