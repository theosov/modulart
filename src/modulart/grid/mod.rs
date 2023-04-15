mod region;

use region::Region;
use region::cell::Cell;

use iced::mouse::Interaction;
use iced;
use iced::widget::canvas;
use iced::widget::canvas::{
    Cache,
    Canvas,
    Cursor,
    Frame,
    Geometry,
};
use iced::{
    Color,
    Element,
    Length,
    Point,
    Rectangle,
    Size,
    Theme,
    Vector,
};
pub struct Grid {
    grid_cache: Cache,
    scaling: f32,
    translation: Vector,
}

#[derive(Debug, Clone)]
pub enum Message {

}

impl Default for Grid {
    fn default() -> Self {
        Self::init()
    }
}

impl Grid {
    const MIN_SCALING: f32 = 0.1;
    const MAX_SCALING: f32 = 2.0;

    pub fn init() -> Self {
        Self {
            grid_cache: Cache::default(),
            scaling: 1.0,
            translation: Vector::default()
        }
    }

    pub fn view(&self) -> Element<Message> {
        Canvas::new(self)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    pub fn visible_region(&self, size: Size) -> Region {
        let width = size.width / self.scaling;
        let height = size.height / self.scaling;

        Region {
            x: -self.translation.x - width / 2.0,
            y: -self.translation.y - height / 2.0,
            width,
            height,
        }
    }
}

impl canvas::Program<Message> for Grid {
    type State = Interaction;

    fn draw(
        &self,
        _interaction: &Interaction,
        _theme: &Theme,
        bounds: Rectangle,
        cursor: Cursor,
    ) -> Vec<Geometry> {
        let center = Vector::new(bounds.width / 2.0, bounds.height / 2.0);

        let overlay = {
            let mut frame = Frame::new(bounds.size());

            let hovered_cell = cursor.position_in(&bounds).map(|position| {
                Cell::at(position)
            });

            if let Some(cell) = hovered_cell {
                frame.with_save(|frame| {
                    frame.translate(center);
                    frame.scale(self.scaling);
                    frame.translate(self.translation);
                    frame.scale(Cell::SIZE as f32);

                    frame.fill_rectangle(
                        Point::new(cell.j as f32, cell.i as f32),
                        Size::UNIT,
                        Color {
                            a: 0.5,
                            ..Color::BLACK
                        },
                    );
                });
            }

            frame.into_geometry()
        };

        let grid = self.grid_cache.draw(bounds.size(), |frame| {
            frame.translate(center);
            frame.scale(self.scaling);
            frame.translate(self.translation);
            frame.scale(Cell::SIZE as f32);

            let region = self.visible_region(frame.size());
            let rows = region.rows();
            let columns = region.columns();
            let (total_rows, total_columns) = (rows.clone().count(), columns.clone().count());
            let width = 2.0 / Cell::SIZE as f32;
            let color = Color::from_rgb8(70, 74, 83);

            frame.translate(Vector::new(-width / 2.0, -width / 2.0));

            for row in region.rows() {
                frame.fill_rectangle(
                    Point::new(*columns.start() as f32, row as f32),
                    Size::new(total_columns as f32, width),
                    color
                );
            }

            for collumn in region.columns() {
                frame.fill_rectangle(
                    Point::new(collumn as f32, *rows.start() as f32),
                    Size::new(width, total_rows as f32),
                    color,
                );
            }
        });

        vec![grid, overlay]
    }
}