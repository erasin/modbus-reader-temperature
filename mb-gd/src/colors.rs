use godot::builtin::Color;
use mb::voltage::VoltageState;

pub const BG: Color = Color::from_rgb(236.0, 239.0, 241.0);
pub const BLACK: Color = Color::from_rgb(33.0, 33.0, 33.0);
pub const WHITE: Color = Color::from_rgb(250.0, 250.0, 250.0);
pub const GREY: Color = Color::from_rgb(158.0, 158.0, 158.0);
pub const RED: Color = Color::from_rgb(244.0, 67.0, 54.0);
pub const BLUE: Color = Color::from_rgb(33.0, 150.0, 243.0);
pub const YELLOW: Color = Color::from_rgb(255.0, 235.0, 59.0);
pub const GREEN: Color = Color::from_rgb(76.0, 175.0, 80.0);
pub const PUPLE: Color = Color::from_rgb(156.0, 39.0, 176.0);
pub const CYAN: Color = Color::from_rgb(0.0, 188.0, 212.0);
pub const LIGHT_GREEN: Color = Color::from_rgb(139.0, 195.0, 74.0);
pub const LIGHT_BLUE: Color = Color::from_rgb(3.0, 169.0, 244.0);
pub const AMBER: Color = Color::from_rgb(255.0, 193.0, 7.0);
pub const TEAL: Color = Color::from_rgb(0.0, 150.0, 136.0);

/// 输出 color
pub trait Style {
    /// bg, fg
    fn style(&self) -> (Color, Color);
}

impl Style for VoltageState {
    fn style(&self) -> (Color, Color) {
        match self {
            VoltageState::NoConnected => (BLACK, GREY),
            VoltageState::Vacancy => (BLACK, WHITE),
            VoltageState::Qualified => (BLACK, GREEN),
            VoltageState::UnderVoltage => (BLACK, RED),
            VoltageState::UnderCurrent => (BLACK, YELLOW),
            VoltageState::OverVoltage => (BLACK, BLUE),
            VoltageState::OverCurrent => (BLACK, PUPLE),
            VoltageState::NoOutput => (BLACK, CYAN),
        }
    }
}
