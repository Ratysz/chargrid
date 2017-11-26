use std::mem;
use cgmath::Vector2;
use context::*;
use grid::*;

#[derive(Debug, Clone)]
pub struct Text {
    size: Vector2<u16>,
    string: String,
}

impl Text {
    pub fn new<D: Into<Vector2<u16>>, S: Into<String>>(string: S, size: D) -> Self {
        Self {
            size: size.into(),
            string: string.into(),
        }
    }
    pub fn set_size<D: Into<Vector2<u16>>>(&mut self, size: D) {
        self.size = size.into();
    }
    pub fn set<S: Into<String>>(&mut self, new: S) -> String {
        mem::replace(&mut self.string, new.into())
    }
    pub fn get(&self) -> &String {
        &self.string
    }
    pub fn into_handle(self) -> TextHandle { TextHandle(element_cell(self)) }
    pub(crate) fn render(&self, grid: &mut Grid<Cell>, seq: u64, offset: Vector2<i16>, depth: i16) {
        let bottom_right_abs = offset + self.size.cast();
        let mut coord = offset;
        for ch in self.string.chars() {
            match ch {
                '\n' => {
                    coord.x = offset.x;
                    coord.y += 1;
                    if coord.y == bottom_right_abs.y {
                        break;
                    }
                }
                '\r' => {
                    coord.x = offset.x;
                }
                _ => {
                    if let Some(cell) = grid.get_mut(coord) {
                        cell.update(seq, ch, depth);
                    }
                    coord.x += 1;
                    if coord.x == bottom_right_abs.x {
                        coord.x = offset.x;
                        coord.y += 1;
                        if coord.y == bottom_right_abs.y {
                            break;
                        }
                    }
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct TextHandle(ElementCell<Text>);

impl TextHandle {
    pub fn set_size<D: Into<Vector2<u16>>>(&self, size: D) {
        self.0.borrow_mut().set_size(size);
    }
    pub fn set<S: Into<String>>(&self, new: S) -> String {
        self.0.borrow_mut().set(new)
    }
    pub fn get(&self) -> String {
        (*self.0).borrow().get().clone()
    }
    pub(crate) fn render(&self, grid: &mut Grid<Cell>, seq: u64, offset: Vector2<i16>, depth: i16) {
        (*self.0).borrow().render(grid, seq, offset, depth);
    }
}

impl From<TextHandle> for ElementHandle {
    fn from(e: TextHandle) -> Self {
        ElementHandle::Text(e)
    }
}
impl From<Text> for ElementHandle {
    fn from(e: Text) -> Self {
        ElementHandle::Text(e.into())
    }
}
impl From<Text> for TextHandle {
    fn from(e: Text) -> Self {
        e.into_handle()
    }
}


