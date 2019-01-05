use prototty_grid::*;
use prototty_render::{colours, Rgb24};

#[derive(Debug, Clone, Copy)]
pub struct Colour(pub u32);

impl From<Rgb24> for Colour {
    fn from(rgb24: Rgb24) -> Self {
        Colour(rgb24.into_u32())
    }
}

impl DefaultForeground for Colour {
    fn default_foreground() -> Self {
        Colour(colours::WHITE.into_u32())
    }
}

impl DefaultBackground for Colour {
    fn default_background() -> Self {
        Colour(colours::BLACK.into_u32())
    }
}
