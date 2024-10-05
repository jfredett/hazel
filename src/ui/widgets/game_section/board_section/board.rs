#![allow(unused_imports, dead_code)]

use ratatui::prelude::*;
use crate::{constants, ui::model::pieceboard::PieceBoard};
use crate::ui::model::occupant::Occupant;
use crate::constants::Piece;

use ratatui::widgets::{Table, Row};

pub struct Board<'a> {
    state: PieceBoard,
    board: Table<'a>
}

impl From<PieceBoard> for Board<'_> {
    fn from(state: PieceBoard) -> Self {
        let white_bg = Style::default().bg(WHITE_SQUARE).fg(Color::Black);
        let black_bg = Style::default().bg(BLACK_SQUARE).fg(Color::White);
        let white_triple = vec![white_bg].repeat(3);
        let black_triple = vec![black_bg].repeat(3);
        let white_first_row = Row::new(vec![white_triple.clone(), black_triple.clone(), white_triple.clone(), black_triple.clone(), white_triple.clone(), black_triple.clone(), white_triple.clone(), black_triple.clone()].concat().into_iter().map(|style| Span::styled(" ", style)));
        let black_first_row = Row::new(vec![black_triple.clone(), white_triple.clone(), black_triple.clone(), white_triple.clone(), black_triple.clone(), white_triple.clone(), black_triple.clone(), white_triple.clone()].concat().into_iter().map(|style| Span::styled(" ", style)));

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
        ], Constraint::from_maxes(vec![1].repeat(24))).column_spacing(0);

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
const ROOK: &'static str = "R";
const KNIGHT : &'static str = "N";
const BISHOP : &'static str = "B";
const QUEEN : &'static str = "Q";
const KING : &'static str = "K";
const PAWN : &'static str = "P";


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

        // This is a total mess of magic I am not proud of, here is the breakdown:
        //
        // It's a lot easier to set up the big 24x16 board as it's own table once
        // and just render it by reference as a sort of texture.
        //
        // In a second pass, I jump around to the placement of the pieces on the board (centered,
        // upper row o each 3x2 'square'). I then set the symbol and color of the cell based on the
        // state of the board and the offset of the current area I'm rendering.
        //
        // After that, I go and set the style of the cell to make the pieces show up.
        for i in 0..8 {
            for j in 0..8 {
                let cell = buf.get_mut(area.x + 3*i + 1, area.y + 2*j);
                match self.state.get((7 - j).into(), i.into()) {
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
                            (WHITE_SQUARE, constants::Color::BLACK) => cell.fg = BLACK_PIECE_WHITE_SQUARE,
                            (WHITE_SQUARE, constants::Color::WHITE) => cell.fg = WHITE_PIECE_WHITE_SQUARE, 
                            (BLACK_SQUARE, constants::Color::BLACK) => cell.fg = BLACK_PIECE_BLACK_SQUARE,
                            (BLACK_SQUARE, constants::Color::WHITE) => cell.fg = WHITE_PIECE_BLACK_SQUARE,
                            _ => {}
                        }

                        cell.set_style(cell.style().bold());
                    }
                    Occupant::Empty => {} // Empty is the default state of the board.
                }
                // while we're here, I want to do write column/row information on the bottom bit of
                // the board in a nearly invisible color.
                let col_mark = buf.get_mut(area.x + 3*i + 0, area.y + 2*j + 1);
                col_mark.set_char((b'a' + i as u8) as char);
                col_mark.fg = MARK_COLOR;

                let row_mark = buf.get_mut(area.x + 3*i + 1, area.y + 2*j + 1);
                row_mark.set_char((b'8' - j as u8) as char);
                row_mark.fg = MARK_COLOR;

            }
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
