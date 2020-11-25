use crate::display::area::{Area, Areas};

pub trait Layout {
    fn construct(&self, view: &Area) -> (Areas, LayoutCode);
}

// layout code is saved by the Screen to determine what areas are available
pub enum LayoutCode {
    SingleMenuLayout,
    GameLayout
}

pub struct SingleMenuLayout { }

impl Layout for SingleMenuLayout {
    fn construct(&self, view: &Area) -> (Areas, LayoutCode) {
        let layout = LayoutCode::SingleMenuLayout;
        // the menu only has one area to display so we'll use the full view.
        let viewport = Area::new(
            view.left,
            view.top,
            view.width,
            view.height
        );

        (Areas {
            viewport,
            command: None,
            logs: None,
            status: None
        }, layout)
    }
}

pub struct GameLayout { }

impl Layout for GameLayout {
    fn construct(&self, view: &Area) -> (Areas, LayoutCode) {
        let layout = LayoutCode::GameLayout;
        // the game has a game viewport on the top-left, and a command guide on the top-right,
        // a log on the bottom-left, and a status column on the bottom-right.
        let viewport = Area::new(
            view.left,
            view.top,
            view.width * 3 / 4,
            view.height * 1 / 2
        );
        let command = Area::new(
            view.left + viewport.width,
            view.top,
            view.width - viewport.width,
            view.height * 1 / 4
        );
        let logs = Area::new(
            view.left,
            view.top + viewport.height,
            view.width * 3 / 4,
            view.height - viewport.height
        );
        let status = Area::new(
            view.left + logs.width,
            view.top + command.height,
            view.width - logs.width,
            view.height - command.height
        );

        (Areas {
            viewport,
            command: Some(command),
            logs: Some(logs),
            status: Some(status)
        }, layout)
    }
}