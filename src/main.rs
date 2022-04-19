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
        .max_size()
        .title("Window")
        .events(Events::new()
            .on_close(Window::close)
            .on_init(move |window| {
                window
                    .commands()
                    .clear(Color::Blue)
                    .draw(&triangle)
                    .flush();
            }))
        .create().await;
}

*/

fn main() {

}
