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
    point_color: Color,
    line_color: Color,
    background_color: Color,
    grid_color: Color,
    x_coord: Vec<f32>,
    y_coord: Vec<f32>,
    x_coord_color: Color,
    y_coord_color: Color,
    x_label: String,
    y_label: String,
    label_color: Color,

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
            point_color: ColorPlate::Blue.into(),
            line_color: ColorPlate::White.into(),
            background_color: ColorPlate::Black.into(),
            grid_color: ColorPlate::Grey8.into(),
            x_coord: Vec::new(),
            y_coord: Vec::new(),
            x_coord_color: ColorPlate::LightGreen.into(),
            y_coord_color: ColorPlate::LightBlue.into(),
            x_label: String::new(),
            y_label: String::new(),
            label_color: ColorPlate::White.into(),

            max_x: 0.,
            max_y: 0.,
            hovered_point: None,
            base,
        }
    }

    fn draw(&mut self) {
        self.draw_grid_lines();
        self.draw_labels();
        self.draw_points();
        self.draw_hover();
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        let size = self.base().get_size();
        let pos = self.base().get_global_position();
        // self.base().get_global_transform().origin;

        if let Ok(mouse_event) = event.try_cast::<InputEventMouseMotion>() {
            let mouse_position = mouse_event.get_position();
            self.hovered_point = None; // Reset hovered point

            // Check if mouse is hovering over any point
            for &point in &self.points {
                let point2 = self.convert_point(point);
                let screen_point = pos + point2;
                // godot_print!(
                //     "mg:{mouse_position:?}\np1:{point:?}\np2:{point2:?}\nsp:{screen_point:?}"
                // );
                if mouse_position.distance_to(screen_point) < 95.0 {
                    // godot_print!("---dis---");
                    let value = format!("({}, {})", point.x, point.y);
                    self.hovered_point = Some((point, value));
                    break;
                }
            }
        }
    }
}

impl ChartView {
    pub fn set_points(&mut self, points: Vec<Vector2>) {
        self.points = points;
    }

    pub fn set_color(&mut self, color: Color) {
        self.line_color = color;
    }

    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
    }

    pub fn set_grid_color(&mut self, color: Color) {
        self.grid_color = color;
    }

    pub fn set_x_coord(&mut self, coord: Vec<f32>) {
        self.x_coord = coord;
        self.max_x = self.x_coord.iter().cloned().fold(f32::NAN, f32::max);
    }

    pub fn set_y_coord(&mut self, coord: Vec<f32>) {
        self.y_coord = coord;
        self.max_y = self.y_coord.iter().cloned().fold(f32::NAN, f32::max);
    }

    pub fn set_x_label<T: Into<String>>(&mut self, label: T) {
        self.x_label = label.into();
    }

    pub fn set_y_label<T: Into<String>>(&mut self, label: T) {
        self.y_label = label.into();
    }

    pub fn set_label_color(&mut self, color: Color) {
        self.label_color = color;
    }

    fn draw_grid_lines(&mut self) {
        let size = self.base().get_size();
        let step_x = size.x / 10.0;
        let step_y = size.y / 10.0;
        let grid_color = self.grid_color;

        let bg = self.background_color;
        self.base_mut().draw_rect(
            Rect2::new(Vector2::new(-20.0, -20.0), size + Vector2::new(30., 40.)),
            bg,
        );

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
        let x_labels = self.x_coord.clone();
        let y_labels = self.y_coord.clone();
        let x_color = self.x_coord_color;
        let y_color = self.y_coord_color;

        // Draw x-axis labels
        for x in x_labels {
            let point = self.convert_point(Vector2::new(x, 0.0));
            self.base_mut().draw_string(
                font.clone(),
                point + Vector2::new(-20., 20.),
                x.to_string().into(),
            );

            let p2 = self.convert_point(Vector2::new(x, self.max_y));
            self.base_mut().draw_dashed_line(point, p2, x_color);
        }

        // Draw y-axis labels
        for y in y_labels {
            let point = self.convert_point(Vector2::new(0.0, y));
            self.base_mut().draw_string(
                font.clone(),
                point + Vector2::new(-20., 20.),
                y.to_string().into(),
            );
            let p2 = self.convert_point(Vector2::new(self.max_x, y));

            self.base_mut().draw_dashed_line(point, p2, y_color);
        }

        let point = Vector2::new(size.x + 2., size.y + 20.);
        let label = self.x_label.clone();
        self.base_mut()
            .draw_string(font.clone(), point, label.into());

        let point = Vector2::new(-15.0, 0.);
        let label = self.y_label.clone();
        self.base_mut()
            .draw_string(font.clone(), point, label.into());
    }

    fn draw_points(&mut self) {
        if self.points.len() < 2 {
            return;
        }

        let color = self.line_color;

        for i in 0..self.points.len() - 1 {
            let p1 = self.convert_point(self.points[i]);
            let p2 = self.convert_point(self.points[i + 1]);
            self.base_mut()
                .draw_line_ex(p1, p2, color)
                .width(4.0)
                .done();
        }

        let color = self.point_color;

        // 数据原点
        for i in 0..self.points.len() {
            let p1 = self.convert_point(self.points[i]);
            self.base_mut().draw_circle(p1, 5., color);
        }
    }
    fn draw_hover(&mut self) {
        if self.hovered_point.is_none() {
            return;
        }
        godot_print!("---- hover---");

        let p = self.hovered_point.clone().unwrap();

        let font = self
            .base()
            .get_theme_font("HarmonyOS Sans SC Regular".into())
            .unwrap();

        let point = self.convert_point(p.0);

        self.base_mut().draw_string(font.clone(), point, p.1.into());
    }

    // 转化为 rect 的坐标
    fn convert_point(&self, point: Vector2) -> Vector2 {
        let size = self.base().get_size();

        let x = (point.x / self.max_x) * size.x;
        let y = size.y - (point.y / self.max_y) * size.y;

        Vector2::new(x, y)
    }
}
