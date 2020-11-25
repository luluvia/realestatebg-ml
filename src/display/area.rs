use crossterm::terminal;

#[derive(Copy, Clone)]
pub struct Area {
    pub left: u16,
    pub top: u16,
    pub width: u16,
    pub height: u16,
}

impl Area {
    pub fn new(left: u16, top: u16, width: u16, height: u16) -> Area {
        if width < 1 || height < 1 {
            eprintln!("New rectangle too small to be valid");
        }
        Area {
            left,
            top,
            width,
            height,
        }
    }

    pub fn full_area(&self) -> (u16, u16, u16, u16) { (self.left, self.top, self.width, self.height) }

    pub fn fullscreen() -> Area {
        let (width, height) = terminal::size().unwrap();
        Area {
            left: 0,
            top: 0,
            width,
            height,
        }
    }

    pub fn get_coords_from_percent(&self, pc_width: f32, pc_height: f32) -> (u16, u16) {
        (self.left + (self.width as f32 * pc_width).round() as u16 - 1,
         self.top + (self.height as f32 * pc_height).round() as u16 - 1)
    }
}

pub struct Areas {
    pub viewport: Area, // not optional because we need to have a display if we are using windowed mode
    // and not just print mode
    pub command: Option<Area>,
    pub logs: Option<Area>,
    pub status: Option<Area>,
}