use glow::*;

pub fn create_storage_texture(gl: &Context, w: i32, h: i32) -> Texture {
    unsafe {
        let tex = gl.create_texture().unwrap();
        gl.bind_texture(TEXTURE_2D, Some(tex));

        gl.tex_parameter_i32(TEXTURE_2D, TEXTURE_MIN_FILTER, NEAREST as i32);
        gl.tex_parameter_i32(TEXTURE_2D, TEXTURE_MAG_FILTER, NEAREST as i32);

        gl.tex_image_2d(
            TEXTURE_2D,
            0,
            RGBA32F as i32,
            w,
            h,
            0,
            RGBA,
            FLOAT,
            None,
        );

        gl.bind_image_texture(0, tex, 0, false, 0, WRITE_ONLY, RGBA32F);

        tex
    }
}
