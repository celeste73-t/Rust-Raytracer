mod app;
mod gl;
mod scene;

fn main() {
    let mut app = app::App::new();

    while app.running {
        app.update();
    }
}