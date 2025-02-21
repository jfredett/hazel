pub mod board_section;
pub mod info_section;

use ratatui::layout::Direction;
use ratatui::prelude::*;

use board_section::BoardSection;
use info_section::InfoSection;

use crate::board::simple::PieceBoard;
use crate::notation::pgn::PGN;

lazy_static! {
    static ref LAYOUT : Layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Length(40),
                Constraint::Length(24)
            ].as_ref(),
        );
}

#[derive(Default)]
pub struct GameSectionLayout<'a> {
    info_section: InfoSection,
    board_section: BoardSection<'a>
}


impl GameSectionLayout<'_> {
    // FIXME: this should really take a "Thing that has a variation" and not necessarily a PGN.
    // There are a set of UI traits that I should probably build independently of the engine, this
    // would be one of them.
    pub fn new(pgn: PGN) -> Self {
        let mut board = PieceBoard::default();
        board.set_position(pgn.current_position());
        Self {
            info_section: InfoSection::new(pgn),
            board_section: BoardSection::from(board),
        }
    }
}

impl Widget for &GameSectionLayout<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let chunks = LAYOUT.split(area);

        self.info_section.render(chunks[0], buf);
        self.board_section.render(chunks[1], buf);
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use insta::*;

    fn example_game() -> PGN {
        PGN::load("tests/fixtures/no-variations-and-halts.pgn").unwrap()
    }

    #[test]
    fn renders_as_expected() {
        let rect = Rect::new(0, 0, 64, 17);
        let mut buffer = Buffer::empty(rect);
        buffer.set_style(rect, Style::default().fg(Color::White).bg(Color::Black));

        let game_section = GameSectionLayout::new(example_game());
        game_section.render(rect, &mut buffer);

        assert_debug_snapshot!(buffer);
    }
}

