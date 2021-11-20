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