#include "rtweekend.h"

#include "color.h"
#include "hittable_list.h"
#include "sphere.h"
#include "camera.h"
#include "material.h"

#include <iostream>

// Functions

color ray_color(const ray& r, const hittable& world, int depth) {
    // Hit details
    hit_record rec;
    // Depth limiting
    if (depth <= 0)
        return color(0, 0, 0);
    // If the ray hits the world
    // 0.001 values gives us some tolerance for imperfect hits
    if (world.hit(r, 0.001, infinity, rec)) {
        ray scattered;
        color attenutation;
        // If it scatters
        if (rec.mat_ptr->scatter(r, rec, attenutation, scattered))
            return attenutation * ray_color(scattered, world, depth - 1);
        // Else
        return color(0, 0, 0);
    }
    // Else colour by ray colour map
    // Get the direction the ray points
    vec3 unit_direction = unit_vector(r.direction());
    // Use it to generate a gradient
    auto t = 0.5*(unit_direction.y() + 1.0);
    // Linear interpolation (LERP) blendedValue = (1 - t) * startValue + t * endValue
    return (1.0 - t) * color(1.0, 1.0, 1.00) + t * color(0.5, 0.7, 1.0);
}

// Main

int main() {

    // Image Parameters
    const auto aspect_ratio = 16.0/9.0;
    const int image_width = 400;
    const int image_height = static_cast<int>(image_width / aspect_ratio);
    const int samples_per_pixel = 100;
    const int max_depth = 50;

    // World
    hittable_list world;
    // Set some materials
    auto material_ground = make_shared<lambertian>(color(0.8, 0.8, 0.0));
    auto material_center = make_shared<lambertian>(color(0.7, 0.3, 0.3));
    auto material_left   = make_shared<metal>(color(0.8, 0.8, 0.8));
    auto material_right  = make_shared<metal>(color(0.8, 0.6, 0.2));
    // Create spheres
    world.add(make_shared<sphere>(point3( 0.0, -100.5, -1.0), 100.0, material_ground));
    world.add(make_shared<sphere>(point3( 0.0,    0.0, -1.0),   0.5, material_center));
    world.add(make_shared<sphere>(point3(-1.0,    0.0, -1.0),   0.5, material_left));
    world.add(make_shared<sphere>(point3( 1.0,    0.0, -1.0),   0.5, material_right));

    // Camera
    camera cam;

    // Render
    // Setup file
    // P3 - Colours are ASCII
    // image_width image_height - Image dimensions
    // 255 - Max colour
    std::cout << "P3\n" << image_width << " " << image_height << "\n255\n";

    // From bottom up
    for (int j = image_height - 1; j >= 0; --j) {
        // Progress bar
        std::cerr << "\rScanlines remaining: " << j << " " << std::flush;
        // From left to right
        for (int i = 0; i < image_width; ++i) {
            // Initialise colour
            color pixel_color(0, 0, 0);
            // Take samples of pixel
            for (int s = 0; s < samples_per_pixel; ++s) {
                // Proportions of way through image
                // Plus some shake for anti-aliasing
                auto u = (i + random_double()) / (image_width - 1); // Proportion across
                auto v = (j + random_double()) / (image_height - 1); // Proportion down
                // Compute ray from proportions
                ray r = cam.get_ray(u, v);
                // Colour from ray
                pixel_color += ray_color(r, world, max_depth);
            }
            // Write value
            write_color(std::cout, pixel_color, samples_per_pixel);
        }
    }

    // Ding!
    std::cerr << "\nDone!\n";
}