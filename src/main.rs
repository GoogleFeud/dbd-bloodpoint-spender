use autopilot::{bitmap::{capture_screen_portion}, mouse::{Button, toggle, move_to}, geometry::{Rect, Point, Size}};
use image::{Rgba};
use std::{error, time::Duration};

const START_X: f64 = 210.0;
const START_Y: f64 = 210.0;
const SIZE_X: f64 = 1000.0;
const SIZE_Y: f64 = 750.0;

fn main() -> Result<(), Box<dyn error::Error>> {
    let mut fail_count = 0;
    while fail_count < 5 {
        move_to(Point::new(0.0, 0.0))?;
        let bitmap = capture_screen_portion(Rect::new(Point::new(START_X, START_Y), Size::new(SIZE_X, SIZE_Y)))?;
        if let Some(point) = bitmap.find_color(Rgba([156 as u8, 148 as u8, 116 as u8, 1 as u8]), Some(0.001), None, None) {
            move_to(Point::new(START_X + point.x + 30.0, START_Y + point.y))?;
            std::thread::sleep(Duration::from_millis(100));
            toggle(Button::Left, true);
            std::thread::sleep(Duration::from_millis(500));
            toggle(Button::Left, false);
        } else {
            fail_count += 1;
        }
    }
    Ok(())
}
