mod app;
mod gl;

fn main() {
    let mut app = app::App::new();

    while app.running {
        app.update();
    }
}