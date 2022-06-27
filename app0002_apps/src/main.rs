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
    let _window = app.new_window().size(600, 400).title("Why!? thanks.").view(view).build().unwrap();
    Model{_window}
}

// Will be called every frame
fn update(_app: &App, _model:&mut Model, _update: Update ) {

}

fn view (app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(PLUM);
    draw.ellipse().color(STEELBLUE);
    draw.to_frame(app, &frame).unwrap();
}
