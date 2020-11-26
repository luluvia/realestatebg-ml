use crossterm::{terminal, ExecutableCommand, QueueableCommand, cursor,
                style::{style, Color}, Result,
                event::{poll, read, Event, KeyCode}};
use std::io::{stdout, BufWriter};
use std::time::Duration;
use realestatebg_ml::display::screen::Screen;
use realestatebg_ml::display::layout::{SingleMenuLayout, GameLayout};
use realestatebg_ml::display::writer::{Writer, Justify};
use spin_sleep::LoopHelper;

fn main() -> Result<()> {
    let mut stream = BufWriter::new(stdout());
    stream.execute(terminal::EnterAlternateScreen)?;

    terminal::enable_raw_mode()?;

    let layout = Box::new(GameLayout { });
    let mut screen = Screen::new(layout, Writer::new(Box::new(&mut stream)));
    screen.writer.reset(true);

    let mut loop_helper = LoopHelper::builder()
        .report_interval(Duration::from_secs_f64(1f64 / 60f64))
        .build_with_target_rate(30.0);
    let mut alternator = false;

    loop {
        let delta = loop_helper.loop_start();

        screen.writer.reset(false);

        let viewport = screen.areas.viewport;
        let command = screen.areas.command.unwrap();
        let status = screen.areas.status.unwrap();
        let logs = screen.areas.logs.unwrap();

        if alternator {
            alternator = false;
            screen.writer.draw_uniform_rect(viewport.full_area(), style("&").with(Color::Blue), 0);
        } else {
            alternator = true;
            screen.writer.draw_uniform_rect(viewport.full_area(), style("&").with(Color::Blue), 0);
        }

        let (terminal_size_x, terminal_size_y) = terminal::size().unwrap();
        let (viewport_left, viewport_top, viewport_size_x, viewport_size_y) = viewport.full_area();
        screen.writer.write_styled_text(
            style(format!("[DEBUG] Screen size: {}, {}; Terminal size: {}, {}, Viewport size: {}, {}", screen.area.width, screen.area.height, terminal_size_x, terminal_size_y, viewport_size_x, viewport_size_y).as_str())
                .with(Color::Yellow),
            Justify::CENTER, logs.get_coords_from_percent(0.5, 0.9));
        screen.writer.flush();

        if poll_events(&mut screen).unwrap() == false {
            break;
        }
        loop_helper.loop_sleep();
    }
    drop(screen);

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
                // workaround because Windows shells have a buffer size that is returned, may
                // be fixed in future crossterm update
                let (term_width, term_height) = terminal::size().unwrap();
                if term_width != screen.area.width || term_height != screen.area.height {
                    screen.resize_terminal(term_width, term_height);
                }
                true
            }
        };
        Ok(res)
    } else {
        Ok(true)
    }
}