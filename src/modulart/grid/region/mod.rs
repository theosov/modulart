pub mod cell;

pub use cell::Cell;
use std::ops::RangeInclusive;

pub struct Region {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Region {
    pub fn rows(&self) -> RangeInclusive<isize> {
        let first_row = (self.y / Cell::SIZE as f32).floor() as isize;

        let visible_rows = (self.height / Cell::SIZE as f32).ceil() as isize;

        first_row..=first_row + visible_rows
    }

    pub fn columns(&self) -> RangeInclusive<isize> {
        let first_column = (self.x / Cell::SIZE as f32).floor() as isize;

        let visible_columns = (self.width / Cell::SIZE as f32).ceil() as isize;

        first_column..=first_column + visible_columns
    }

    fn cull<'a>(
        &self,
        cells: impl Iterator<Item = &'a Cell>,
    ) -> impl Iterator<Item = &'a Cell> {
        let rows = self.rows();
        let columns = self.columns();

        cells.filter(move |cell| {
            rows.contains(&cell.i) && columns.contains(&cell.j)
        })
    }
}