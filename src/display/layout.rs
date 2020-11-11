use termimad::Area;

pub trait Layout {
    fn construct(&self, view: Area);
}

pub struct SingleMenuLayout {
    areas: Vec::new(),
}

impl Layout for SingleMenuLayout {
    fn construct(&self, view: Area) {
        // the menu only has one area to display so we'll use the full view.
        let frame = Area::new(
            view.left,
            view.top,
            view.width,
            view.height
        );
        areas.push(frame);
    }
}

pub struct GameLayout {
    areas: Vec::new(),
}

impl Layout for GameLayout {
    fn construct(&self, view: Area) {
        // the game has a game viewport on the top-left, and a command guide on the top-right,
        // a log on the bottom-left, and a status column on the bottom-right.
        let game_port = Area::new(
            view.left,
            view.top,
            view.width * 3 / 4,
            view.height * 1 / 2
        );
        let command_area = Area::new(
            view.left + game_port.width,
            view.top,
            view.width - game_part.width,
            view.height * 1 / 4
        );
        let log_area = Area::new(
            view.left,
            view.top + game_port.height,
            view.width * 3 / 4,
            view.height - game_port.height
        );
        let status_area = Area::new(
            view.left + log_area.width,
            view.top + command_area.height,
            view.width - log_area.width,
            view.height - command_area.height
        );
        areas.push(game_port);
        areas.push(command_area);
        areas.push(log_area);
        areas.push(status_area);
    }
}