use godot::builtin::Color;
use mb::voltage::VoltageState;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ColorPlate {
    Bg,
    Black,
    #[default]
    White,
    Grey,
    Grey8,
    Red,
    Blue,
    Yellow,
    Green,
    Puple,
    Cyan,
    LightGreen,
    LightBlue,
    Amber,
    Teal,
}

impl Into<Color> for ColorPlate {
    fn into(self) -> Color {
        match self {
            ColorPlate::Bg => Color::from_rgba8(236, 239, 241, 255),
            ColorPlate::Black => Color::from_rgba8(33, 33, 33, 255),
            ColorPlate::White => Color::from_rgba8(250, 250, 250, 255),
            ColorPlate::Grey => Color::from_rgba8(158, 158, 158, 255),
            ColorPlate::Grey8 => Color::from_rgba8(66, 66, 66, 255),
            ColorPlate::Red => Color::from_rgba8(244, 67, 54, 255),
            ColorPlate::Blue => Color::from_rgba8(33, 150, 243, 255),
            ColorPlate::Yellow => Color::from_rgba8(255, 235, 59, 255),
            ColorPlate::Green => Color::from_rgba8(76, 175, 80, 255),
            ColorPlate::Puple => Color::from_rgba8(156, 39, 176, 255),
            ColorPlate::Cyan => Color::from_rgba8(0, 188, 212, 255),
            ColorPlate::LightGreen => Color::from_rgba8(139, 195, 74, 255),
            ColorPlate::LightBlue => Color::from_rgba8(3, 169, 244, 255),
            ColorPlate::Amber => Color::from_rgba8(255, 193, 7, 255),
            ColorPlate::Teal => Color::from_rgba8(0, 150, 136, 255),
        }
    }
}

/// 输出 color
pub trait Style {
    /// bg, fg
    fn style(&self) -> Color;
}

impl Style for VoltageState {
    fn style(&self) -> Color {
        match self {
            VoltageState::NoConnected => ColorPlate::Grey.into(),
            VoltageState::Vacancy => ColorPlate::White.into(),
            VoltageState::Qualified => ColorPlate::Green.into(),
            VoltageState::UnderVoltage => ColorPlate::Red.into(),
            VoltageState::UnderCurrent => ColorPlate::Yellow.into(),
            VoltageState::OverVoltage => ColorPlate::Blue.into(),
            VoltageState::OverCurrent => ColorPlate::Puple.into(),
            VoltageState::NoOutput => ColorPlate::Cyan.into(),
        }
    }
}
