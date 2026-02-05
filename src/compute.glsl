#version 430

layout (local_size_x = 8, local_size_y = 8) in;

layout (rgba32f, binding = 0) uniform image2D img;

void main() {
    ivec2 pixel = ivec2(gl_GlobalInvocationID.xy);
    vec2 uv = vec2(pixel) / vec2(imageSize(img));

    vec3 color = vec3(uv, 0.0);
    imageStore(img, pixel, vec4(color, 1.0));
}
