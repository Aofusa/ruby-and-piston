extern crate piston_window;

use artichoke_backend::prelude::core::*;
use artichoke_backend::prelude::*;
use piston_window::*;

fn example() -> i64 {
    let mut interp = artichoke_backend::interpreter().unwrap();
    let result = interp.eval(b"10 * 10").unwrap();
    let result = result.try_into::<i64>(&interp).unwrap();
    assert_eq!(100, result);
    return result;
}

fn main() {
    println!("result {}", example());

    let mut window: PistonWindow =
        WindowSettings::new("Hello Piston!", [640, 480])
        .exit_on_esc(true).build().unwrap();
    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics, _device| {
            clear([1.0; 4], graphics);
            rectangle([1.0, 0.0, 0.0, 1.0], // red
                      [0.0, 0.0, 100.0, 100.0],
                      context.transform,
                      graphics);
        });
    }
}