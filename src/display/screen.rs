use crate::display::layout::{Layout, LayoutCode};
use crate::display::area::{Area, Areas};
use crate::display::writer::Writer;

pub struct Screen<'a> {
    layout: Box<dyn Layout>,
    layout_code: LayoutCode,
    pub area: Area,
    pub areas: Areas,
    pub writer: Writer<'a>
}

impl<'a> Screen<'a> {
    pub fn new(layout: Box<dyn Layout>, writer: Writer<'a>) -> Screen {
        let area = Area::fullscreen();
        let (areas, layout_code) = layout.construct(&area);
        Self {
            area,
            areas,
            layout,
            layout_code,
            writer,
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
        self.writer.clear();
    }
}