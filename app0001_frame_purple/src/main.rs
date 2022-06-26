use nannou::prelude::*;

fn main() {
    nannou::sketch(view)
        .run();
}


fn view(_app: &App, frame: Frame){
    // get canvas
    let draw = _app.draw();

    // set the background to blue
    draw.background().color(PURPLE);

    draw.to_frame(_app, &frame).unwrap();

}