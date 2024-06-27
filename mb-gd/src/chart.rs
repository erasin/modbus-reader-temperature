use godot::{
    engine::{Control, IControl, InputEvent, InputEventMouseMotion},
    obj::WithBaseField,
    prelude::*,
};

use crate::colors::ColorPlate;

#[derive(GodotClass)]
#[class(base=Control)]
pub struct ChartView {
    points: Vec<Vector2>,
    color: Color,
    background_color: Color,
    grid_color: Color,
    x_labels: Vec<f32>,
    y_labels: Vec<f32>,
    x_labels_color: Color,
    y_labels_color: Color,

    max_x: f32,
    max_y: f32,
    hovered_point: Option<(Vector2, String)>,

    base: Base<Control>,
}

#[godot_api]
impl IControl for ChartView {
    fn init(base: Base<Control>) -> Self {
        Self {
            points: Vec::new(),
            color: ColorPlate::White.into(),
            background_color: ColorPlate::Black.into(),
            grid_color: ColorPlate::Grey8.into(),
            x_labels: Vec::new(),
            y_labels: Vec::new(),
            x_labels_color: ColorPlate::LightGreen.into(),
            y_labels_color: ColorPlate::LightBlue.into(),
            max_x: 0.,
            max_y: 0.,
            hovered_point: None,
            base,
        }
    }

    fn draw(&mut self) {
        let size = self.base().get_size();

        let bg = self.background_color;
        self.base_mut()
            .draw_rect(Rect2::new(Vector2::new(0.0, 0.0), size), bg);

        self.draw_grid_lines();
        self.draw_labels();

        if self.points.len() < 2 {
            return;
        }

        for i in 0..self.points.len() - 1 {
            let p1 = self.convert_point(self.points[i]);
            let p2 = self.convert_point(self.points[i + 1]);
            let color = self.color;
            self.base_mut()
                .draw_line_ex(p1, p2, color)
                .width(4.0)
                .done();
        }

        // 数据原点
        for i in 0..self.points.len() {
            let p1 = self.convert_point(self.points[i]);
            let color = self.grid_color;
            self.base_mut().draw_circle(p1, 5., color);
        }
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        let size = self.base().get_size();
        let pos = self.base().get_position();

        match event.try_cast::<InputEventMouseMotion>() {
            Ok(mouse_event) => {
                let mouse_position = mouse_event.get_position();
                self.hovered_point = None; // Reset hovered point

                // Check if mouse is hovering over any point
                for &point in &self.points {
                    let screen_point = pos + Vector2::new(point.x, size.y - point.y);
                    if mouse_position.distance_to(screen_point) < 5.0 {
                        let value = format!("({}, {})", point.x, point.y);
                        self.hovered_point = Some((screen_point, value));
                        break;
                    }
                }
            }
            Err(_) => return,
        }
    }
}

impl ChartView {
    pub fn set_points(&mut self, points: Vec<Vector2>) {
        self.points = points;
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    pub fn set_background_color(&mut self, background_color: Color) {
        self.background_color = background_color;
    }

    pub fn set_grid_color(&mut self, grid_color: Color) {
        self.grid_color = grid_color;
    }

    pub fn set_x_labels(&mut self, x_labels: Vec<f32>) {
        self.x_labels = x_labels;
        self.max_x = self.x_labels.iter().cloned().fold(0. / 0., f32::max);
    }

    pub fn set_y_labels(&mut self, y_labels: Vec<f32>) {
        self.y_labels = y_labels;
        self.max_y = self.y_labels.iter().cloned().fold(0. / 0., f32::max);
    }

    pub fn set_x_labels_color(&mut self, color: Color) {
        self.x_labels_color = color;
    }

    pub fn set_y_labels_color(&mut self, color: Color) {
        self.y_labels_color = color;
    }

    fn draw_grid_lines(&mut self) {
        let size = self.base().get_size();
        let step_x = size.x / 10.0;
        let step_y = size.y / 10.0;
        let grid_color = self.grid_color;

        for i in 0..=10 {
            // Vertical grid lines
            let x = i as f32 * step_x;
            self.base_mut().draw_dashed_line(
                Vector2::new(x, 0.0),
                Vector2::new(x, size.y),
                grid_color,
            );

            // Horizontal grid lines
            let y = i as f32 * step_y;
            self.base_mut().draw_dashed_line(
                Vector2::new(0.0, y),
                Vector2::new(size.x, y),
                grid_color,
            );
        }
    }

    fn draw_labels(&mut self) {
        let font = self
            .base()
            .get_theme_font("HarmonyOS Sans SC Regular".into())
            .unwrap();

        let size = self.base().get_size();
        let x_labels = self.x_labels.clone();
        let y_labels = self.y_labels.clone();
        let x_color = self.x_labels_color;
        let y_color = self.y_labels_color;

        // Draw x-axis labels
        for x in x_labels {
            let point = self.convert_point(Vector2::new(x, 0.0));
            self.base_mut()
                .draw_string(font.clone(), point, x.to_string().into());

            let p2 = self.convert_point(Vector2::new(x, self.max_y));
            self.base_mut().draw_dashed_line(point, p2, x_color);
        }

        // Draw y-axis labels
        for y in y_labels {
            let point = self.convert_point(Vector2::new(0.0, y));
            self.base_mut()
                .draw_string(font.clone(), point, y.to_string().into());
            let p2 = self.convert_point(Vector2::new(self.max_x, y));

            self.base_mut().draw_dashed_line(point, p2, y_color);
        }
    }

    fn convert_point(&self, point: Vector2) -> Vector2 {
        let size = self.base().get_size();

        let x = (point.x / self.max_x) * size.x;
        let y = size.y - (point.y / self.max_y) * size.y;

        Vector2::new(x, y)
    }
}
