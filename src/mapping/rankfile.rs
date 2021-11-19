use crate::mapping::Square;

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