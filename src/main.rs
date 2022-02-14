mod level;
mod keys;
mod events;
mod map;

use std::f64::consts::TAU;
use std::time::Duration;

use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels::Color;
use sdl2::rect::Rect;


fn main() {
    let map = map::map();
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let height = 675;
    let width = 1200;
    let window = video_subsystem.window("Window", width, height).build().unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    let unit = 23;
    let size = 10;
    let mut x;
    let mut y;
    let mut map_width = 0;
    for a in map {
        map_width = a.len() as i32;
        for b in a {
            print!("{}", b)
        }
        println!();
    }
    let map_height = map.len() as i32;
    println!("{}", map_width);
    println!("{}", map_height);
    let mut current_x = 0.0;
    let mut current_y = 0.0;
    let mut key = 0;
    let mut rotate_key = 0;
    let minimap = Rect::new(0, 0, (unit * map_width) as u32, (unit * map_height) as u32);
    let rect = Rect::new(0, (height / 2) as i32, width, height);
    let rects = level::init(size);
    let mut rotation: f64 = 0.0;
    let mut error;
    let rad = 0.3;
    let mut key_results;
    let mut event_results;
    let mut direction = 0.0;
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        canvas.set_draw_color(Color::RGB(63, 196, 255));
        canvas.clear();

        event_results = events::events(&mut event_pump, key, rotate_key);
        key = event_results.0;
        rotate_key = event_results.1;
        error = event_results.2;
        if error == 1 {
            break 'running;
        }

        if rotation >= TAU {
            rotation -= TAU;
        } else if rotation < 0.0 {
            rotation += TAU;
        }

        key_results = keys::check_keys(key, rotate_key, direction, rotation);
        direction = key_results.0;
        rotation = key_results.1;
        error = key_results.2;
        if error == 1 {
            println!("Impossible key combination found. Please send keys to author.\nkey: {}, rotate_key: {}", key, rotate_key);
            break 'running;
        }
        if direction < 1.0
            && current_x + (rotation + direction * TAU).cos() < (size as f64 - 1.0) / 2.0 - rad
            && current_x + (rotation + direction * TAU).cos() > -(size as f64 - 1.0) / 2.0 - rad
            && (((current_x + (rotation + direction * TAU).cos() > -3.0 / 2.0 - rad)
                    && (current_x + (rotation + direction * TAU).cos() < 3.0 / 2.0 - rad))
                || ((current_x + (rotation + direction * TAU).cos() > -(size as f64 - 1.0) / 2.0 - rad)
                    && (current_x + (rotation + direction * TAU).cos() < -5.0 / 2.0 + rad))
                || ((current_x + (rotation + direction * TAU).cos() > 5.0 / 2.0 + rad)
                    && (current_x + (rotation + direction * TAU).cos() < (size as f64 - 1.0) / 2.0 - rad))
                || (current_y < -5.0 / 2.0 + rad)
                || (current_y > 5.0 / 2.0 + rad)
                || ((current_y > -3.0 / 2.0 - rad)
                    && (current_y < 3.0 / 2.0 - rad))) {
            current_x += (rotation + direction * TAU).cos();
        }
        if direction < 1.0
            && current_y + (rotation + direction * TAU).sin() < (size as f64 - 1.0) / 2.0 - rad
            && current_y + (rotation + direction * TAU).sin() > -(size as f64 - 1.0) / 2.0 - rad
            && (((current_y + (rotation + direction * TAU).sin() > -3.0 / 2.0 - rad)
                    && (current_y + (rotation + direction * TAU).sin() < 3.0 / 2.0 - rad))
                || ((current_y + (rotation + direction * TAU).sin() > -(size as f64 - 1.0) / 2.0 - rad)
                    && (current_y + (rotation + direction * TAU).sin() < -5.0 / 2.0 + rad))
                || ((current_y + (rotation + direction * TAU).sin() > 5.0 / 2.0 + rad)
                    && (current_y + (rotation + direction * TAU).sin() < (size as f64 - 1.0) / 2.0 - rad))
                || (current_x < -5.0 / 2.0 + rad)
                || (current_x > 5.0 / 2.0 + rad)
                || ((current_x > -3.0 / 2.0 - rad)
                    && (current_x < 3.0 / 2.0 - rad))) {
            current_y += (rotation + direction * TAU).sin();
        }
        x = current_x as i32;
        y = current_y as i32;

        /*
        1 unit at a time will get everything
        big for loop for 1 to number of angles, a while loop for every ray
        only need trig once at the start of every ray
        1 ray per pixel, 1920 total (window width)
         */

        canvas.set_draw_color(Color::RGB(136, 136, 136));
        let _ = canvas.fill_rect(rect);

        canvas.set_draw_color(Color::RGB(136, 136, 136));
        let _ = canvas.fill_rect(minimap);

        let _ = canvas.line(((size + 3) / 2 + x) as i16 * unit as i16,((size + 3) / 2 + y) as i16 * unit as i16,((size + 3) as f64 / 2.0 + x as f64 + (rotation - (1.0/6.0) * TAU).cos() * 2.5) as i16 * unit as i16,((size + 3) as f64 / 2.0 + y as f64 + (rotation - (1.0/6.0) * TAU).sin() * 2.5) as i16 * unit as i16,Color::YELLOW);
        let _ = canvas.line(((size + 3) / 2 + x) as i16 * unit as i16,((size + 3) / 2 + y) as i16 * unit as i16,((size + 3) as f64 / 2.0 + x as f64 + (rotation + (1.0/6.0) * TAU).cos() * 2.5) as i16 * unit as i16,((size + 3) as f64 / 2.0 + y as f64 + (rotation + (1.0/6.0) * TAU).sin() * 2.5) as i16 * unit as i16,Color::YELLOW);

        for rect in rects.clone() {
            canvas.set_draw_color(Color::RGB(20, 150, 20));
            let draw_rect_x = rect.x() * unit;
            let draw_rect_y = rect.y() * unit;
            let draw_rect_w = rect.width() * unit as u32;
            let draw_rect_h = rect.height() * unit as u32;
            let draw_rect = Rect::new(draw_rect_x, draw_rect_y, draw_rect_w, draw_rect_h);
            let _ = canvas.fill_rect(draw_rect);
        }
        let _ = canvas.line(((size + 3) / 2 + x) as i16 * unit as i16,((size + 3) / 2 + y) as i16 * unit as i16,((size + 3) as f64 / 2.0 + x as f64 + rotation.cos() * 2.5) as i16 * unit as i16,((size + 3) as f64 / 2.0 + y as f64 + rotation.sin() * 2.5) as i16 * unit as i16,Color::RED);
        let _ = canvas.filled_circle(((size + 3) / 2 + x) as i16 * unit as i16, ((size + 3) / 2 + y) as i16 * unit as i16, (rad * unit as f64) as i16, Color::RED);
        let _ = canvas.aa_circle(((size + 3) / 2 + x) as i16 * unit as i16, ((size + 3) / 2 + y) as i16 * unit as i16, (rad * unit as f64) as i16, Color::RED);
        // let _ = canvas.filled_polygon(&[300, 450, 450, 300], &[150, 100, 350, 300], Color::RGB(20, 150, 20));
        // let r = (2.0/6.0) * TAU / width as f64;
        // for ray in 0..width {
        //     let ray_angle = r * ray as f64;
        //     let player_x = ((size + 3) * unit) as f64 / 2.0 + x as f64;
        //     let player_y = ((size + 3) * unit) as f64 / 2.0 + y as f64;
        //     let x_dir = if (rotation - (1.0/6.0) * TAU + ray_angle).cos() == 0.0 { 0.0000001 } else { (rotation - (1.0/6.0) * TAU + ray_angle).cos() };
        //     let y_dir = if (rotation - (1.0/6.0) * TAU + ray_angle).sin() == 0.0 { 0.0000001 } else { (rotation - (1.0/6.0) * TAU + ray_angle).sin() };
        //     let delta_x = (x_dir / y_dir).abs() * 10.0;
        //     let delta_y = (y_dir / x_dir).abs() * 10.0;
        //     let mut delta_ratio = 1.0;
        //     if delta_x > delta_y {
        //         delta_ratio = (delta_x / delta_y).floor();
        //     } else {
        //         delta_ratio = (delta_y / delta_x).floor();
        //     }
        //     let mut start_x = 0.0;
        //     let mut start_y = 0.0;
        //     let mut step_x = 0;
        //     let mut step_y = 0;
        //     if x_dir > 0.0 {
        //         step_x = 1;
        //         start_x = (((current_x) / unit as f64).floor() * unit as f64 + 1.0 - current_x) * delta_x
        //     } else {
        //         step_x = -1;
        //         start_x = (current_x - ((current_x) / unit as f64).floor() * unit as f64) * delta_x
        //     }
        //     if y_dir > 0.0 {
        //         step_x = 1;
        //         start_y = (((current_y) / unit as f64).floor() * unit as f64 + 1.0 - current_y) * delta_y
        //     } else {
        //         step_y = -1;
        //         start_y = (current_y - ((current_y) / unit as f64).floor() * unit as f64) * delta_y
        //     }
        //     let mut no_wall_found = true;
        //     let mut wall_x = current_x + start_x;
        //     let mut wall_y = current_y + start_y;
        //     let mut map_x = ((current_x) / unit as f64).floor() as i32 + 3;
        //     let mut map_y = ((current_y) / unit as f64).floor() as i32 + 3;
        //     while no_wall_found {
        //         if wall_x < wall_y {
        //             wall_x += delta_x;
        //             map_x += step_x;
        //             let side = 0;
        //         } else {
        //             wall_y += delta_y;
        //             map_y += step_y;
        //             let side = 1;
        //         }
        //         let map = map::map();
        //         if map[map_x as usize][map_y as usize] > 0 { no_wall_found = false; }
        //     }
        //     //let _ = canvas.line(((size + 3) * unit / 2 + x) as i16,((size + 3) * unit / 2 + y) as i16,wall_x as i16,wall_y as i16,Color::YELLOW);
        //     let dist = (current_x.powf(2.0) + wall_x.powf(2.0)).sqrt();
        //     let _ = canvas.line(ray as i16, (height as f64 / 2.0 - dist) as i16, ray as i16,(height as f64 / 2.0 + dist) as i16,Color::YELLOW);
        // }
        canvas.present();

        std::thread::sleep(Duration::new(0, 1000000000/60));
    }
}
