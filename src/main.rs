use autopilot::{bitmap::{capture_screen_portion}, mouse::{Button, toggle, move_to}, geometry::{Rect, Point, Size}};
use image::{Rgba};
use std::{error, time::Duration};

const START_X: f64 = 210.0;
const START_Y: f64 = 210.0;
const SIZE_X: f64 = 1000.0;
const SIZE_Y: f64 = 750.0;


fn compare_colors(a: Rgba<u8>, b: Rgba<u8>, tol: i32) -> bool {
    a.0.into_iter().zip(b.0).all(|(val_a, val_b)| (val_a as i32 - val_b as i32).abs() <= tol)
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let mut fail_count = 0;
    const RED_COLOR: Rgba<u8> = Rgba::<u8>([200, 0, 0, 255]);
    const ACTIVE_BTN_COLOR: Rgba<u8> = Rgba::<u8>([156, 148, 116, 255]);

    while fail_count < 35 {
        move_to(Point::new(0.0, 0.0))?;
        let bitmap = capture_screen_portion(Rect::new(Point::new(START_X, START_Y), Size::new(SIZE_X, SIZE_Y)))?;

        // Check for prestige
        if compare_colors(bitmap.get_pixel(Point::new(678.0 - START_X, 513.0 - START_Y)), RED_COLOR, 60) &&
           compare_colors(bitmap.get_pixel(Point::new(677.0 - START_X, 655.0 - START_Y)), RED_COLOR, 60) &&
           compare_colors(bitmap.get_pixel(Point::new(614.0 - START_X, 620.0 - START_Y)), RED_COLOR, 60)
        {
            move_to(Point::new(670.0, 580.0))?;
            toggle(Button::Left, true);
            std::thread::sleep(Duration::from_millis(2500));
            toggle(Button::Left, false);
        }

        if let Some(point) = bitmap.find_color(ACTIVE_BTN_COLOR, Some(0.001), None, None) {
            move_to(Point::new(START_X + point.x + 30.0, START_Y + point.y))?;
            std::thread::sleep(Duration::from_millis(30));
            toggle(Button::Left, true);
            std::thread::sleep(Duration::from_millis(500));
            toggle(Button::Left, false);
            fail_count = 0;
        } else {
            fail_count += 1;
        }
    }
    Ok(())
}
