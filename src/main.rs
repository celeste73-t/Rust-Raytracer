use beryllium::*;
use glow::*;

const WINDOW: &str = "OpenGL + Rust";
const WIDTH: i32 = 800;
const HEIGHT: i32 = 600;

struct App {
    sdl: Sdl,
    window: video::GlWindow,
    running: bool,

    gl: glow::Context,
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

    let gl: Context = unsafe {
        glow::Context::from_loader_function(|s| {
            window.get_proc_address(s.as_ptr() as *const _)
        })
    };

    let output_tex = unsafe { 
        let tex = gl.create_texture().unwrap(); 
        gl.bind_texture(glow::TEXTURE_2D, Some(tex)); 
        
        gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MIN_FILTER, glow::NEAREST as i32); 
        gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MAG_FILTER, glow::NEAREST as i32); 
        
        gl.tex_image_2d( 
            glow::TEXTURE_2D, 
            0, 
            glow::RGBA32F as i32, 
            WIDTH, 
            HEIGHT, 
            0, 
            glow::RGBA, 
            glow::FLOAT, 
            None, 
        ); 
        
        gl.bind_image_texture( 
            0, 
            tex, 
            0, 
            false, 
            0, 
            glow::WRITE_ONLY, 
            glow::RGBA32F, 
        ); 
        
        tex
    };

    let compute_src = include_str!("compute.glsl");
    let compute_program = create_compute_program(&gl, compute_src);


    App {
        sdl,
        window,
        running: true,

        gl,
        compute_program,
        output_tex,
    }
}

fn create_compute_program(gl: &glow::Context, src: &str) -> glow::Program {
    unsafe {
        let shader = gl.create_shader(glow::COMPUTE_SHADER).unwrap();
        gl.shader_source(shader, src);
        gl.compile_shader(shader);

        if !gl.get_shader_compile_status(shader) {
            panic!("Compute shader error: {}", gl.get_shader_info_log(shader));
        }

        let program = gl.create_program().unwrap();
        gl.attach_shader(program, shader);
        gl.link_program(program);

        if !gl.get_program_link_status(program) { 
            panic!("Program link error: {}", gl.get_program_info_log(program)); 
        }

        gl.delete_shader(shader);
        program
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
    let gl = &app.gl;
    unsafe {
        gl.use_program(Some(app.compute_program));

        let gx = (WIDTH as u32 + 7) / 8;
        let gy = (HEIGHT as u32 + 7) / 8;
        gl.dispatch_compute(gx, gy, 1);

        gl.memory_barrier(glow::SHADER_IMAGE_ACCESS_BARRIER_BIT);
    }
    app.window.swap_window();
}