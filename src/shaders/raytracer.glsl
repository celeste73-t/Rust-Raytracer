#version 430

struct Camera {
    vec3 cam_pos;
    vec3 cam_forward;
    float cam_fov;
};

struct Sphere {
    vec3 sphere_center;
    float sphere_radius;
    vec3 sphere_color;
};

struct Ray {
    vec3 origin;
    vec3 dir;
};

layout (local_size_x = 8, local_size_y = 8) in;

layout (rgba32f, binding = 0) uniform writeonly image2D out_image;

uniform int frame;

const Camera cam = Camera(vec3(-15.,0.,0.), vec3(1.,0.,0.), 90.);
const Sphere sphere[3] = Sphere[3](
    Sphere(vec3(0.,0.,0.), 5., vec3(1.,0.,0.)),
    Sphere(vec3(4.,-3.,3.), 5., vec3(0.,1.,0.)),
    Sphere(vec3(0.,3.,0.), 5., vec3(0.,0.,1.))
);

Ray generate_ray(vec2 uv) {
    float h = tan(cam.cam_fov / 2.0);
    float aspect = float(imageSize(out_image).x) / float(imageSize(out_image).y);
    float w = h * aspect;

    vec2 screen = uv * 2.0 - 1.0;

    vec3 forward = normalize(cam.cam_forward);
    vec3 up = vec3(0,1,0);
    vec3 right = normalize(cross(forward, up));
    up = normalize(cross(right, forward));

    vec3 ray_dir = normalize(forward + screen.x * w * right + screen.y * h * up);

    return Ray(cam.cam_pos, ray_dir);
}

bool intersect_sphere(Ray ray, Sphere sphere, out float t) {
    vec3 oc = ray.origin - sphere.sphere_center;
    float a = dot(ray.dir, ray.dir); // 1 si ray.dir normalisé
    float b = dot(oc, ray.dir);
    float c = dot(oc, oc) - sphere.sphere_radius * sphere.sphere_radius;
    float discriminant = b*b - a*c;

    if (discriminant < 0.0) {
        return false;
    }

    float t0 = -b - sqrt(discriminant);
    float t1 = -b + sqrt(discriminant);

    if (t0 > 0.0) {
        t = t0;
        return true;
    } else if (t1 > 0.0) {
        t = t1;
        return true;
    }
    return false;
}

void main() {
    ivec2 pix = ivec2(gl_GlobalInvocationID.xy);
    vec2 uv = vec2(pix) / vec2(imageSize(out_image));

    Ray ray = generate_ray(uv);

    vec3 color = vec3(0.0); // fond noir
    float closest_t = 1e20; // distance max
    vec3 sphere_color;

    for (int i = 0; i < sphere.length(); i++) {
        float t;
        if (intersect_sphere(ray, sphere[i], t)) {
            if (t < closest_t) {
                closest_t = t;
                color = sphere[i].sphere_color;
            }
        }
    }

    imageStore(out_image, pix, vec4(color, 1.0));
}


