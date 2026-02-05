use beryllium::*;
use glow::*;

const WINDOW: &str = "OpenGL + Rust";
const WIDTH: i32 = 800;
const HEIGHT: i32 = 600;

struct App {
    sdl: Sdl,
    window: video::GlWindow,
    running: bool,

    compute_program: glow::Program,
    output_tex: glow::Texture,
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

    let gl = unsafe {
        glow::Context::from_loader_function(|s| {
            window.get_proc_address(s.as_ptr() as *const _)
        })
    };

    let mut output_tex = 0;
    unsafe {
        glGenTextures(1, &mut output_tex);
        glBindTexture(GL_TEXTURE_2D, output_tex);

        TexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_NEAREST as _);
        glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_NEAREST as _);

        glTexImage2D(
            GL_TEXTURE_2D,
            0,
            GL_RGBA32F as _,
            WIDTH,
            HEIGHT,
            0,
            GL_RGBA,
            GL_FLOAT,
            std::ptr::null(),
        );

        glBindImageTexture(
            0,
            output_tex,
            0,
            GL_FALSE,
            0,
            GL_WRITE_ONLY,
            GL_RGBA32F,
        );
    }

    let compute_src = include_str!("compute.glsl");
    let compute_program = unsafe { create_compute_program(compute_src) };


    App {
        sdl,
        window,
        running: true,

        compute_program,
        output_tex,
    }
}

unsafe fn create_compute_program(src: &str) -> u32 {
    let shader = glCreateShader(GL_COMPUTE_SHADER);
    glShaderSource(shader, 1, &src.as_ptr().cast(), std::ptr::null());
    glCompileShader(shader);

    let program = glCreateProgram();
    glAttachShader(program, shader);
    glLinkProgram(program);

    glDeleteShader(shader);
    program
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
        glUseProgram(app.compute_program);

        let gx = (WIDTH as u32 + 7) / 8;
        let gy = (HEIGHT as u32 + 7) / 8;
        glDispatchCompute(gx, gy, 1);

        glMemoryBarrier(GL_SHADER_IMAGE_ACCESS_BARRIER_BIT);
    }
    app.window.swap_window();
}