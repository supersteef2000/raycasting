pub fn check_keys(key: i32, rotate_key: i32, mut direction: f64, mut rotation: f64) -> (f64, f64, i32) {
    let mut error = 0;
    match key {
        0 | 101 | 1010 | 1111  =>  direction = 1.0,   // Used to indicate no movement
        1 | 1011               =>  direction = 0.0,
        1001                   =>  direction = 0.125,
        1000 | 1101            =>  direction = 0.25,
        1100                   =>  direction = 0.375,
        100 | 1110             =>  direction = 0.5,
        110                    =>  direction = 0.625,
        10 | 111               =>  direction = 0.75,
        11                     =>  direction = 0.875,
        _                      =>  error     = 1,
    }
    match rotate_key {
        0 | 11                 =>  rotation += 0.0,   // No rotation, ignored
        1                      =>  rotation -= 0.05,
        10                     =>  rotation += 0.05,
        _                      =>  error     = 1,
    }
    (direction, rotation, error)
}