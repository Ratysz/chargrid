#[cfg(feature = "prototty_graphical")]
use prototty_graphical::*;
#[cfg(feature = "prototty_graphical_gfx")]
use prototty_graphical_gfx::*;
use roguelike_native::{simon::Arg, NativeCommon};
use roguelike_prototty::{app, Frontend};

const CELL_SIZE: f64 = 16.;

fn main() {
    env_logger::init();
    let NativeCommon {
        rng_seed,
        file_storage,
        controls,
        save_file,
        audio_player,
        game_config,
    } = NativeCommon::arg().with_help_default().parse_env_or_exit();
    let context = Context::new(ContextDescriptor {
        font_bytes: FontBytes {
            normal: include_bytes!("./fonts/PxPlus_IBM_CGAthin.ttf").to_vec(),
            bold: include_bytes!("./fonts/PxPlus_IBM_CGA.ttf").to_vec(),
        },
        title: "RIP".to_string(),
        window_dimensions: WindowDimensions::Windowed(Dimensions {
            width: 720.,
            height: 640.,
        }),
        cell_dimensions: Dimensions {
            width: CELL_SIZE,
            height: CELL_SIZE,
        },
        font_dimensions: Dimensions {
            width: CELL_SIZE,
            height: CELL_SIZE,
        },
        font_source_dimensions: Dimensions {
            width: CELL_SIZE as f32,
            height: CELL_SIZE as f32,
        },
        underline_width: 0.1,
        underline_top_offset: 0.8,
    })
    .unwrap();
    let app = app(
        game_config,
        Frontend::Native,
        controls,
        file_storage,
        save_file,
        audio_player,
        rng_seed,
    );
    context.run_app(app);
}
