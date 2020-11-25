use crossterm::{cursor, Result, QueueableCommand};
use std::io::Write;
use crate::display::layout::{Layout, LayoutCode};
use crate::display::area::{Area, Areas};

pub struct Screen {
    area: Area,
    pub areas: Areas,
    layout: Box<dyn Layout>,
    layout_code: LayoutCode
}

impl Screen {
    pub fn new(layout: Box<dyn Layout>) -> Screen {
        let area = Area::fullscreen();
        let (areas, layout_code) = layout.construct(&area);
        Self {
            area,
            areas,
            layout,
            layout_code,
        }
    }
    pub fn load_layout(&mut self, layout: Box<dyn Layout>) {
        let (areas, layout_code) = layout.construct(&self.area);
        self.areas = areas;
        self.layout_code = layout_code;
        self.layout = layout;
    }
    pub fn resize_terminal(&mut self, w: u16, h: u16) {
        self.area = Area::new(0, 0, w, h);
        let (areas, layout_code) = self.layout.construct(&self.area);
        self.areas = areas;
        self.layout_code = layout_code;
    }
    pub fn clear(&self, w: &mut impl Write) -> Result<()> {
        w.queue(cursor::MoveTo(0, 0))?;
        Ok(())
    }
}