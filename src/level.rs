use sdl2::rect::Rect;

pub fn init(size: i32) -> Vec<Rect> {
    let mut rects = Vec::new();
    for num in 0..size {
        let rect1 = Rect::new(num, 0, 1 as u32, 1 as u32);
        let rect2 = Rect::new(size, num, 1 as u32, 1 as u32);
        let rect3 = Rect::new(0, num + 1, 1 as u32, 1 as u32);
        let rect4 = Rect::new(num + 1, size, 1 as u32, 1 as u32);
        rects.push(rect1);
        rects.push(rect2);
        rects.push(rect3);
        rects.push(rect4);
    }
    let rect1 = Rect::new(3, 3, 1 as u32, 1 as u32);
    let rect2 = Rect::new(7, 3, 1 as u32, 1 as u32);
    let rect3 = Rect::new(3, 7, 1 as u32, 1 as u32);
    let rect4 = Rect::new(7, 7, 1 as u32, 1 as u32);
    rects.push(rect1);
    rects.push(rect2);
    rects.push(rect3);
    rects.push(rect4);
    rects
}
