// LERF mapping
// A1 = 0, H1 = 7, A8 = 56, H8 = 63
pub const FIRST_RANK: u64 = 0x00000000000000FF;
pub const A_FILE: u64 = 0x0101010101010101;
pub const MAIN_DIAG: u64 = 0x8040201008040201;
pub const MAIN_ANTIDIAG: u64 = 0x0102040810204080;


#[derive(Copy, Clone, PartialOrd, PartialEq)]
pub enum Square {
    A1, B1, C1, D1, E1, F1, G1, H1,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A8, B8, C8, D8, E8, F8, G8, H8
}

impl From<u8> for Square {
    fn from(square_val: u8) -> Self {
        match square_val {
            0 => Square::A1,
            1 => Square::B1,
            2 => Square::C1,
            3 => Square::D1,
            4 => Square::E1,
            5 => Square::F1,
            6 => Square::G1,
            7 => Square::H1,

            8 => Square::A2,
            9 => Square::B2,
            10 => Square::C2,
            11 => Square::D2,
            12 => Square::E2,
            13 => Square::F2,
            14 => Square::G2,
            15 => Square::H2,

            16 => Square::A3,
            17 => Square::B3,
            18 => Square::C3,
            19 => Square::D3,
            20 => Square::E3,
            21 => Square::F3,
            22 => Square::G3,
            23 => Square::H3,

            24 => Square::A4,
            25 => Square::B4,
            26 => Square::C4,
            27 => Square::D4,
            28 => Square::E4,
            29 => Square::F4,
            30 => Square::G4,
            31 => Square::H4,

            32 => Square::A5,
            33 => Square::B5,
            34 => Square::C5,
            35 => Square::D5,
            36 => Square::E5,
            37 => Square::F5,
            38 => Square::G5,
            39 => Square::H5,

            40 => Square::A6,
            41 => Square::B6,
            42 => Square::C6,
            43 => Square::D6,
            44 => Square::E6,
            45 => Square::F6,
            46 => Square::G6,
            47 => Square::H6,

            48 => Square::A7,
            49 => Square::B7,
            50 => Square::C7,
            51 => Square::D7,
            52 => Square::E7,
            53 => Square::F7,
            54 => Square::G7,
            55 => Square::H7,

            56 => Square::A8,
            57 => Square::B8,
            58 => Square::C8,
            59 => Square::D8,
            60 => Square::E8,
            61 => Square::F8,
            62 => Square::G8,
            63 => Square::H8,

            _ => panic!("failed to convert from RankFile to Square")
        }
    }
}

impl From<RankFile> for Square {
    fn from(rf: RankFile) -> Self {
        let file_val = rf.file as u8;
        let square_val = 8 * rf.rank + file_val;
        Square::from(square_val)
    }
}

pub enum File {
    A, B, C, D, E, F, G, H
}

impl From<u8> for File {
    fn from(file_num: u8) -> Self {
        match file_num {
            0 => File::A,
            1 => File::B,
            2 => File::C,
            3 => File::D,
            4 => File::E,
            5 => File::F,
            6 => File::G,
            7 => File::H,
            _ => panic!("failed to convert from u8 to File")
        }
    }
}

pub(crate) struct RankFile {
    pub(crate) rank: u8,
    pub(crate) file: File
}

impl From<Square> for RankFile {
    fn from(square: Square) -> Self {
        let square_val = square as u8;
        let file_val = square_val & 7;
        let rank_val = square_val >> 3;

        let determined_file = File::from(file_val);
        RankFile {
            file: determined_file,
            rank: rank_val,
        }
    }
}