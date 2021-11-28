#include "rtweekend.h"

#include "color.h"
#include "hittable_list.h"
#include "sphere.h"
#include "camera.h"

#include <iostream>

// Functions

color ray_color(const ray& r, const hittable& world) {
    // Hit details
    hit_record rec;
    // If the ray hits the world
    if (world.hit(r, 0, infinity, rec)) {
        // Then colour by world colour map
        return 0.5 * (rec.normal + color(1,1,1));
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

    // World
    hittable_list world;
    // Add spheres to world
    world.add(make_shared<sphere>(point3(0, 0, -1), 0.5)); // Std. sphere
    world.add(make_shared<sphere>(point3(0, -100.5, -1), 100)); // Gnd.

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
                pixel_color += ray_color(r, world);
            }
            // Write value
            write_color(std::cout, pixel_color, samples_per_pixel);
        }
    }

    // Ding!
    std::cerr << "\nDone!\n";
}