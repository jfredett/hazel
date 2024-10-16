use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::widgets::Borders;
use ratatui::prelude::*;

use super::engine_io_section::EngineIOSection;
use super::fen::FEN;
use super::game_section::GameSectionLayout;
use super::placeholder::Placeholder;

use crate::board::simple::PieceBoard;

lazy_static! {
    static ref LAYOUT : Layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(19), // Game Section w/ Info and Board Sections
                Constraint::Length(1), // Query Line, shown in the sketch w/ a FEN of the current position.
                Constraint::Length(12), // Engine IO Section
            ]
                .as_ref(),
        );
}

const WIDTH : u16 = 64;
const HEIGHT : u16 = 32;

pub struct Tile {
    /*
    game_section: GameSection,
    query_line: Query,
    */
    engine_io_section: EngineIOSection,
    state: PieceBoard,
}


impl Tile {
    pub fn new() -> Self {
        Self {
            /*
            game_section: GameSection::new(),
            */
            engine_io_section: EngineIOSection::default(),
            state: PieceBoard::default(),
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

    pub fn set_state(&mut self, state: PieceBoard) {
        self.state = state;
    }

    pub fn query_line(&self) -> FEN {
        FEN::from(self.state).center()
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

        let game_section = GameSectionLayout::new(self.state);
        game_section.render(chunks[0], buf);
        //Placeholder::of_size(chunks[0].width, chunks[0].height).render(chunks[0], buf);
        Placeholder::of_size(chunks[1].width, chunks[1].height).borders(Borders::LEFT | Borders::RIGHT).render(chunks[1], buf);

        self.engine_io_section.render(chunks[2], buf);

        // self.game_section.render(chunks[0], buf, state);
        self.query_line().render(chunks[1], buf);
        // self.engine_io_section.render(chunks[2], buf, state);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_as_expected_in_larger_canvas() {
        let rect = Rect::new(0, 0, 65, 33);
        let mut actual = Buffer::empty(rect);
        actual.set_style(rect, Style::default().fg(Color::White).bg(Color::Black));

        let tile = Tile::new();
        tile.render(rect, &mut actual);

        // FIXME: https://github.com/ratatui/ratatui/issues/605 This issue does what I _wish_ this
        // was doing, in particular, I'd prefer the corners 'merge' into the next set of borders,
        // but they don't do that right now and I can't really ascertain what state that fork is in
        // other than it's 260something commits old at time of writing.
        let mut expected = Buffer::with_lines(vec![
            "               Placeholder                                       ",
            "               Placeholder              a8 b8 c8 d8 e8 f8 g8 h8  ",
            "               Placeholder                                       ",
            "┌──────────────────┐┌──────────────────┐a7 b7 c7 d7 e7 f7 g7 h7  ",
            "│    Placeholder   ││    Placeholder   │                         ",
            "│    Placeholder   ││    Placeholder   │a6 b6 c6 d6 e6 f6 g6 h6  ",
            "│    Placeholder   ││    Placeholder   │                         ",
            "│    Placeholder   ││    Placeholder   │a5 b5 c5 d5 e5 f5 g5 h5  ",
            "│    Placeholder   ││    Placeholder   │                         ",
            "│    Placeholder   ││    Placeholder   │a4 b4 c4 d4 e4 f4 g4 h4  ",
            "│    Placeholder   ││    Placeholder   │                         ",
            "│    Placeholder   ││    Placeholder   │a3 b3 c3 d3 e3 f3 g3 h3  ",
            "│    Placeholder   ││    Placeholder   │                         ",
            "│    Placeholder   ││    Placeholder   │a2 b2 c2 d2 e2 f2 g2 h2  ",
            "│    Placeholder   ││    Placeholder   │                         ",
            "│    Placeholder   ││    Placeholder   │a1 b1 c1 d1 e1 f1 g1 h1  ",
            "│    Placeholder   ││    Placeholder   │       Placeholder       ",
            "│    Placeholder   ││    Placeholder   │       Placeholder       ",
            "└──────────────────┘└──────────────────┘       Placeholder       ",
            "│                 8/8/8/8/8/8/8/8 w KQkq - 0 1                 │",
            "┌──────────────────────────────────────────────────────────────┐ ",
            "│                                                              │ ",
            "│                                                              │ ",
            "│                                                              │ ",
            "│                                                              │ ",
            "│                                                              │ ",
            "│                                                              │ ",
            "│                                                              │ ",
            "│                                                              │ ",
            "│                                                              │ ",
            "└──────────────────────────────────────────────────────────────┘ ",
            "$>                                                               ",
            "                                                                 ",
        ]);

        // Ignore style differences for now
        expected.set_style(rect, Style::default().fg(Color::White).bg(Color::Black));
        actual.set_style(rect, Style::default().fg(Color::White).bg(Color::Black));
        assert_eq!(actual, expected);
    }
    #[test]
    fn renders_as_expected() {
        let rect = Rect::new(0, 0, 64, 32);
        let mut actual = Buffer::empty(rect);
        actual.set_style(rect, Style::default().fg(Color::White).bg(Color::Black));

        let tile = Tile::new();
        tile.render(rect, &mut actual);

        // FIXME: see above
        let mut expected = Buffer::with_lines(vec![
            "               Placeholder                                      ",
            "               Placeholder              a8 b8 c8 d8 e8 f8 g8 h8 ",
            "               Placeholder                                      ",
            "┌──────────────────┐┌──────────────────┐a7 b7 c7 d7 e7 f7 g7 h7 ",
            "│    Placeholder   ││    Placeholder   │                        ",
            "│    Placeholder   ││    Placeholder   │a6 b6 c6 d6 e6 f6 g6 h6 ",
            "│    Placeholder   ││    Placeholder   │                        ",
            "│    Placeholder   ││    Placeholder   │a5 b5 c5 d5 e5 f5 g5 h5 ",
            "│    Placeholder   ││    Placeholder   │                        ",
            "│    Placeholder   ││    Placeholder   │a4 b4 c4 d4 e4 f4 g4 h4 ",
            "│    Placeholder   ││    Placeholder   │                        ",
            "│    Placeholder   ││    Placeholder   │a3 b3 c3 d3 e3 f3 g3 h3 ",
            "│    Placeholder   ││    Placeholder   │                        ",
            "│    Placeholder   ││    Placeholder   │a2 b2 c2 d2 e2 f2 g2 h2 ",
            "│    Placeholder   ││    Placeholder   │                        ",
            "│    Placeholder   ││    Placeholder   │a1 b1 c1 d1 e1 f1 g1 h1 ",
            "│    Placeholder   ││    Placeholder   │       Placeholder      ",
            "│    Placeholder   ││    Placeholder   │       Placeholder      ",
            "└──────────────────┘└──────────────────┘       Placeholder      ",
            "│                 8/8/8/8/8/8/8/8 w KQkq - 0 1                 │",
            "┌──────────────────────────────────────────────────────────────┐",
            "│                                                              │",
            "│                                                              │",
            "│                                                              │",
            "│                                                              │",
            "│                                                              │",
            "│                                                              │",
            "│                                                              │",
            "│                                                              │",
            "│                                                              │",
            "└──────────────────────────────────────────────────────────────┘",
            "$>                                                              ",
        ]);

        // Unify the styles, I'm comfortable not testing that for now.
        expected.set_style(rect, Style::default().fg(Color::White).bg(Color::Black));
        actual.set_style(rect, Style::default().fg(Color::White).bg(Color::Black));
        assert_eq!(actual, expected);
    }
}

