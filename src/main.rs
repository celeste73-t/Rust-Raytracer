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
    frame:i32, // dummy value
    display_program: glow::Program, 
    vao: glow::VertexArray,
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

    let compute_src = include_str!("shader/compute.glsl");
    let compute_program = create_compute_program(&gl, compute_src);

    let vs_src = include_str!("shader/display.vert"); 
    let fs_src = include_str!("shader/display.frag"); 
    let display_program = create_program(&gl, vs_src, fs_src); 
    
    // VAO obligatoire en Core Profile 
    
    let vao = unsafe { gl.create_vertex_array().unwrap() }; unsafe { gl.bind_vertex_array(Some(vao)); }

    let frame = 0;

    App {
        sdl,
        window,
        running: true,

        gl,
        compute_program,
        output_tex,
        frame,
        display_program, 
        vao,
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

fn create_program(gl: &glow::Context, vs_src: &str, fs_src: &str) -> glow::Program {
    unsafe {
        let vs = gl.create_shader(glow::VERTEX_SHADER).unwrap();
        gl.shader_source(vs, vs_src);
        gl.compile_shader(vs);
        if !gl.get_shader_compile_status(vs) {
            panic!("VS error: {}", gl.get_shader_info_log(vs));
        }

        let fs = gl.create_shader(glow::FRAGMENT_SHADER).unwrap();
        gl.shader_source(fs, fs_src);
        gl.compile_shader(fs);
        if !gl.get_shader_compile_status(fs) {
            panic!("FS error: {}", gl.get_shader_info_log(fs));
        }

        let program = gl.create_program().unwrap();
        gl.attach_shader(program, vs);
        gl.attach_shader(program, fs);
        gl.link_program(program);

        if !gl.get_program_link_status(program) {
            panic!("Link error: {}", gl.get_program_info_log(program));
        }

        gl.delete_shader(vs);
        gl.delete_shader(fs);

        program
    }
}


fn update(app: &mut App) {
    handle_event(app);
    render(app);
    app.frame += 1;
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
        let gl = &app.gl;

        gl.use_program(Some(app.compute_program));

        // test de passage de donnée
        let frame_loc = gl.get_uniform_location(app.compute_program, "frame"); 
        if let Some(loc) = frame_loc { 
            gl.uniform_1_i32(Some(&loc), app.frame); 
        }

        let gx = (WIDTH as u32 + 7) / 8;
        let gy = (HEIGHT as u32 + 7) / 8;
        gl.dispatch_compute(gx, gy, 1);

        gl.memory_barrier(glow::SHADER_IMAGE_ACCESS_BARRIER_BIT);

        gl.use_program(Some(app.display_program)); 
        gl.bind_vertex_array(Some(app.vao)); 
        
        gl.active_texture(glow::TEXTURE0); 
        gl.bind_texture(glow::TEXTURE_2D, Some(app.output_tex)); 
        
        let loc = gl.get_uniform_location(app.display_program, "tex"); 
        if let Some(loc) = loc { 
            gl.uniform_1_i32(Some(&loc), 0); 
        } 
        
        gl.draw_arrays(glow::TRIANGLES, 0, 3);
    }
    app.window.swap_window();
}