use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

// This will be the shape of our data. Empty for now
struct Model {
    _window: window::Id,
}

// managed the model of our app (MVC), 'that's our data'
fn model(app: &App) -> Model {
    let _window = app.new_window().size(600, 400).title("This is a rectangle").view(view).build().unwrap();
    Model{_window}
}

// Will be called every frame
fn update(_app: &App, _model:&mut Model, _update: Update ) {

}

fn view (app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();

    // sets the background to GREY
    draw.background().color(GREY);

    // as for some reason they created the center in the middle, we are getting the top left point
    let top_left = app.window_rect().top_left();
    let rect_size = Vec2::new(50.0, 50.0);

    // Draws a rectangle
    draw.rect()
        // .x_y(top_left.x + rect_size.x / 2.0, top_left.y - rect_size.y / 2.0) // draw at a corner
        // .x_y(0.0, 0.0) // draw at the origin
        .x_y(app.mouse.x, app.mouse.y) // draw at the mouse position
        .w_h(rect_size.x, rect_size.y)
        // .z_degrees(45.0) 
        .color(PLUM);

    draw.to_frame(app, &frame).unwrap();
}
