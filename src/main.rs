extern crate piston_window;
extern crate artichoke_backend;
extern crate artichoke_core;

use artichoke_backend::eval::Eval;
use artichoke_core::value::Value as ValueLike;
use piston_window::*;

fn main() {
    let interp = artichoke_backend::interpreter().unwrap();
    let result = interp.eval("10 * 10").unwrap();
    let result = result.try_into::<i64>();
    assert_eq!(result, Ok(100));
    println!("result {}", result.unwrap());

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