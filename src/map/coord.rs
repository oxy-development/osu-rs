/*
    By default struct contains only in-game coords.
    All real windowed pos is calculating
    via functions get_real_pos

    @TODO: move to another mod, cause its not a part of beatmap
*/

pub const MAX_X = 512;
pub const MAX_Y = 384;

struct Coord {
    x: u8,
    y: u8,
    x_offset: u8,
    y_offset: u8,
    w_container: u8,
    h_container: u8,
    x_multiplier: f32,
    y_multiplier: f32
}

impl Coord {
    pub fn new(x: u8, y: u8, width: u8, height: u8) -> Coord {
        let mut s_width = width;
        let mut s_height = height;

        if s_width * 3 > s_height * 4 {
			s_width = s_height * 4 / 3;
		} else {
			s_height = s_width * 3 / 4;
        }

        Coord {
            x: x,
            y: y,
            w_container: s_width,
            h_container: s_height,
            x_multiplier: s_width / 640,
            y_multiplier: s_height / 480,
            x_offset: ((width - MAX_X * x_multiplier) / 2).round() as u8,
            y_offset: ((height - MAX_Y * y_multiplier) / 2).round() as u8
        }
    }

    pub fn get_real_pos(&self) -> Coord {

    }
}
