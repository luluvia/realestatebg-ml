use std::fmt::Display;
use std::io::Write;
use std::cmp::min;
use crossterm::{style, cursor, QueueableCommand, ErrorKind};

pub struct Writer<'a> {
    out: Box<dyn Write + 'a>,
}

impl<'a> Writer<'a> {
    pub fn new(out: Box<dyn Write + 'a>) -> Writer<'a> {
        Writer { out }
    }
    pub fn draw_uniform_rect(&mut self, top: u16, left: u16, width: u16, height: u16,
                             char: impl Display, border_size: u8) {
        let available_frames = min((width as f32 / 2.0).ceil() as u16, (height as f32 / 2.0).ceil() as u16);
        let frames_to_fill;
        if border_size > 0 {
            frames_to_fill = min(available_frames, border_size as u16);
        } else {
            frames_to_fill = available_frames;
        }
        let bottom = top + height - 1;
        let right = left + width - 1;
        for x in left..(right + 1) {
            self.write_char(&char, x, top);
            self.write_char(&char, x, bottom);
        }
        for y in (top + 1)..bottom {
            self.write_char(&char, left, y);
            self.write_char(&char, right, y);
        }
        if frames_to_fill > 1 {
            self.draw_uniform_rect(top + 1, left + 1, width - 2, height - 2, char, (frames_to_fill - 1) as u8);
        } else if frames_to_fill == 0 {
            self.draw_uniform_rect(top + 1, left + 1, width - 2, height - 2, char, frames_to_fill as u8);
        }

        self.out.flush();
    }
    pub fn write_char(&mut self, char: impl Display, x: u16, y: u16) -> crossterm::Result<()> {
        self.out
            .queue(cursor::MoveTo(x,y))?
            .queue(style::Print(&char))?;
        Ok(())
    }
}