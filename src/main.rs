use beryllium::*;
use ogl33::*;

const WINDOW: &str = "OpenGL + Rust";
const WIDTH: i32 = 800;
const HEIGHT: i32 = 600;

struct App {
    sdl: Sdl,
    window: video::GlWindow,
    running: bool,
}

fn main() {
    let mut app = init();

    while app.running {
        update(&mut app);
    }
}

fn init() -> App {
    let sdl = Sdl::init(init::InitFlags::EVERYTHING);
    sdl.set_gl_context_major_version(4).unwrap();
    sdl.set_gl_context_minor_version(3).unwrap();
    sdl.set_gl_profile(video::GlProfile::Core).unwrap();
    sdl.set_gl_context_flags(video::GlContextFlags::FORWARD_COMPATIBLE).unwrap();

    let win_args = video::CreateWinArgs {
        title: WINDOW,
        width: WIDTH,
        height: HEIGHT,
        allow_high_dpi: true,
        borderless: false,
        resizable: false,
    };

    let window = sdl
        .create_gl_window(win_args)
        .expect("couldn't make a window and context");

    unsafe {
        load_gl_with(|f_name| window.get_proc_address(f_name as *const u8));
    }

    App {
        sdl,
        window,
        running: true,
    }
}

fn update(app: &mut App) {
    handle_event(app);
    render(app);
}

fn handle_event(app: &mut App) {
    while let Some(event) = app.sdl.poll_events() {
        match event {
            (events::Event::Quit, _) => app.running = false, _ => {}
        }
    }
}

fn render(app: &mut App) {
    unsafe {
        glClearColor(0.5, 0.3, 0.3, 1.0);
        glClear(GL_COLOR_BUFFER_BIT);
    }
    app.window.swap_window();
}