use prototty_render::*;

pub trait Wrap: private_wrap::Sealed {
    #[doc(hidden)]
    fn clear(&mut self);
    #[doc(hidden)]
    fn process_character<G: ViewGrid, R: ViewTransformRgb24>(
        &mut self,
        character: char,
        style: Style,
        context: ViewContext<R>,
        grid: &mut G,
    );
    #[doc(hidden)]
    fn flush<G: ViewGrid, R: ViewTransformRgb24>(
        &mut self,
        context: ViewContext<R>,
        grid: &mut G,
    ) {
        let _ = context;
        let _ = grid;
    }
    #[doc(hidden)]
    fn num_lines(&self) -> usize;
}

#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct None {
    cursor: Coord,
}

#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct Word {
    cursor: Coord,
    current_word_buffer: Vec<ViewCell>,
}

#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct Char {
    cursor: Coord,
}

impl None {
    pub fn new() -> Self {
        Self {
            cursor: Coord::new(0, 0),
        }
    }
}

impl Word {
    pub fn new() -> Self {
        Self {
            cursor: Coord::new(0, 0),
            current_word_buffer: Vec::new(),
        }
    }
}

impl Char {
    pub fn new() -> Self {
        Self {
            cursor: Coord::new(0, 0),
        }
    }
}

impl Wrap for None {
    fn clear(&mut self) {
        self.cursor = Coord::new(0, 0);
    }
    fn process_character<G: ViewGrid, R: ViewTransformRgb24>(
        &mut self,
        character: char,
        style: Style,
        context: ViewContext<R>,
        grid: &mut G,
    ) {
        match character {
            '\n' => {
                self.cursor.x = 0;
                self.cursor.y += 1;
            }
            '\r' => self.cursor.x = 0,
            other => {
                let view_cell = ViewCell {
                    character: Some(other),
                    style,
                };
                grid.set_cell_relative(self.cursor, 0, view_cell, context);
                self.cursor += Coord::new(1, 0);
            }
        }
    }
    fn num_lines(&self) -> usize {
        self.cursor.y as usize + 1
    }
}

impl Wrap for Word {
    fn clear(&mut self) {
        self.cursor = Coord::new(0, 0);
        self.current_word_buffer.clear();
    }

    fn process_character<G: ViewGrid, R: ViewTransformRgb24>(
        &mut self,
        character: char,
        style: Style,
        context: ViewContext<R>,
        grid: &mut G,
    ) {
        match character {
            '\n' => {
                self.flush(context, grid);
                self.cursor.x = 0;
                self.cursor.y += 1;
            }
            '\r' => {
                self.flush(context, grid);
                self.cursor.x = 0;
            }
            ' ' => {
                self.flush(context, grid);
                if self.cursor.x != 0 {
                    let view_cell = ViewCell {
                        character: Some(' '),
                        style,
                    };
                    grid.set_cell_relative(self.cursor, 0, view_cell, context);
                    self.cursor.x += 1;
                    assert!(self.cursor.x <= context.size.width() as i32);
                    if self.cursor.x == context.size.width() as i32 {
                        self.cursor.x = 0;
                        self.cursor.y += 1;
                    }
                }
            }
            other => {
                let view_cell = ViewCell {
                    character: Some(other),
                    style,
                };
                self.current_word_buffer.push(view_cell);
                assert!(
                    self.cursor.x + self.current_word_buffer.len() as i32
                        <= context.size.width() as i32
                );
                if self.cursor.x + self.current_word_buffer.len() as i32
                    == context.size.width() as i32
                {
                    if self.cursor.x == 0 {
                        self.flush(context, grid);
                    } else {
                        self.cursor.x = 0;
                        self.cursor.y += 1;
                    }
                }
            }
        }
    }

    fn flush<G: ViewGrid, R: ViewTransformRgb24>(
        &mut self,
        context: ViewContext<R>,
        grid: &mut G,
    ) {
        for view_cell in self.current_word_buffer.drain(..) {
            grid.set_cell_relative(self.cursor, 0, view_cell, context);
            self.cursor.x += 1;
        }
        assert!(self.cursor.x <= context.size.width() as i32);
        if self.cursor.x == context.size.width() as i32 {
            self.cursor.x = 0;
            self.cursor.y += 1;
        }
    }

    fn num_lines(&self) -> usize {
        self.cursor.y as usize + 1
    }
}

impl Wrap for Char {
    fn clear(&mut self) {
        self.cursor = Coord::new(0, 0);
    }

    fn process_character<G: ViewGrid, R: ViewTransformRgb24>(
        &mut self,
        character: char,
        style: Style,
        context: ViewContext<R>,
        grid: &mut G,
    ) {
        match character {
            '\n' => {
                self.cursor.x = 0;
                self.cursor.y += 1;
            }
            '\r' => self.cursor.x = 0,
            other => {
                let view_cell = ViewCell {
                    character: Some(other),
                    style,
                };
                grid.set_cell_relative(self.cursor, 0, view_cell, context);
                self.cursor += Coord::new(1, 0);
                if self.cursor.x >= context.size.width() as i32 {
                    self.cursor.x = 0;
                    self.cursor.y += 1;
                }
            }
        }
    }

    fn num_lines(&self) -> usize {
        self.cursor.y as usize + 1
    }
}

mod private_wrap {
    pub trait Sealed {}
    impl Sealed for super::None {}
    impl Sealed for super::Word {}
    impl Sealed for super::Char {}
}