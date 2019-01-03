use glutin::dpi::{LogicalPosition, LogicalSize};
use glutin::{
    ElementState, ModifiersState, MouseButton as GlutinMouseButton, MouseScrollDelta,
    VirtualKeyCode, WindowEvent,
};
use prototty::{inputs, Coord, Input, MouseButton as ProtottyMouseButton, ScrollDirection};

pub enum InputEvent {
    Input(Input),
    Resize(u32, u32),
    Quit,
}

macro_rules! convert_char_shift {
    ($lower:expr, $upper:expr, $shift:expr) => {
        Input::Char(if $shift { $upper } else { $lower })
    };
}

fn convert_keycode(code: VirtualKeyCode, keymod: ModifiersState) -> Option<Input> {
    let shift = keymod.shift;
    let input = match code {
        VirtualKeyCode::Space => Input::Char(' '),
        VirtualKeyCode::A => convert_char_shift!('a', 'A', shift),
        VirtualKeyCode::B => convert_char_shift!('b', 'B', shift),
        VirtualKeyCode::C => convert_char_shift!('c', 'C', shift),
        VirtualKeyCode::D => convert_char_shift!('d', 'D', shift),
        VirtualKeyCode::E => convert_char_shift!('e', 'E', shift),
        VirtualKeyCode::F => convert_char_shift!('f', 'F', shift),
        VirtualKeyCode::G => convert_char_shift!('g', 'G', shift),
        VirtualKeyCode::H => convert_char_shift!('h', 'H', shift),
        VirtualKeyCode::I => convert_char_shift!('i', 'I', shift),
        VirtualKeyCode::J => convert_char_shift!('j', 'J', shift),
        VirtualKeyCode::K => convert_char_shift!('k', 'K', shift),
        VirtualKeyCode::L => convert_char_shift!('l', 'L', shift),
        VirtualKeyCode::M => convert_char_shift!('m', 'M', shift),
        VirtualKeyCode::N => convert_char_shift!('n', 'N', shift),
        VirtualKeyCode::O => convert_char_shift!('o', 'O', shift),
        VirtualKeyCode::P => convert_char_shift!('p', 'P', shift),
        VirtualKeyCode::Q => convert_char_shift!('q', 'Q', shift),
        VirtualKeyCode::R => convert_char_shift!('r', 'R', shift),
        VirtualKeyCode::S => convert_char_shift!('s', 'S', shift),
        VirtualKeyCode::T => convert_char_shift!('t', 'T', shift),
        VirtualKeyCode::U => convert_char_shift!('u', 'U', shift),
        VirtualKeyCode::V => convert_char_shift!('v', 'V', shift),
        VirtualKeyCode::W => convert_char_shift!('w', 'W', shift),
        VirtualKeyCode::X => convert_char_shift!('x', 'X', shift),
        VirtualKeyCode::Y => convert_char_shift!('y', 'Y', shift),
        VirtualKeyCode::Z => convert_char_shift!('z', 'Z', shift),
        VirtualKeyCode::Key1 => convert_char_shift!('1', '!', shift),
        VirtualKeyCode::Key2 => Input::Char('2'),
        VirtualKeyCode::Key3 => convert_char_shift!('3', '#', shift),
        VirtualKeyCode::Key4 => convert_char_shift!('4', '$', shift),
        VirtualKeyCode::Key5 => convert_char_shift!('5', '%', shift),
        VirtualKeyCode::Key6 => convert_char_shift!('6', '^', shift),
        VirtualKeyCode::Key7 => convert_char_shift!('7', '&', shift),
        VirtualKeyCode::Key8 => convert_char_shift!('8', '*', shift),
        VirtualKeyCode::Key9 => convert_char_shift!('9', '(', shift),
        VirtualKeyCode::Key0 => convert_char_shift!('0', ')', shift),
        VirtualKeyCode::Numpad1 => Input::Char('1'),
        VirtualKeyCode::Numpad2 => Input::Char('2'),
        VirtualKeyCode::Numpad3 => Input::Char('3'),
        VirtualKeyCode::Numpad4 => Input::Char('4'),
        VirtualKeyCode::Numpad5 => Input::Char('5'),
        VirtualKeyCode::Numpad6 => Input::Char('6'),
        VirtualKeyCode::Numpad7 => Input::Char('7'),
        VirtualKeyCode::Numpad8 => Input::Char('8'),
        VirtualKeyCode::Numpad9 => Input::Char('9'),
        VirtualKeyCode::Numpad0 => Input::Char('0'),
        VirtualKeyCode::Period => convert_char_shift!('.', '>', shift),
        VirtualKeyCode::Comma => convert_char_shift!(',', '<', shift),
        VirtualKeyCode::Slash => convert_char_shift!('/', '?', shift),
        VirtualKeyCode::Left => Input::Left,
        VirtualKeyCode::Right => Input::Right,
        VirtualKeyCode::Up => Input::Up,
        VirtualKeyCode::Down => Input::Down,
        VirtualKeyCode::Escape => inputs::ESCAPE,
        VirtualKeyCode::Return => inputs::RETURN,
        VirtualKeyCode::At => Input::Char('@'),
        VirtualKeyCode::Add => Input::Char('+'),
        VirtualKeyCode::Subtract => Input::Char('-'),
        VirtualKeyCode::Equals => convert_char_shift!('=', '+', shift),
        VirtualKeyCode::Backslash => convert_char_shift!('\\', '|', shift),
        VirtualKeyCode::Grave => convert_char_shift!('`', '~', shift),
        VirtualKeyCode::Apostrophe => convert_char_shift!('\'', '"', shift),
        VirtualKeyCode::LBracket => convert_char_shift!('[', '{', shift),
        VirtualKeyCode::RBracket => convert_char_shift!(']', '}', shift),
        VirtualKeyCode::PageUp => Input::PageUp,
        VirtualKeyCode::PageDown => Input::PageDown,
        VirtualKeyCode::Home => Input::Home,
        VirtualKeyCode::End => Input::End,
        _ => return None,
    };
    Some(input)
}

pub fn convert_event(
    event: WindowEvent,
    (cell_width, cell_height): (f32, f32),
    last_mouse_coord: &mut Coord,
) -> Option<InputEvent> {
    match event {
        WindowEvent::CloseRequested => {
            return Some(InputEvent::Quit);
        }
        WindowEvent::Resized(LogicalSize { width, height }) => {
            return Some(InputEvent::Resize(width as u32, height as u32));
        }
        WindowEvent::KeyboardInput { input, .. } => {
            if let ElementState::Pressed = input.state {
                if let Some(virtual_keycode) = input.virtual_keycode {
                    if let Some(input) = convert_keycode(virtual_keycode, input.modifiers) {
                        return Some(InputEvent::Input(input));
                    }
                }
            }
            None
        }
        WindowEvent::CursorMoved {
            position: LogicalPosition { x, y },
            ..
        } => {
            let x = (x / (cell_width as f64)) as i32;
            let y = (y / (cell_height as f64)) as i32;
            let coord = Coord::new(x, y);
            *last_mouse_coord = coord;
            Some(InputEvent::Input(Input::MouseMove(coord)))
        }
        WindowEvent::MouseInput { state, button, .. } => {
            let button = match button {
                GlutinMouseButton::Left => ProtottyMouseButton::Left,
                GlutinMouseButton::Middle => ProtottyMouseButton::Middle,
                GlutinMouseButton::Right => ProtottyMouseButton::Right,
                GlutinMouseButton::Other(_) => return None,
            };
            let input = match state {
                ElementState::Pressed => Input::MousePress {
                    coord: *last_mouse_coord,
                    button,
                },
                ElementState::Released => Input::MouseRelease {
                    coord: *last_mouse_coord,
                    button,
                },
            };
            Some(InputEvent::Input(input))
        }
        WindowEvent::MouseWheel { delta, .. } => {
            let (x, y) = match delta {
                MouseScrollDelta::LineDelta(x, y) => (x, y),
                MouseScrollDelta::PixelDelta(LogicalPosition { x, y }) => (x as f32, y as f32),
            };
            let direction = if y > 0. {
                ScrollDirection::Up
            } else if y < 0. {
                ScrollDirection::Down
            } else if x > 0. {
                ScrollDirection::Right
            } else if x < 0. {
                ScrollDirection::Left
            } else {
                return None;
            };
            Some(InputEvent::Input(Input::MouseScroll {
                direction,
                coord: *last_mouse_coord,
            }))
        }
        _ => None,
    }
}
