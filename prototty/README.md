# prototty

[![Version](https://img.shields.io/crates/v/prototty.svg)](https://crates.io/crates/prototty)
[![Documentation](https://docs.rs/prototty/badge.svg)](https://docs.rs/prototty)

Definition of types and traits for describing how to render a type to a grid of
characters (such as a terminal). Also provides an input type to simplify making
an application that works with multiple frontends.

## Example

Let's make a title element which renders a single line of text, within
a fixed width rectangle of height 1. The entire source code for this
example is in the "examples/title" directory in this repo.

```rust
extern crate prototty;
use prototty::*;

// Define a type representing the element
pub struct Title {
    pub width: u32,
    pub text: String,
}

// Define a type representing how the element will be rendered.
pub struct TitleView;

// Describe how a TitleView renders a Title by implementing View.
impl View<Title> for TitleView {
    fn view<G: ViewGrid>(&mut self, title: &Title, offset: Coord, depth: i32, grid: &mut G) {
        for (i, ch) in title.text.chars().enumerate() {
            if let Some(cell) = grid.get_mut(offset + Coord::new(i as i32, 0), depth) {
                cell.set_character(ch);
                cell.set_underline(true);
            }
        }
    }
}

// What if we want a way to rendered titles centered within their width?
pub struct CenteredTitleView;
impl View<Title> for CenteredTitleView {
    fn view<G: ViewGrid>(&mut self, title: &Title, offset: Coord, depth: i32, grid: &mut G) {
        let space = ::std::cmp::max(title.width as i32 - title.text.len() as i32, 0) / 2;
        for (i, ch) in title.text.chars().enumerate() {
            if let Some(cell) = grid.get_mut(offset + Coord::new(space + i as i32, 0), depth) {
                cell.set_character(ch);
                cell.set_underline(true);
            }
        }
    }
}

// Let's demonstrate both of these in action by rendering a title
// twice - once left aligned, an once centered:
pub struct DemoTitleView;
impl View<Title> for DemoTitleView {
    fn view<G: ViewGrid>(&mut self, title: &Title, offset: Coord, depth: i32, grid: &mut G) {
        // render the title left-aligned in the top-left corner
        TitleView.view(title, offset, depth, grid);

        // render the title centered 2 lines down
        CenteredTitleView.view(title, offset + Coord::new(0, 2), depth, grid);
    }
}

```

We can now render a `Title` with 3 of different views:
 - `TitleView` renders a `Title` left-aligned
 - `CenteredTitleView` renders a `Title` centered within its width
 - `DemoTitleView` renders a `Title` twice, using the other two views

This isn't much use by itself. To actually get something onto a screen, we'll
need a frontend. Continue this example in:
- [prototty-unix](https://github.com/stevebob/prototty/tree/master/unix)
- [prototty-glutin](https://github.com/stevebob/prototty/tree/master/glutin)
- [prototty-wasm](https://github.com/stevebob/prototty/tree/master/wasm)

Or see how to decorate it with a border in [prototty-common](https://github.com/stevebob/prototty/tree/master/common).
