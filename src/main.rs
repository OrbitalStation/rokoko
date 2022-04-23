extern crate rokoko;

use rokoko::prelude::*;
/*
use rokoko::prelude::*;

///
/// This example draws a red triangle onto the blue window
///
fn main() {
    let triangle = PolygonBuilder::fixed([
        (0.0, 0.5, 0.0),
        (0.5, 0.0, 0.0),
        (-0.25, -0.25, 0.0)
    ]).color(Color::RED);

    WindowBuilder::new()
        .maximized()
        .title("Window")
        .on_init(move |window| window
            .commands()
            .clear(Color::Blue)
            .draw(&triangle)
            .flush())
        .create()
        .unwrap();
}

*/

fn main() {
    Window::new()
        .title("")
        .size((1000., 1000.).into())
        .on_init(|_| println!("Initialization completed!"))
        .on_close(|w| {
            println!("Closing!");
            w.close()
        })
        .create()
        .unwrap()
}
