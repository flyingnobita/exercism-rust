use enum_iterator::{all, Sequence};
use int_enum::IntEnum;
// use std::fmt::{self, Debug}; // Iteration 1

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, IntEnum, Sequence)]
pub enum ResistorColor {
    Black = 0,
    Brown = 1,
    Red = 2,
    Orange = 3,
    Yellow = 4,
    Green = 5,
    Blue = 6,
    Violet = 7,
    Grey = 8,
    White = 9,
}

pub fn color_to_value(_color: ResistorColor) -> u32 {
    _color.int_value()
}

// Iteration 1
// impl fmt::Display for ResistorColor {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         fmt::Debug::fmt(self, f)
//     }
// }

pub fn value_to_color_string(value: u32) -> String {
    // let colors = colors();
    // for color in colors {
    //     if color.int_value() == value {
    //         return color.to_string();
    //     }
    // }
    // return String::from("value out of range");

    match ResistorColor::from_int(value) {
        Ok(color) => format!("{:?}", color),
        Err(_) => String::from("value out of range"),
    }
}

pub fn colors() -> Vec<ResistorColor> {
    all::<ResistorColor>().collect()
}
