use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::widgets::{Borders, StatefulWidget};
use ratatui::prelude::*;

use crate::ui::app::Hazel;

use super::placeholder::Placeholder;

lazy_static! {
    static ref LAYOUT : Layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(17), // Game Section w/ Info and Board Sections
                Constraint::Length(1), // Query Line, shown in the sketch w/ a FEN of the current position.
                Constraint::Length(14), // Engine IO Section
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
    engine_io_section: EngineIOSection,
    */
}


impl Tile {
    pub fn new() -> Self {
        Self {
            /*
            game_section: GameSection::new(),
            query_line: Query::new(),
            engine_io_section: EngineIOSection::new(),
            */
        }
    }
}

impl StatefulWidget for &Tile {
    type State = Hazel;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        // TODO: We always render at the same size, this will likely be wrong if the size is too
        // small, but I can add logic later.

        let adjusted_area = Rect::new(area.x, area.y, WIDTH, HEIGHT);
        let chunks = LAYOUT.split(area);

        Placeholder::of_size(chunks[0].width, chunks[0].height).render(chunks[0], buf);
        Placeholder::of_size(chunks[1].width, chunks[1].height).borders(Borders::LEFT | Borders::RIGHT).render(chunks[1], buf);
        Placeholder::of_size(chunks[2].width, chunks[2].height).render(chunks[2], buf);

        // self.game_section.render(chunks[0], buf, state);
        // self.query_line.render(chunks[1], buf, state);
        // self.engine_io_section.render(chunks[2], buf, state);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_layout() {
        let rect = Rect::new(0, 0, 64, 32);
        let mut buffer = Buffer::empty(rect);
        buffer.set_style(rect, Style::default().fg(Color::White).bg(Color::Black));

        let tile = &mut Tile::new();
        tile.render(rect, &mut buffer, &mut Hazel::new());

        // FIXME: https://github.com/ratatui/ratatui/issues/605 This issue does what I _wish_ this
        // was doing, in particular, I'd prefer the corners 'merge' into the next set of borders,
        // but they don't do that right now and I can't really ascertain what state that fork is in
        // other than it's 260something commits old at time of writing.
        let mut expected = Buffer::with_lines(vec![
            "┌──────────────────────────────────────────────────────────────┐",
            "│                          Placeholder                         │",
            "│                          Placeholder                         │",
            "│                          Placeholder                         │",
            "│                          Placeholder                         │",
            "│                          Placeholder                         │",
            "│                          Placeholder                         │",
            "│                          Placeholder                         │",
            "│                          Placeholder                         │",
            "│                          Placeholder                         │",
            "│                          Placeholder                         │",
            "│                          Placeholder                         │",
            "│                          Placeholder                         │",
            "│                          Placeholder                         │",
            "│                          Placeholder                         │",
            "│                          Placeholder                         │",
            "└──────────────────────────────────────────────────────────────┘",
            "│                          Placeholder                         │",
            "┌──────────────────────────────────────────────────────────────┐",
            "│                          Placeholder                         │",
            "│                          Placeholder                         │",
            "│                          Placeholder                         │",
            "│                          Placeholder                         │",
            "│                          Placeholder                         │",
            "│                          Placeholder                         │",
            "│                          Placeholder                         │",
            "│                          Placeholder                         │",
            "│                          Placeholder                         │",
            "│                          Placeholder                         │",
            "│                          Placeholder                         │",
            "│                          Placeholder                         │",
            "└──────────────────────────────────────────────────────────────┘",
        ]);

        expected.set_style(rect, Style::default().fg(Color::White).bg(Color::Black));
        assert_eq!(buffer, expected);
    }
}

