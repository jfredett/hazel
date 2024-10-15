use std::fmt::{Debug, Display};

#[derive(Debug, Clone, Copy)]
enum Origin {
    BottomLeft,
    TopLeft,
    TopRight,
    BottomRight,
}

#[derive(Clone)]
struct Charray<const H: usize, const W: usize> {
    origin: Origin,
    data: [[u8; W]; H],
}

impl Debug for Charray<3, 3> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "\n{}\n", self.to_string())
    }
}

impl<const H: usize, const W: usize> Charray<H, W> {
    pub fn new() -> Self {
        Self {
            origin: Origin::BottomLeft,
            data: [[0; W]; H],
        }
    }

    pub fn to_string(&self) -> String {
        format!("{}", self)
    }

    pub fn get(&self, x: usize, y: usize) -> u8 {
        let (xprime, yprime) = self.origin.adjust_coordinates(x, y, H, W);
        self.data[xprime][yprime]
    }

    pub fn set(&mut self, x: usize, y: usize, value: u8) {
        let (xprime, yprime) = self.origin.adjust_coordinates(x, y, H, W);
        self.data[xprime][yprime] = value;
    }

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

    pub fn with_texture(&self, texture: Vec<&str>) -> Self {
        let mut new = Self {
            origin: self.origin,
            data: [[0; W]; H],
        };

        for i in 0..H {
            for j in 0..W {
                new.set(i, j, texture[i].as_bytes()[j] - b'0');
            }
        }

        new
    }
}


impl Origin {
    fn adjust_coordinates(&self, x: usize, y: usize, h: usize, w: usize) -> (usize, usize) {
        match self {
            Origin::BottomLeft => (w - x - 1, y),
            Origin::TopLeft => (x, y),
            Origin::TopRight => (x, h - y - 1),
            Origin::BottomRight => (w - x - 1, h - y - 1),
        }
    }

}

impl<const H: usize, const W: usize> Display for Charray<H, W> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..H {
            for j in 0..W {
                write!(f, "{}", self.data[i][j])?;
            }
            write!(f, "\n")?;
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
            charray.set(0, 0, 1);
            charray.set(1, 1, 2);
            charray.set(2, 2, 3);
            dbg!(charray.to_string());
            assert_eq!(charray.to_string(), "003\n020\n100\n");
        }

        #[test]
        fn set() {
            let mut charray = Charray::<3, 3>::new();
            charray.set(0, 0, 1);
            charray.set(1, 1, 2);
            charray.set(2, 2, 3);
            assert_eq!(charray.get(0, 0), 1);
            assert_eq!(charray.get(1, 1), 2);
            assert_eq!(charray.get(2, 2), 3);
        }

        #[test]
        fn display() {
            let mut charray = Charray::<3, 3>::new();
            charray.set(0, 0, 1);
            charray.set(1, 1, 2);
            charray.set(2, 2, 3);
            assert_eq!(format!("{}", charray), "003\n020\n100\n");
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
            charray.set(0, 0, 1);
            charray.set(1, 1, 2);
            charray.set(2, 2, 3);
            assert_eq!(charray.to_string(), "100\n020\n003\n");
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
            charray.set(0, 0, 1);
            charray.set(1, 1, 2);
            charray.set(2, 2, 3);
            assert_eq!(format!("{}", charray), "100\n020\n003\n");
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
            charray.set(0, 0, 1);
            charray.set(1, 1, 2);
            charray.set(2, 2, 3);
            assert_eq!(charray.to_string(), "300\n020\n001\n");
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
            charray.set(0, 0, 1);
            charray.set(1, 1, 2);
            charray.set(2, 2, 3);
            assert_eq!(format!("{}", charray), "300\n020\n001\n");
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
            charray.set(0, 0, 1);
            charray.set(1, 1, 2);
            charray.set(2, 2, 3);
            assert_eq!(charray.to_string(), "001\n020\n300\n");
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
            charray.set(0, 0, 1);
            charray.set(1, 1, 2);
            charray.set(2, 2, 3);
            assert_eq!(format!("{}", charray), "001\n020\n300\n");
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

    mod transform {
        use super::*;

        #[test]
        fn bottomleft_to_topleft() {
            let mut charray = Charray::<3, 3>::new();
            charray.set(0, 0, 1);
            charray.set(1, 1, 2);
            charray.set(2, 2, 3);

            assert_eq!(charray.get(0, 0), 1);
            assert_eq!(charray.get(1, 1), 2);
            assert_eq!(charray.get(2, 2), 3);

            let before = charray.to_string();

            let expected = "100\n020\n003\n";

            let transformed = charray.transform(Origin::TopLeft);
            let after = transformed.to_string();

            assert_eq!(charray.get(0, 0), 1);
            assert_eq!(charray.get(1, 1), 2);
            assert_eq!(charray.get(2, 2), 3);

            assert_ne!(before, after);
            assert_eq!(after, expected);
        }

        #[test]
        fn bottomleft_to_bottomright() {
            let mut charray = Charray::<3, 3>::new();
            charray.set(0, 0, 1);
            charray.set(1, 1, 2);
            charray.set(2, 2, 3);

            assert_eq!(charray.get(0, 0), 1);
            assert_eq!(charray.get(1, 1), 2);
            assert_eq!(charray.get(2, 2), 3);

            let before = charray.to_string();

            let expected = "300\n020\n001\n";

            let transformed = charray.transform(Origin::BottomRight);
            let after = transformed.to_string();

            assert_eq!(charray.get(0, 0), 1);
            assert_eq!(charray.get(1, 1), 2);
            assert_eq!(charray.get(2, 2), 3);

            assert_ne!(before, after);
            assert_eq!(after, expected);
        }

        #[test]
        fn bottomleft_to_topright() {
            let mut charray = Charray::<3, 3>::new();
            charray.set(0, 0, 1);
            charray.set(1, 1, 2);
            charray.set(2, 2, 3);

            assert_eq!(charray.get(0, 0), 1);
            assert_eq!(charray.get(1, 1), 2);
            assert_eq!(charray.get(2, 2), 3);

            let before = charray.to_string();

            let expected = "001\n020\n300\n";

            let transformed = charray.transform(Origin::TopRight);
            let after = transformed.to_string();

            assert_eq!(charray.get(0, 0), 1);
            assert_eq!(charray.get(1, 1), 2);
            assert_eq!(charray.get(2, 2), 3);

            assert_ne!(before, after);
            assert_eq!(after, expected);
        }

        #[test]
        fn topleft_to_bottomleft() {
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

            let before = charray.to_string();

            let expected = "003\n020\n100\n";

            let transformed = charray.transform(Origin::BottomLeft);
            let after = transformed.to_string();

            assert_eq!(charray.get(0, 0), 1);
            assert_eq!(charray.get(1, 1), 2);
            assert_eq!(charray.get(2, 2), 3);

            assert_ne!(before, after);
            assert_eq!(after, expected);
        }
    }

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

            assert_eq!(charray.get(0, 0), 1);
            assert_eq!(charray.get(1, 1), 5);
            assert_eq!(charray.get(2, 2), 9);
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

            assert_eq!(charray.get(0, 0), 1);
            assert_eq!(charray.get(1, 1), 5);
            assert_eq!(charray.get(2, 2), 9);
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

            assert_eq!(charray.get(0, 0), 1);
            assert_eq!(charray.get(1, 1), 5);
            assert_eq!(charray.get(2, 2), 9);
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

            assert_eq!(charray.get(0, 0), 1);
            assert_eq!(charray.get(1, 1), 5);
            assert_eq!(charray.get(2, 2), 9);
        }
    }
}
