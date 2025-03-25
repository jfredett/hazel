// These are fine in this module for now. Since a Charray is a collection of bytes, testing fills
// the slots with nulls on initialization, so the literal `\0` being a null character is in fact
// what we want. At some point I'll build a better test macro for this.
#![allow(clippy::octal_escapes)]

use std::fmt::{Debug, Display};

#[derive(Debug, Default, Clone, Copy)]
pub enum Origin {
    #[default] BottomLeft,
    TopLeft,
    TopRight,
    BottomRight,
}

#[derive(Debug, Clone)]
pub struct Charray<const H: usize, const W: usize> {
    origin: Origin,
    data: [[u8; W]; H],
}

impl<const H: usize, const W: usize> Default for Charray<H, W> {
    fn default() -> Self {
        Self::new()
    }
}


impl<const H: usize, const W: usize> Charray<H, W> {
    pub fn new() -> Self {
        Self {
            origin: Origin::BottomLeft,
            data: [[0; W]; H],
        }
    }

    pub fn get(&self, rank: usize, file: usize) -> u8 {
        let (rprime, fprime) = self.adjust_coordinates(rank, file);
        self.data[rprime][fprime]
    }

    pub fn set(&mut self, rank: usize, file: usize, value: u8) {
        let (rprime, fprime) = self.adjust_coordinates(rank, file);
        self.data[rprime][fprime] = value;
    }

    pub fn set_origin(&mut self, origin: Origin) {
        self.origin = origin;
    }

    /* TODO: This really needs to do math on the thing, so probably needs to be an external method
    pub fn transform(&self, origin: Origin) -> Self {
        let mut new = Self {
            origin,
            data: [[0; W]; H],
        };

        for i in 0..H {
            for j in 0..W {
                new.set(i, j, self.get(i, j));
            }
        }

        new
    }
    */

    pub fn with_texture(&self, texture: Vec<&str>) -> Self {
        let mut new = Self {
            origin: Origin::TopLeft, // for textures to load correctly, we need to start at the top
                                     // left
            data: [[0; W]; H],
        };

        for i in 0..H {
            for j in 0..W {
                let b = texture[i].as_bytes()[j];
                new.set(i, j, b);
            }
        }

        new.origin = self.origin; // now we'll non-transformatively switch the origin to what the
                                  // user expects
        new
    }

    fn adjust_coordinates(&self, rank: usize, file: usize) -> (usize, usize) {
        match self.origin {
            Origin::BottomLeft => (H - rank - 1, file),
            Origin::TopLeft => (rank, file),
            Origin::TopRight => (rank, W - file - 1),
            Origin::BottomRight => (H - rank - 1, W - file - 1),
        }
    }
}

impl<const H: usize, const W: usize> Display for Charray<H, W> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..H {
            for j in 0..W {
                write!(f, "{}", self.data[i][j] as char)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod bottomleft_is_default {
        use super::*;

        #[test]
        fn new() {
            let charray = Charray::<3, 3>::new();
            assert_eq!(charray.get(0, 0), 0);
            assert_eq!(charray.get(1, 1), 0);
            assert_eq!(charray.get(2, 2), 0);
        }

        #[test]
        fn to_string() {
            let mut charray = Charray::<3, 3>::new();
            charray.set(0, 0, b'1');
            charray.set(1, 1, b'2');
            charray.set(2, 2, b'3');
            assert_eq!(charray.to_string(), "\0\03\n\02\0\n1\0\0\n");
        }

        #[test]
        fn set() {
            let mut charray = Charray::<3, 3>::new();
            charray.set(0, 0, b'1');
            charray.set(1, 1, b'2');
            charray.set(2, 2, b'3');
            assert_eq!(charray.get(0, 0), b'1');
            assert_eq!(charray.get(1, 1), b'2');
            assert_eq!(charray.get(2, 2), b'3');
        }

        #[test]
        fn display() {
            let mut charray = Charray::<3, 3>::new();
            charray.set(0, 0, b'1');
            charray.set(1, 1, b'2');
            charray.set(2, 2, b'3');
            assert_eq!(format!("{}", charray), "\0\03\n\02\0\n1\0\0\n");
        }
    }

    mod topleft {
        use super::*;

        #[test]
        fn new() {
            let charray = Charray::<3, 3> {
                origin: Origin::TopLeft,
                data: [[0; 3]; 3],
            };
            assert_eq!(charray.get(0, 0), 0);
            assert_eq!(charray.get(1, 1), 0);
            assert_eq!(charray.get(2, 2), 0);
        }

        #[test]
        fn to_string() {
            let mut charray = Charray::<3, 3> {
                origin: Origin::TopLeft,
                data: [[0; 3]; 3],
            };
            charray.set(0, 0, b'1');
            charray.set(1, 1, b'2');
            charray.set(2, 2, b'3');
            assert_eq!(charray.to_string(), "1\0\0\n\02\0\n\0\03\n");
        }

        #[test]
        fn set() {
            let mut charray = Charray::<3, 3> {
                origin: Origin::TopLeft,
                data: [[0; 3]; 3],
            };
            charray.set(0, 0, 1);
            charray.set(1, 1, 2);
            charray.set(2, 2, 3);
            assert_eq!(charray.get(0, 0), 1);
            assert_eq!(charray.get(1, 1), 2);
            assert_eq!(charray.get(2, 2), 3);
        }

        #[test]
        fn display() {
            let mut charray = Charray::<3, 3> {
                origin: Origin::TopLeft,
                data: [[0; 3]; 3],
            };
            charray.set(0, 0, b'1');
            charray.set(1, 1, b'2');
            charray.set(2, 2, b'3');
            assert_eq!(format!("{}", charray), "1\0\0\n\02\0\n\0\03\n");
        }
    }

    mod bottomright {
        use super::*;

        #[test]
        fn new() {
            let charray = Charray::<3, 3> {
                origin: Origin::BottomRight,
                data: [[0; 3]; 3],
            };
            assert_eq!(charray.get(0, 0), 0);
            assert_eq!(charray.get(1, 1), 0);
            assert_eq!(charray.get(2, 2), 0);
        }

        #[test]
        fn to_string() {
            let mut charray = Charray::<3, 3> {
                origin: Origin::BottomRight,
                data: [[0; 3]; 3],
            };
            charray.set(0, 0, b'1');
            charray.set(1, 1, b'2');
            charray.set(2, 2, b'3');
            assert_eq!(charray.to_string(), "3\0\0\n\02\0\n\0\01\n");
        }

        #[test]
        fn set() {
            let mut charray = Charray::<3, 3> {
                origin: Origin::BottomRight,
                data: [[0; 3]; 3],
            };
            charray.set(0, 0, 1);
            charray.set(1, 1, 2);
            charray.set(2, 2, 3);
            assert_eq!(charray.get(0, 0), 1);
            assert_eq!(charray.get(1, 1), 2);
            assert_eq!(charray.get(2, 2), 3);
        }

        #[test]
        fn display() {
            let mut charray = Charray::<3, 3> {
                origin: Origin::BottomRight,
                data: [[0; 3]; 3],
            };
            charray.set(0, 0, b'1');
            charray.set(1, 1, b'2');
            charray.set(2, 2, b'3');
            assert_eq!(format!("{}", charray), "3\0\0\n\02\0\n\0\01\n");
        }
    }

    mod topright {
        use super::*;

        #[test]
        fn new() {
            let charray = Charray::<3, 3> {
                origin: Origin::TopRight,
                data: [[0; 3]; 3],
            };
            assert_eq!(charray.get(0, 0), 0);
            assert_eq!(charray.get(1, 1), 0);
            assert_eq!(charray.get(2, 2), 0);
        }

        #[test]
        fn to_string() {
            let mut charray = Charray::<3, 3> {
                origin: Origin::TopRight,
                data: [[0; 3]; 3],
            };
            charray.set(0, 0, b'1');
            charray.set(1, 1, b'2');
            charray.set(2, 2, b'3');
            assert_eq!(charray.to_string(), "\0\01\n\02\0\n3\0\0\n");
        }

        #[test]
        fn set() {
            let mut charray = Charray::<3, 3> {
                origin: Origin::TopRight,
                data: [[0; 3]; 3],
            };
            charray.set(0, 0, 1);
            charray.set(1, 1, 2);
            charray.set(2, 2, 3);
            assert_eq!(charray.get(0, 0), 1);
            assert_eq!(charray.get(1, 1), 2);
            assert_eq!(charray.get(2, 2), 3);
        }

        #[test]
        fn display() {
            let mut charray = Charray::<3, 3> {
                origin: Origin::TopRight,
                data: [[0; 3]; 3],
            };
            charray.set(0, 0, b'1');
            charray.set(1, 1, b'2');
            charray.set(2, 2, b'3');
            assert_eq!(format!("{}", charray), "\0\01\n\02\0\n3\0\0\n");
        }
    }


    #[test]
    fn effect_of_directly_changing_origin_after_the_fact_is_spooky() {
        // NOTE: Changing the Origin without transforming the underlying data essentially rotates
        // the frame without rotating the contents. This may, in some cases, be useful, but I
        // don't think it is to me right now, and generally a transform is preferable, so that's
        // all I've implemented. This test demonstrates the spooky behavior.
        
        // Here's an array like:
        //
        //```
        //  003
        //  020
        //  100
        // /
        // ```
        //
        // Charrays have origin at the bottom left by default, as indicated by the `/`
        let mut charray = Charray::<3, 3>::new();
        charray.set(0, 0, 1);
        charray.set(1, 1, 2);
        charray.set(2, 2, 3);
        assert_eq!(charray.get(0, 0), 1);
        assert_eq!(charray.get(1, 1), 2);
        assert_eq!(charray.get(2, 2), 3);
        let before = charray.to_string();

        // If we set without transformation. The array is now:
        //
        // ```
        //     /
        //  003
        //  020
        //  100
        // ```
        //
        // so the origin is no longer going to return the same value.
        charray.origin = Origin::TopRight;
        assert_eq!(charray.get(0, 0), 3);
        assert_eq!(charray.get(1, 1), 2);
        assert_eq!(charray.get(2, 2), 1);

        // The spookiness comes when you write to a string, the values are the same, even though
        // the origin changed! That's because we always right with the origin in the _top left_, as
        // that's the most natural representation with a 2D array.
        let after = charray.to_string();

        assert_eq!(before, after);
    }

    /*
    mod transform {
        use super::*;

        fn transform_test(start: Origin, dest: Origin, prediction: &str) {
            let mut charray = Charray::<4, 3>::new().transform(start);


            charray.set(0, 0, b'1');
            charray.set(1, 1, b'2');
            charray.set(2, 2, b'3');
            charray.set(3, 0, b'4');

            assert_eq!(charray.get(0, 0), b'1');
            assert_eq!(charray.get(1, 1), b'2');
            assert_eq!(charray.get(2, 2), b'3');
            assert_eq!(charray.get(3, 0), b'4');

            let before = charray.to_string();

            let transformed = charray.transform(dest);
            let after = transformed.to_string();


            assert_eq!(charray.get(0, 0), b'1');
            assert_eq!(charray.get(1, 1), b'2');
            assert_eq!(charray.get(2, 2), b'3');
            assert_eq!(charray.get(3, 0), b'4');

            assert_ne!(before, after);
            assert_eq!(after, prediction);
        }

        #[test]
        fn bottomleft_to_topleft() {
            transform_test(Origin::BottomLeft, Origin::TopLeft, "1\0\0\0\n\02\0\0\n\0\03\0\n\0\0\04\n");
        }

        #[test]
        fn bottomleft_to_bottomright() {
            transform_test(Origin::BottomLeft, Origin::BottomRight, "\0\0\01\n\0\02\0\n\03\0\0\n4\0\0\0\n");
        }

        #[test]
        fn bottomleft_to_topright() {
            transform_test(Origin::BottomLeft, Origin::TopRight, "\0\0\01\n\0\02\0\n\03\0\0\n4\0\0\0\n");
        }

        #[test]
        fn bottomleft_to_bottomleft() {
            transform_test(Origin::BottomLeft, Origin::BottomLeft, "\0\03\0\n\02\0\0\n1\0\0\0\n\0\0\04\n");
        }

    }
    */

    mod with_texture {
        use super::*;

        #[test]
        fn bottomleft() {
            let texture = vec![
                "123",
                "456",
                "789",
            ];

            let charray = Charray::<3, 3>::new().with_texture(texture);


            assert_eq!(charray.get(0, 0), b'7');
            assert_eq!(charray.get(1, 1), b'5');
            assert_eq!(charray.get(2, 2), b'3');
        }

        #[test]
        fn topleft() {
            let texture = vec![
                "123",
                "456",
                "789",
            ];

            let charray = Charray::<3, 3> {
                origin: Origin::TopLeft,
                data: [[0; 3]; 3],
            }.with_texture(texture);

            assert_eq!(charray.get(0, 0), b'1');
            assert_eq!(charray.get(1, 1), b'5');
            assert_eq!(charray.get(2, 2), b'9');
        }

        #[test]
        fn bottomright() {
            let texture = vec![
                "123",
                "456",
                "789",
            ];

            let charray = Charray::<3, 3> {
                origin: Origin::BottomRight,
                data: [[0; 3]; 3],
            }.with_texture(texture);

            assert_eq!(charray.get(0, 0), b'9');
            assert_eq!(charray.get(1, 1), b'5');
            assert_eq!(charray.get(2, 2), b'1');
        }

        #[test]
        fn topright() {
            let texture = vec![
                "123",
                "456",
                "789",
            ];

            let charray = Charray::<3, 3> {
                origin: Origin::TopRight,
                data: [[0; 3]; 3],
            }.with_texture(texture);


            assert_eq!(charray.get(0, 0), b'3');
            assert_eq!(charray.get(1, 1), b'5');
            assert_eq!(charray.get(2, 2), b'7');
        }
    }
}
