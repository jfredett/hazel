#![allow(unused_imports, dead_code)]

use ratatui::prelude::*;
use crate::board::simple::PieceBoard;
use crate::board::Query;
use crate::types::{self, Occupant, Piece};
use crate::notation::*;

use ratatui::widgets::{Table, Row};

/// A widget what representeth a board in a 8x8 grid of 3x2 character cells. This is messy and bad
/// and will undergo some kind of finishing someday when I tire of it. Be ye warned, dragons doth
/// lie here.
#[derive(Default)]
pub struct Board<'a> {
    state: PieceBoard,
    board: Table<'a>
}

impl From<PieceBoard> for Board<'_> {
    fn from(state: PieceBoard) -> Self {
        let white_bg = Style::default().bg(WHITE_SQUARE).fg(Color::Black);
        let black_bg = Style::default().bg(BLACK_SQUARE).fg(Color::White);
        let white_triple = [white_bg].repeat(3);
        let black_triple = [black_bg].repeat(3);
        let white_first_row = Row::new([white_triple.clone(), black_triple.clone(), white_triple.clone(), black_triple.clone(), white_triple.clone(), black_triple.clone(), white_triple.clone(), black_triple.clone()].concat().into_iter().map(|style| Span::styled(" ", style)));
        let black_first_row = Row::new([black_triple.clone(), white_triple.clone(), black_triple.clone(), white_triple.clone(), black_triple.clone(), white_triple.clone(), black_triple.clone(), white_triple.clone()].concat().into_iter().map(|style| Span::styled(" ", style)));

        let table = Table::new([
            white_first_row.clone(),
            white_first_row.clone(),
            black_first_row.clone(),
            black_first_row.clone(),
            white_first_row.clone(),
            white_first_row.clone(),
            black_first_row.clone(),
            black_first_row.clone(),
            white_first_row.clone(),
            white_first_row.clone(),
            black_first_row.clone(),
            black_first_row.clone(),
            white_first_row.clone(),
            white_first_row.clone(),
            black_first_row.clone(),
            black_first_row.clone(),
        ], Constraint::from_maxes([1].repeat(24))).column_spacing(0);

        Self {
            state,
            board: table
        }
    }
}

/*
const ROOK: &'static str = "♜";
const KNIGHT : &'static str = "♞";
const BISHOP : &'static str = "♝";
const QUEEN : &'static str = "♛";
const KING : &'static str = "♚";
const PAWN : &'static str = "♟";
*/
const ROOK: &str = "R";
const KNIGHT : &str = "N";
const BISHOP : &str = "B";
const QUEEN : &str = "Q";
const KING : &str = "K";
const PAWN : &str = "P";


const WHITE_SQUARE : ratatui::prelude::Color = Color::Rgb(0x80, 0x80, 0x80);
const BLACK_SQUARE : ratatui::prelude::Color = Color::Rgb(0x40, 0x40, 0x40);

const BLACK_PIECE_WHITE_SQUARE : ratatui::prelude::Color = Color::Rgb(0x00, 0x00, 0x00);
const BLACK_PIECE_BLACK_SQUARE : ratatui::prelude::Color = Color::Rgb(0x00, 0x00, 0x00);
const WHITE_PIECE_WHITE_SQUARE : ratatui::prelude::Color = Color::Rgb(0xFF, 0xFF, 0xFF);
const WHITE_PIECE_BLACK_SQUARE : ratatui::prelude::Color = Color::Rgb(0xFF, 0xFF, 0xFF);

const MARK_COLOR : ratatui::prelude::Color = Color::Rgb(0x65, 0x65, 0x65);

impl Widget for &Board<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        ratatui::prelude::Widget::render(&self.board, area, buf);

        let mut cursor = Square::by_rank_and_file();
        cursor.downward();
        for s in cursor {
            // Ratatui uses an Origin in the Top Left, with the first component being the
            // left/right offset ('file') and the second being the up/down offset ('rank').
            //
            // Chess, and in particular Hazel, uses an Origin in the Bottom Left, with the first
            // component being the rank and the second being the file. This is deeply annoying, but
            // can be neatly solved by doing some magic here.
            let buf_file : u16 = 14 - 2*s.rank() as u16;
            let buf_rank : u16 = 3*s.file() as u16 + 1;

            let cell = buf.get_mut(area.x + buf_rank, area.y + buf_file);
            let occ = self.state.get(s);

            match occ {
                Occupant::Occupied(piece, color) => {
                    let symbol = match piece {
                        Piece::Rook => { ROOK },
                        Piece::Knight => { KNIGHT },
                        Piece::Bishop => { BISHOP },
                        Piece::Queen => { QUEEN },
                        Piece::King => { KING },
                        Piece::Pawn => { PAWN }
                    };
                    cell.set_symbol(symbol);

                    match (cell.bg, color) {
                        (WHITE_SQUARE, types::Color::BLACK) => cell.fg = BLACK_PIECE_WHITE_SQUARE,
                        (WHITE_SQUARE, types::Color::WHITE) => cell.fg = WHITE_PIECE_WHITE_SQUARE,
                        (BLACK_SQUARE, types::Color::BLACK) => cell.fg = BLACK_PIECE_BLACK_SQUARE,
                        (BLACK_SQUARE, types::Color::WHITE) => cell.fg = WHITE_PIECE_BLACK_SQUARE,
                        _ => {}
                    }

                    cell.set_style(cell.style().bold());
                }
                Occupant::Empty => {} // Empty is the default state of the board.
            }

            // while we're here, I want to do write column/row information on the bottom bit of
            // the board in a nearly invisible color.
            let col_mark = buf.get_mut(area.x + buf_rank - 1, area.y + buf_file + 1);
            col_mark.set_char((b'a' + s.file() as u8) as char);
            col_mark.fg = MARK_COLOR;

            let row_mark = buf.get_mut(area.x + buf_rank, area.y + buf_file + 1);
            // note this sign flip, counting up from 'a' above, down from '8' here.
            row_mark.set_char((b'1' + s.rank() as u8) as char);
            row_mark.fg = MARK_COLOR;
        }


    }
}

#[cfg(test)]
mod tests {
    use ratatui::widgets::Cell;

    use super::*;

    #[test]
    fn renders_empty_board() {
        let rect = Rect::new(0, 0, 33, 17);
        let mut buffer = Buffer::empty(rect);
        buffer.set_style(rect, Style::default().fg(Color::White).bg(Color::Black));

        let board = PieceBoard::default();
        let board_widget = &Board::from(board);
        board_widget.render(rect, &mut buffer);

        let mut expected = Buffer::with_lines(vec![
            "                                 ",
            "a8 b8 c8 d8 e8 f8 g8 h8          ",
            "                                 ",
            "a7 b7 c7 d7 e7 f7 g7 h7          ",
            "                                 ",
            "a6 b6 c6 d6 e6 f6 g6 h6          ",
            "                                 ",
            "a5 b5 c5 d5 e5 f5 g5 h5          ",
            "                                 ",
            "a4 b4 c4 d4 e4 f4 g4 h4          ",
            "                                 ",
            "a3 b3 c3 d3 e3 f3 g3 h3          ",
            "                                 ",
            "a2 b2 c2 d2 e2 f2 g2 h2          ",
            "                                 ",
            "a1 b1 c1 d1 e1 f1 g1 h1          ",
            "                                 ",
        ]);
        buffer.set_style(rect, Style::default().fg(Color::White).bg(Color::Black));
        expected.set_style(rect, Style::default().fg(Color::White).bg(Color::Black));


        assert_eq!(buffer, expected);
    }
}
