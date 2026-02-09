use beryllium::*;

use crate::gl;

pub struct App {
    pub sdl: Sdl,
    pub window: video::GlWindow,
    pub running: bool,
 
    pub gl: glow::Context, 
    pub compute: gl::compute::ComputePipeline, 
    pub display: gl::display::DisplayPipeline, 
    pub output_tex: glow::Texture, 
    
    pub frame: i32,
}

impl App {
    pub fn new() -> Self {
        let sdl = Sdl::init(init::InitFlags::EVERYTHING); 
        sdl.set_gl_context_major_version(4).unwrap(); 
        sdl.set_gl_context_minor_version(3).unwrap(); 
        sdl.set_gl_profile(video::GlProfile::Core).unwrap(); 
        
        let window = sdl.create_gl_window(video::CreateWinArgs { 
            title: "OpenGL + Rust", 
            width: 800, 
            height: 600, 
            allow_high_dpi: true, 
            borderless: false, 
            resizable: false, 
        }).unwrap(); 
        
        let gl = unsafe { 
            glow::Context::from_loader_function(|s| { 
                window.get_proc_address(s.as_ptr() as *const _)
            }) 
        }; 
        
        let output_tex = gl::texture::create_storage_texture(&gl, 800, 600); 
        
        let compute = gl::compute::ComputePipeline::new(&gl); 
        let display = gl::display::DisplayPipeline::new(&gl);

        Self { 
            sdl, 
            window, 
            running: true,
            
            gl, 
            compute, 
            display, 
            output_tex, 

            frame: 0, 
        }
    }

    pub fn update(&mut self) {
        self.handle_event(); 
        self.render(); 
        self.frame += 1;
    }

    fn handle_event(&mut self) {
        while let Some(event) = self.sdl.poll_events() {
            if let (events::Event::Quit, _) = event {
                self.running = false;
            }
        }
    }

    fn render(&mut self) {
        self.compute.run(&self.gl, self.output_tex, self.frame); // ajoute la scene (self.scene)
        self.display.draw(&self.gl, self.output_tex);

        self.window.swap_window();
    }
}