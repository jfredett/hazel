#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
pub enum File {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
    E = 4,
    F = 5,
    G = 6,
    H = 7
}

pub const FILES : [File; 8] = [ File::A, File::B, File::C, File::D, File::E, File::F, File::G, File::H ];

pub const NOT_A_FILE : u64 = 0xfefefefefefefefe;
pub const NOT_H_FILE : u64 = 0x7f7f7f7f7f7f7f7f;