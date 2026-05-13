use crate::{
    model::elements::pos3::{self, Pos3},
    screenspace::{
        elements::{
            cell_color::CellColor,
            drawable::{self, Drawable},
            screenspace_position::ScreenPosition,
        },
        screen::screen::Screen,
    },
};

pub struct Line {
    from: Pos3,
    to: Pos3,
    color: CellColor,
}
impl Line {
    pub fn from_to(from: &Pos3, to: &Pos3) -> Self {
        Self {
            from: *from,
            to: *to,
            color: CellColor::WHITE,
        }
    }
    pub fn from_to_with_color(from: &Pos3, to: &Pos3, color: &CellColor) -> Self {
        Self {
            from: *from,
            to: *to,
            color: *color,
        }
    }
    fn distanceVector(from: &ScreenPosition, to: &ScreenPosition) -> (f64, f64) {
        let diffVector = (
            (to.x() as isize) - (from.x() as isize),
            (to.y() as isize) - (from.y() as isize),
        );
        let magnitude = ((diffVector.0 as f64).powi(2) + (diffVector.1 as f64).powi(2)).sqrt();
        (
            diffVector.0 as f64 / magnitude,
            diffVector.1 as f64 / magnitude,
        )
    }
    //TODO: change to Bresenham's line algorithm
    fn move_pos_towards_dir(x_dir: &f64, y_dir: &f64, pos: &mut ScreenPosition) {
        if (x_dir.abs() - y_dir.abs()).abs() < 0.5 {
            match *y_dir {
                n if n < 0.0 => pos.set_y(&(pos.y() - 1)),
                _ => pos.set_y(&(pos.y() + 1)),
            };
            match *x_dir {
                n if n < 0.0 => pos.set_x(&(pos.x() - 1)),
                _ => pos.set_x(&(pos.x() + 1)),
            };
        } else if x_dir.abs() > y_dir.abs() {
            if *x_dir > 0.0 {
                pos.set_x(&(pos.x() + 1));
            } else {
                pos.set_x(&(pos.x() - 1));
            }
        } else {
            if *y_dir > 0.0 {
                pos.set_y(&(pos.y() + 1));
            } else {
                pos.set_y(&(pos.y() - 1));
            }
        }
    }
    fn mid_point(&self) -> Pos3 {
        Pos3::new(
            &(self.from.x() + self.to.x() / 2.0),
            &(self.from.y() + self.to.y() / 2.0),
            &(self.from.z() + self.to.z() / 2.0),
        )
    }
}
impl Drawable for Line {
    fn draw(&self, screen: &mut Screen) {
        let from_pos: ScreenPosition = screen.project_point(&self.from);
        let to_pos: ScreenPosition = screen.project_point(&self.to);
        screen.color_cell(&from_pos, &self.color);
        let mut cur_pos = from_pos;
        while (cur_pos != to_pos) {
            let (x_dir, y_dir) = Self::distanceVector(&cur_pos, &to_pos);
            Self::move_pos_towards_dir(&x_dir, &y_dir, &mut cur_pos);
            screen.color_cell(&cur_pos, &self.color);
        }
    }

    fn position(&self) -> Pos3 {
        self.mid_point()
    }
}
