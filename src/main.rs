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
    let mut stream = BufWriter::new(stdout());
    stream.execute(terminal::EnterAlternateScreen)?;

    terminal::enable_raw_mode()?;

    stream.queue(cursor::Hide)?;
    stream.queue(cursor::MoveTo(1, 1))?;

    let layout = Box::new(GameLayout { });
    let mut screen = Screen::new(layout);
    let mut writer = Writer::new(Box::new(&mut stream));
    let mut loop_helper = LoopHelper::builder()
        .report_interval(Duration::from_secs_f64(1f64 / 60f64))
        .build_with_target_rate(3.0);
    let mut alternator = false;

    loop {
        let delta = loop_helper.loop_start();
        let viewport = screen.areas.viewport;
        let command = screen.areas.command.unwrap();
        writer.clear();
        if alternator {
            alternator = false;
            writer.draw_uniform_rect(viewport.full_area(), style("&").with(Color::Blue), 0);
        } else {
            alternator = true;
            writer.draw_uniform_rect(viewport.full_area(), style("&").with(Color::Green), 0);
        }
        writer.flush();

        if poll_events(&mut screen).unwrap() == false {
            break;
        }
        loop_helper.loop_sleep();
    }
    drop(writer);

    stream.execute(cursor::Show);
    stream.execute(terminal::LeaveAlternateScreen);

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