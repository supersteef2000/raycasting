use sdl2::event::Event;
use sdl2::EventPump;
use sdl2::keyboard::Keycode;

pub fn events(event_pump: &mut EventPump, mut key: i32, mut rotate_key: i32) -> (i32, i32, i32) {
    let mut error = 0;
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit {..} |
            Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                error = 1;
            },
            Event::KeyDown { keycode: Some(Keycode::W), repeat: false, .. } => key += 1,
            Event::KeyDown { keycode: Some(Keycode::A), repeat: false, .. } => key += 10,
            Event::KeyDown { keycode: Some(Keycode::S), repeat: false, .. } => key += 100,
            Event::KeyDown { keycode: Some(Keycode::D), repeat: false, .. } => key += 1000,
            Event::KeyDown { keycode: Some(Keycode::Q), repeat: false, .. } => rotate_key += 1,
            Event::KeyDown { keycode: Some(Keycode::E), repeat: false, .. } => rotate_key += 10,
            //Event::KeyDown { keycode: Some(Keycode::LShift), repeat: false, .. } => speed_mod = 2.0,
            Event::KeyUp { keycode: Some(Keycode::W), .. } => key -= 1,
            Event::KeyUp { keycode: Some(Keycode::A), .. } => key -= 10,
            Event::KeyUp { keycode: Some(Keycode::S), .. } => key -= 100,
            Event::KeyUp { keycode: Some(Keycode::D), .. } => key -= 1000,
            Event::KeyUp { keycode: Some(Keycode::Q), .. } => rotate_key -= 1,
            Event::KeyUp { keycode: Some(Keycode::E), .. } => rotate_key -= 10,
            //Event::KeyUp { keycode: Some(Keycode::LShift), .. } => speed_mod = 1.0,
            _ => ()
        }
    }
    (key, rotate_key, error)
}