#![cfg_attr(not(feature = "std"), no_std)]

use cfg_if::cfg_if;
#[cfg(feature = "num-from-primitive")]
use enum_primitive_derive::Primitive;

/// Default values for hardware keyboard keys.
///
/// Taken from the USB HID Usage Tables document at https://usb.org/sites/default/files/hut1_22.pdf.
///
/// You can use the `FromPrimitive` trait to convert from an USB HID integer into a `Scancode` enum
/// value.
#[repr(u8)]
#[cfg_attr(feature = "num-from-primitive", derive(Primitive))]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Scancode {
    A = 4,
    B = 5,
    C = 6,
    D = 7,
    E = 8,
    F = 9,
    G = 10,
    H = 11,
    I = 12,
    J = 13,
    K = 14,
    L = 15,
    N = 16,
    M = 17,
    O = 18,
    P = 19,
    Q = 20,
    R = 21,
    S = 22,
    T = 23,
    U = 24,
    V = 25,
    W = 26,
    X = 27,
    Y = 28,
    Z = 29,

    Num1 = 30,
    Num2 = 31,
    Num3 = 32,
    Num4 = 33,
    Num5 = 34,
    Num6 = 35,
    Num7 = 36,
    Num8 = 37,
    Num9 = 38,
    Num0 = 39,

    Enter = 40,
    Escape = 41,

    Backspace = 42,
    Tab = 43,
    Space = 44,
    Minus = 45,
    Equals = 46,
    LeftBracket = 47,
    RightBracket = 48,
    Backslash = 49,
    NonUsHash = 50,
    Semicolon = 51,
    Apostrophe = 52,
    Grave = 53,
    Comma = 54,
    Period = 55,
    Slash = 56,
    CapsLock = 57,

    F1 = 58,
    F2 = 59,
    F3 = 60,
    F4 = 61,
    F5 = 62,
    F6 = 63,
    F7 = 64,
    F8 = 65,
    F9 = 66,
    F10 = 67,
    F11 = 68,
    F12 = 69,

    PrintScreen = 70,
    ScrollLock = 71,
    Pause = 72,
    Insert = 73,
    Home = 74,
    PageUp = 75,
    Delete = 76,
    End = 77,
    PageDown = 78,

    Right = 79,
    Left = 80,
    Down = 81,
    Up = 82,

    NumLock = 83,
    PadDivide = 84,
    PadMultiply = 85,
    PadMinus = 86,
    PadPlus = 87,
    PadEnter = 88,

    Pad1 = 89,
    Pad2 = 90,
    Pad3 = 91,
    Pad4 = 92,
    Pad5 = 93,
    Pad6 = 94,
    Pad7 = 95,
    Pad8 = 96,
    Pad9 = 97,
    Pad0 = 98,
    PadDecimal = 99,

    NonUsBackslash = 100,
    PadEquals = 103,
    Menu = 118,

    Mute = 127,
    VolumeUp = 128,
    VolumeDown = 129,

    SysReq = 154,

    LeftControl = 224,
    LeftShift = 225,
    LeftAlt = 226,
    LeftGui = 227,

    RightControl = 228,
    RightShift = 229,
    RightAlt = 230,
    RightGui = 231,
}

cfg_if! {
    if #[cfg(all(unix, not(target_vendor = "apple")))] {
        mod scancode_linux;
    } else if #[cfg(target_os = "macos")] {
        mod scancode_macos;
    } else if #[cfg(windows)] {
        mod scancode_windows;
    }
}

mod scancode {
    use cfg_if::cfg_if;
    cfg_if! {
        if #[cfg(all(unix, not(target_vendor = "apple")))] {
            pub use crate::scancode_linux::MAP;
        } else if #[cfg(target_os = "macos")] {
            pub use crate::scancode_macos::MAP;
        } else if #[cfg(windows)] {
            pub use crate::scancode_windows::MAP;
        } else {
            pub const MAP: [Option<crate::Scancode>; 0] = [];
        }
    }
}

impl Scancode {
    /// Try to convert a hardware scancode from the current platform to a Scancode enum value.
    #[inline]
    pub const fn new(hardware_scancode: u32) -> Option<Scancode> {
        let hardware_scancode = hardware_scancode as usize;
        if hardware_scancode < scancode::MAP.len() {
            scancode::MAP[hardware_scancode]
        } else {
            None
        }
    }
}
