use glow::*;

pub struct DisplayPipeline {
    pub program: Program,
    pub vao: VertexArray,
}

impl DisplayPipeline {
    pub fn new(gl: &Context) -> Self {
        let vs = include_str!("../shaders/display.vert");
        let fs = include_str!("../shaders/display.frag");

        let program = create_program(gl, vs, fs) ;

        let vao = unsafe { gl.create_vertex_array().unwrap() };

        Self { program, vao }
    }

    pub fn draw(&self, gl: &Context, tex: Texture) {
        unsafe {
            gl.use_program(Some(self.program));
            gl.bind_vertex_array(Some(self.vao));

            gl.active_texture(TEXTURE0);
            gl.bind_texture(TEXTURE_2D, Some(tex));

            if let Some(loc) = gl.get_uniform_location(self.program, "tex") {
                gl.uniform_1_i32(Some(&loc), 0);
            }

            gl.draw_arrays(TRIANGLES, 0, 3);
        }
    }
}

fn create_program(gl: &Context, vs_src: &str, fs_src: &str) -> Program {
    unsafe {
        let vs = gl.create_shader(VERTEX_SHADER).unwrap();
        gl.shader_source(vs, vs_src);
        gl.compile_shader(vs);

        let fs = gl.create_shader(FRAGMENT_SHADER).unwrap();
        gl.shader_source(fs, fs_src);
        gl.compile_shader(fs);

        let program = gl.create_program().unwrap();
        gl.attach_shader(program, vs);
        gl.attach_shader(program, fs);
        gl.link_program(program);

        gl.delete_shader(vs);
        gl.delete_shader(fs);

        program
    }
}
