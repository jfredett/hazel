use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::widgets::Borders;
use ratatui::prelude::*;

use super::engine_io_section::EngineIOSection;
use super::fen::FEN;
use super::game_section::GameSectionLayout;
use super::placeholder::Placeholder;

use hazel::notation::pgn::PGN;

lazy_static! {
    static ref LAYOUT : Layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(19), // Game Section w/ Info and Board Sections
                Constraint::Length(1),  // Query Line, shown in the sketch w/ a FEN of the current position.
                Constraint::Length(12), // Engine IO Section
            ]
                .as_ref(),
        );
}

const WIDTH : u16 = 64;
const HEIGHT : u16 = 32;

#[derive(Default)]
pub struct Tile {
    engine_io_section: EngineIOSection,
    state: PGN,
}


impl Tile {
    pub fn new() -> Self {
        Self {
            engine_io_section: EngineIOSection::default(),
            // HACK: Obviously wrong, but pending loading up from a game/interacting with an engine
            // backend.
            state: PGN::load("tests/fixtures/no-variations-and-halts.pgn").unwrap(),
        }
    }

    pub fn handle_input(&mut self, input: char) {
        self.engine_io_section.handle_input(input);
    }

    pub fn handle_backspace(&mut self) {
        self.engine_io_section.handle_backspace();
    }

    pub fn handle_enter(&mut self) {
        self.engine_io_section.handle_enter();
    }

    pub fn set_state(&mut self, state: PGN) {
        self.state = state;
    }

    pub fn query_line(&self) -> FEN {
        FEN::new(self.state.current_position()).alignment(Alignment::Center)
    }
}

impl Widget for &Tile {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // TODO: We always render at the same size, this will likely be wrong if the size is too
        // small, but I can add logic later.
        // Really I need an 'offset' Rect, the x,y components are added, the WIDTH/HEIGHT are min'd
        // against the provided area, if the area is too small, we render to an internal buffer and
        // cut the piece we out to match.
        let adjusted_area = Rect::new(area.x, area.y, WIDTH, HEIGHT);
        let chunks = LAYOUT.split(adjusted_area);

        let game_section = GameSectionLayout::new(self.state.clone());
        game_section.render(chunks[0], buf);
        Widget::render(
            &Placeholder::of_size(chunks[1].width, chunks[1].height).borders(Borders::LEFT | Borders::RIGHT),
            chunks[1],
            buf
        );

        self.engine_io_section.render(chunks[2], buf);

        self.query_line().render(chunks[1], buf);
        self.engine_io_section.render(chunks[2], buf);
    }
}


#[cfg(test)]
mod tests {
    use insta::assert_debug_snapshot;

    use super::*;

    #[test]
    fn renders_as_expected_in_larger_canvas() {
        let rect = Rect::new(0, 0, 65, 33);
        let mut actual = Buffer::empty(rect);
        actual.set_style(rect, Style::default().fg(Color::White).bg(Color::Black));

        let tile = Tile::new();
        tile.render(rect, &mut actual);

        assert_debug_snapshot!(actual);
    }
    #[test]
    fn renders_as_expected() {
        let rect = Rect::new(0, 0, 64, 32);
        let mut actual = Buffer::empty(rect);
        actual.set_style(rect, Style::default().fg(Color::White).bg(Color::Black));

        let tile = Tile::new();
        tile.render(rect, &mut actual);

        assert_debug_snapshot!(actual);
    }
}

