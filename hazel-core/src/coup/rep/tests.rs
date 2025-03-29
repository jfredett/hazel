use super::*;

mod creation {
    use super::*;

    #[test]
    fn new_move() {
        let m = Move::new(D2, D4, MoveType::DOUBLE_PAWN);
        assert_eq!(m.source(), D2);
        assert_eq!(m.target(), D4);
        assert!(!m.is_promotion());
        assert!(!m.is_null());
        assert!(m.is_double_pawn_push_for(Color::WHITE));
    }

    #[test]
    fn empty_move() {
        let m = Move::empty();
        assert_eq!(m.0, 0);
    }

    #[test]
    fn null_move() {
        let m = Move::null();
        assert!(m.is_null());
    }

}

mod from_notation {
    use super::*;

    #[test]
    fn quiet_move_parses_correctly() {
        let m = Move::from_notation("d2", "d4", MoveType::QUIET);

        assert_eq!(m.source_idx(), 0o13);
        assert_eq!(m.target_idx(), 0o33);
        assert!(!m.is_promotion());
        assert!(m.move_metadata().is_quiet());
    }

    #[test]
    fn promotion_move_parses_correctly() {
        let pm = Move::from_notation("d7", "d8", MoveType::PROMOTION_QUEEN);
        assert_eq!(pm.source_idx(), 0o63);
        assert_eq!(pm.target_idx(), 0o73);
        assert!(pm.is_promotion());
        assert_eq!(pm.promotion_piece(), Piece::Queen)
    }
}

mod castling {
    use super::*;

    mod white {
        use super::*;

        #[test]
        fn short_castle_parses_correctly() {
            let m = Move::short_castle(Color::WHITE);
            assert_eq!(m.source(), E1);
            assert_eq!(m.target(), G1);
            assert!(m.is_short_castle());
        }

        #[test]
        fn long_castle_parses_correctly() {
            let m = Move::long_castle(Color::WHITE);
            assert_eq!(m.source(), E1);
            assert_eq!(m.target(), C1);
            assert!(m.is_long_castle());
        }
    }

    mod black {
        use super::*;

        #[test]
        fn short_castle_parses_correctly() {
            let m = Move::short_castle(Color::BLACK);
            assert_eq!(m.source(), E8);
            assert_eq!(m.target(), G8);
            assert!(m.is_short_castle());
        }

        #[test]
        fn long_castle_parses_correctly() {
            let m = Move::long_castle(Color::BLACK);
            assert_eq!(m.source(), E8);
            assert_eq!(m.target(), C8);
            assert!(m.is_long_castle());
        }
    }
}

mod disambiguate {
    use hazel_basic::interface::{Alter, Alteration};
    use hazel_basic::square::*;
    use crate::board::PieceBoard;

    use super::*;

    #[test]
    fn quiet_move_disambiguates_correctly() {
        let m = Move::new(D2, D3, MoveType::UCI_AMBIGUOUS);
        let mut context = PieceBoard::default();
        context.set_startpos();

        assert_eq!(m.disambiguate(&context).unwrap(), MoveType::QUIET);
    }

    #[test]
    fn capture_move_disambiguates_correctly() {
        let m = Move::new(C3, D4, MoveType::UCI_AMBIGUOUS);
        let mut context = PieceBoard::default();
        context.alter_mut(Alteration::place(C3, Occupant::pawn(Color::WHITE)));
        context.alter_mut(Alteration::place(D4, Occupant::pawn(Color::BLACK)));

        assert_eq!(m.disambiguate(&context).unwrap(), MoveType::CAPTURE);
    }

    #[test]
    fn double_pawn_move_disambiguates_correctly() {
        let m = Move::new(D2, D4, MoveType::UCI_AMBIGUOUS);
        let mut context = PieceBoard::default();
        context.set_startpos();

        assert_eq!(m.disambiguate(&context).unwrap(), MoveType::DOUBLE_PAWN);
    }

    #[test]
    fn short_castle_disambiguates_correctly() {
        let m = Move::new(E1, G1, MoveType::UCI_AMBIGUOUS);
        let mut context = PieceBoard::default();
        context.set_startpos();
        context.alter_mut(Alteration::remove(F1, Occupant::bishop(Color::WHITE)));
        context.alter_mut(Alteration::remove(G1, Occupant::knight(Color::WHITE)));

        assert_eq!(m.disambiguate(&context).unwrap(), MoveType::SHORT_CASTLE);
    }

    #[test]
    fn long_castle_disambiguates_correctly() {
        let m = Move::new(E1, C1, MoveType::UCI_AMBIGUOUS);
        let mut context = PieceBoard::default();
        context.set_startpos();
        context.alter_mut(Alteration::remove(B1, Occupant::knight(Color::WHITE)));
        context.alter_mut(Alteration::remove(C1, Occupant::bishop(Color::WHITE)));

        assert_eq!(m.disambiguate(&context).unwrap(), MoveType::LONG_CASTLE);
    }
}

mod to_star {
    use crate::board::PieceBoard;
    use hazel_basic::interface::Alter;
    use hazel_basic::interface::Alteration;

    use super::*;

    mod pgn {

        use super::*;

        #[test]
        fn to_pgn_quiet_move() {
            let m = Move::new(D2, D3, MoveType::QUIET);
            let mut context = PieceBoard::default();
            context.set_startpos();

            assert_eq!(m.to_pgn(&context), "d3");
        }

        #[test]
        fn to_pgn_capture_move() {
            let m = Move::new(D4, E5, MoveType::CAPTURE);
            let mut context = PieceBoard::default();
            context.alter_mut(Alteration::place(D4, Occupant::white_pawn()));
            context.alter_mut(Alteration::place(E5, Occupant::black_pawn()));

            assert_eq!(m.to_pgn(&context), "dxe5");
        }

        #[test]
        fn to_pgn_double_pawn_move() {
            let m = Move::new(D2, D4, MoveType::DOUBLE_PAWN);
            let mut context = PieceBoard::default();
            context.set_startpos();

            assert_eq!(m.to_pgn(&context), "d4");
        }

        #[test]
        fn to_pgn_short_castle() {
            let m = Move::short_castle(Color::WHITE);
            let mut context = PieceBoard::default();
            // NOTE: Context does not mean it checks to see if the move is legal, in this position
            // white cannot castle, eppur si muove.
            context.set_startpos();

            assert_eq!(m.to_pgn(&context), "O-O");
        }

        #[test]
        fn to_pgn_long_castle() {
            let m = Move::long_castle(Color::WHITE);
            let mut context = PieceBoard::default();
            // NOTE: same caveat as short castling.
            context.set_startpos();

            assert_eq!(m.to_pgn(&context), "O-O-O");
        }

        #[test]
        fn to_pgn_promotion_move() {
            let m = Move::new(D7, D8, MoveType::PROMOTION_QUEEN);
            let mut context = PieceBoard::default();
            context.alter_mut(Alteration::place(D7, Occupant::white_pawn()));

            assert_eq!(m.to_pgn(&context), "d8=Q");
        }

        #[test]
        fn to_pgn_capture_promotion_move() {
            let m = Move::new(C7, D8, MoveType::PROMOTION_CAPTURE_QUEEN);
            let mut context = PieceBoard::default();
            context.alter_mut(Alteration::place(C7, Occupant::white_pawn()));
            context.alter_mut(Alteration::place(D8, Occupant::black_pawn()));

            assert_eq!(m.to_pgn(&context), "cxd8=Q");
        }
    }

    mod uci {
        use super::*;

        #[test]
        fn to_uci_quiet_move() {
            let m = Move::new(D2, D3, MoveType::QUIET);
            assert_eq!(m.to_uci(), "d2d3");
        }

        #[test]
        fn to_uci_capture_move() {
            let m = Move::new(D4, E5, MoveType::CAPTURE);
            assert_eq!(m.to_uci(), "d4e5");
        }

        #[test]
        fn to_uci_double_pawn_move() {
            let m = Move::new(D2, D4, MoveType::DOUBLE_PAWN);
            assert_eq!(m.to_uci(), "d2d4");
        }

        #[test]
        fn to_uci_short_castle() {
            let m = Move::short_castle(Color::WHITE);
            assert_eq!(m.to_uci(), "e1g1");
        }

        #[test]
        fn to_uci_long_castle() {
            let m = Move::long_castle(Color::WHITE);
            assert_eq!(m.to_uci(), "e1c1");
        }

        #[test]
        fn to_uci_promotion_move() {
            let m = Move::new(D7, D8, MoveType::PROMOTION_QUEEN);
            assert_eq!(m.to_uci(), "d7d8q");
        }

        #[test]
        fn to_uci_capture_promotion_move() {
            let m = Move::new(C7, D8, MoveType::PROMOTION_CAPTURE_QUEEN);
            assert_eq!(m.to_uci(), "c7d8q");
        }
    }
}

mod proxy_methods {
    use super::*;

    #[test]
    fn is_capture() {
        let m = Move::new(D2, D4, MoveType::CAPTURE);
        assert!(m.is_capture());
    }

    #[test]
    fn is_short_castle() {
        let m = Move::short_castle(Color::WHITE);
        assert!(m.is_short_castle());
    }

    #[test]
    fn is_long_castle() {
        let m = Move::long_castle(Color::WHITE);
        assert!(m.is_long_castle());
    }

    #[test]
    fn is_en_passant() {
        let m = Move::new(D6, E7, MoveType::EP_CAPTURE);
        assert!(m.is_en_passant());
    }

    #[test]
    fn is_double_pawn_push_for() {
        let m = Move::new(D2, D4, MoveType::DOUBLE_PAWN);
        assert!(m.is_double_pawn_push_for(Color::WHITE));
    }
}


mod compilation {
    use hazel_basic::ben;
    use hazel_basic::ben::BEN;
    use hazel_basic::position_metadata::PositionMetadata;

    use crate::board::simple::PieceBoard;
    use hazel_basic::interface::Alter;
    use hazel_basic::interface::Alteration;
    use crate::coup::rep::MoveType;

    use super::*;

    #[test]
    fn test_compile() {
        let mut board = PieceBoard::default();
        board.set_fen(BEN::start_position());
        let meta = PositionMetadata::default();
        let meta_after_move = PositionMetadata {
            side_to_move: Color::BLACK,
            en_passant: Some(File::D),
            ..Default::default()
        };

        let expected_alterations = vec![
            Alteration::Turn,
                Alteration::assert(&meta),

                Alteration::remove(D2, Occupant::white_pawn()),
                Alteration::place(D4, Occupant::white_pawn()),

                Alteration::inform(&meta_after_move),
        ];

        let mov = Move::new(D2, D4, MoveType::DOUBLE_PAWN);

        similar_asserts::assert_eq!(mov.new_compile(&board, &meta), expected_alterations);
    }
}
