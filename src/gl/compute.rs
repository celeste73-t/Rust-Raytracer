use glow::*;

pub struct ComputePipeline {
    pub program: Program,
}

impl ComputePipeline {
    pub fn new(gl: &Context) -> Self {
        let src = include_str!("../shaders/raytracer.glsl");
        let program = create_compute_program(gl, &src);
        Self { program }
    }

    pub fn run(&self, gl: &Context, _tex: Texture, frame: i32) {
        unsafe {
            gl.use_program(Some(self.program));

            if let Some(loc) = gl.get_uniform_location(self.program, "frame") {
                gl.uniform_1_i32(Some(&loc), frame);
            }

            let gx = (800 + 7) / 8;
            let gy = (600 + 7) / 8;

            gl.dispatch_compute(gx as u32, gy as u32, 1);
            gl.memory_barrier(SHADER_IMAGE_ACCESS_BARRIER_BIT);
        }
    }
}

fn create_compute_program(gl: &Context, src: &str) -> Program {
    unsafe {
        let shader = gl.create_shader(COMPUTE_SHADER).unwrap();
        gl.shader_source(shader, src);
        gl.compile_shader(shader);

        if !gl.get_shader_compile_status(shader) {
            panic!("{}", gl.get_shader_info_log(shader));
        }

        let program = gl.create_program().unwrap();
        gl.attach_shader(program, shader);
        gl.link_program(program);

        if !gl.get_program_link_status(program) {
            panic!("{}", gl.get_program_info_log(program));
        }

        gl.delete_shader(shader);
        program
    }
}
