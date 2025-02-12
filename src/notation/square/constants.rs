use super::*;

// These can be used to directly index into a board array at comptime with no overhead
// since everything here is const-time.
pub const A1: Square = Square::new(0);
pub const B1: Square = Square::new(1);
pub const C1: Square = Square::new(2);
pub const D1: Square = Square::new(3);
pub const E1: Square = Square::new(4);
pub const F1: Square = Square::new(5);
pub const G1: Square = Square::new(6);
pub const H1: Square = Square::new(7);
pub const A2: Square = Square::new(8);
pub const B2: Square = Square::new(9);
pub const C2: Square = Square::new(10);
pub const D2: Square = Square::new(11);
pub const E2: Square = Square::new(12);
pub const F2: Square = Square::new(13);
pub const G2: Square = Square::new(14);
pub const H2: Square = Square::new(15);
pub const A3: Square = Square::new(16);
pub const B3: Square = Square::new(17);
pub const C3: Square = Square::new(18);
pub const D3: Square = Square::new(19);
pub const E3: Square = Square::new(20);
pub const F3: Square = Square::new(21);
pub const G3: Square = Square::new(22);
pub const H3: Square = Square::new(23);
pub const A4: Square = Square::new(24);
pub const B4: Square = Square::new(25);
pub const C4: Square = Square::new(26);
pub const D4: Square = Square::new(27);
pub const E4: Square = Square::new(28);
pub const F4: Square = Square::new(29);
pub const G4: Square = Square::new(30);
pub const H4: Square = Square::new(31);
pub const A5: Square = Square::new(32);
pub const B5: Square = Square::new(33);
pub const C5: Square = Square::new(34);
pub const D5: Square = Square::new(35);
pub const E5: Square = Square::new(36);
pub const F5: Square = Square::new(37);
pub const G5: Square = Square::new(38);
pub const H5: Square = Square::new(39);
pub const A6: Square = Square::new(40);
pub const B6: Square = Square::new(41);
pub const C6: Square = Square::new(42);
pub const D6: Square = Square::new(43);
pub const E6: Square = Square::new(44);
pub const F6: Square = Square::new(45);
pub const G6: Square = Square::new(46);
pub const H6: Square = Square::new(47);
pub const A7: Square = Square::new(48);
pub const B7: Square = Square::new(49);
pub const C7: Square = Square::new(50);
pub const D7: Square = Square::new(51);
pub const E7: Square = Square::new(52);
pub const F7: Square = Square::new(53);
pub const G7: Square = Square::new(54);
pub const H7: Square = Square::new(55);
pub const A8: Square = Square::new(56);
pub const B8: Square = Square::new(57);
pub const C8: Square = Square::new(58);
pub const D8: Square = Square::new(59);
pub const E8: Square = Square::new(60);
pub const F8: Square = Square::new(61);
pub const G8: Square = Square::new(62);
pub const H8: Square = Square::new(63);


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn square_displays_correctly() {
        assert_eq!(format!("{}", A1), "a1");
        assert_eq!(format!("{}", A2), "a2");
        assert_eq!(format!("{}", A3), "a3");
        assert_eq!(format!("{}", A4), "a4");
        assert_eq!(format!("{}", A5), "a5");
        assert_eq!(format!("{}", A6), "a6");
        assert_eq!(format!("{}", A7), "a7");
        assert_eq!(format!("{}", A8), "a8");

        assert_eq!(format!("{}", B1), "b1");
        assert_eq!(format!("{}", B2), "b2");
        assert_eq!(format!("{}", B3), "b3");
        assert_eq!(format!("{}", B4), "b4");
        assert_eq!(format!("{}", B5), "b5");
        assert_eq!(format!("{}", B6), "b6");
        assert_eq!(format!("{}", B7), "b7");
        assert_eq!(format!("{}", B8), "b8");

        assert_eq!(format!("{}", C1), "c1");
        assert_eq!(format!("{}", C2), "c2");
        assert_eq!(format!("{}", C3), "c3");
        assert_eq!(format!("{}", C4), "c4");
        assert_eq!(format!("{}", C5), "c5");
        assert_eq!(format!("{}", C6), "c6");
        assert_eq!(format!("{}", C7), "c7");
        assert_eq!(format!("{}", C8), "c8");

        assert_eq!(format!("{}", D1), "d1");
        assert_eq!(format!("{}", D2), "d2");
        assert_eq!(format!("{}", D3), "d3");
        assert_eq!(format!("{}", D4), "d4");
        assert_eq!(format!("{}", D5), "d5");
        assert_eq!(format!("{}", D6), "d6");
        assert_eq!(format!("{}", D7), "d7");
        assert_eq!(format!("{}", D8), "d8");

        assert_eq!(format!("{}", E1), "e1");
        assert_eq!(format!("{}", E2), "e2");
        assert_eq!(format!("{}", E3), "e3");
        assert_eq!(format!("{}", E4), "e4");
        assert_eq!(format!("{}", E5), "e5");
        assert_eq!(format!("{}", E6), "e6");
        assert_eq!(format!("{}", E7), "e7");
        assert_eq!(format!("{}", E8), "e8");

        assert_eq!(format!("{}", F1), "f1");
        assert_eq!(format!("{}", F2), "f2");
        assert_eq!(format!("{}", F3), "f3");
        assert_eq!(format!("{}", F4), "f4");
        assert_eq!(format!("{}", F5), "f5");
        assert_eq!(format!("{}", F6), "f6");
        assert_eq!(format!("{}", F7), "f7");
        assert_eq!(format!("{}", F8), "f8");

        assert_eq!(format!("{}", G1), "g1");
        assert_eq!(format!("{}", G2), "g2");
        assert_eq!(format!("{}", G3), "g3");
        assert_eq!(format!("{}", G4), "g4");
        assert_eq!(format!("{}", G5), "g5");
        assert_eq!(format!("{}", G6), "g6");
        assert_eq!(format!("{}", G7), "g7");
        assert_eq!(format!("{}", G8), "g8");

        assert_eq!(format!("{}", H1), "h1");
        assert_eq!(format!("{}", H2), "h2");
        assert_eq!(format!("{}", H3), "h3");
        assert_eq!(format!("{}", H4), "h4");
        assert_eq!(format!("{}", H5), "h5");
        assert_eq!(format!("{}", H6), "h6");
        assert_eq!(format!("{}", H7), "h7");
        assert_eq!(format!("{}", H8), "h8");
    }
}
