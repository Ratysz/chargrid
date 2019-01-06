use cell::*;
use error::*;
use prototty_grid::*;
use prototty_input::*;
use prototty_render::*;
use std::time::Duration;
use terminal::*;

/// An interface to a terminal for rendering `View`s, and getting input.
pub struct Context {
    terminal: Terminal,
    grid: Grid<Colour, Colour>,
}

impl Context {
    /// Initialise a new context using the current terminal.
    pub fn new() -> Result<Self> {
        Terminal::new().and_then(Self::from_terminal)
    }

    fn from_terminal(mut terminal: Terminal) -> Result<Self> {
        let size = terminal.resize_if_necessary()?;
        let grid = Grid::new(size);

        Ok(Self { terminal, grid })
    }

    fn resize_if_necessary(&mut self) -> Result<()> {
        let size = self.terminal.resize_if_necessary()?;
        if size != self.grid.size() {
            self.grid.resize(size);
        }

        Ok(())
    }

    pub fn drain_input(&mut self) -> Result<DrainInput> {
        self.terminal.drain_input()
    }

    /// Gets an input event from the terminal if one is present,
    /// returning immediately.
    pub fn poll_input(&mut self) -> Result<Option<Input>> {
        self.terminal.poll_input()
    }

    /// Gets an input event from the terminal, waiting until
    /// an event occurs.
    pub fn wait_input(&mut self) -> Result<Input> {
        self.terminal.wait_input()
    }

    /// Gets an input event from the terminal, waiting until
    /// either an event occurs, or the timeout expires, in which
    /// case this method returns `None`.
    pub fn wait_input_timeout(&mut self, timeout: Duration) -> Result<Option<Input>> {
        self.terminal.wait_input_timeout(timeout)
    }

    pub fn render<V: View<T>, T>(&mut self, view: &mut V, data: &T) -> Result<()> {
        self.render_at(view, data, Coord::new(0, 0), 0)
    }

    pub fn render_at<V: View<T>, T>(
        &mut self,
        view: &mut V,
        data: &T,
        offset: Coord,
        depth: i32,
    ) -> Result<()> {
        self.resize_if_necessary()?;

        self.grid.clear();
        view.view(data, offset, depth, &mut self.grid);
        self.terminal.draw_grid(&self.grid)
    }

    pub fn size(&self) -> Result<Size> {
        self.terminal.size()
    }
}
