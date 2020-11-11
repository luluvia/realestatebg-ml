use termimad::Area;
use crossterm::{cursor, Result, terminal::{Clear, ClearType}, QueueableCommand};
use std::io::Write;
use crate::display::layout::Layout;

pub struct Screen {
    area: Area,
    pub areas: Vec::new(),
    layout: Layout
}

impl Screen {
    pub fn new(layout: &impl Layout) -> Screen {
        let area = Area::full_screen();
        let layout = layout;
        let areas = layout.construct(&area);
        Self {
            area,
            areas,
            layout,
        }
    }
    pub fn load_layout(&mut self, layout: Layout) {
        self.layout = layout;
        self.areas = layout.construct(&area);
    }
    pub fn resize_terminal(&mut self, w: u16, h: u16) {
        self.area = Area::new(0, 0, w, h);
        self.areas = self.layout.construct(&self.area);
    }
    pub fn clear(&self, w: &mut impl Write) -> Result<()> {
        w.queue(cursor::MoveTo(x, y))?;
        Ok(())
    }
}