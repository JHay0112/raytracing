#include "rtweekend.h"

#include "color.h"
#include "hittable_list.h"
#include "sphere.h"
#include "triangle.h"
#include "camera.h"
#include "material.h"

#include <iostream>
#include <thread>
#include <future>
#include <vector>

// Structs

struct scene {
    scene(const int w = 400, const double ar = 16.0/9.0, const int spp = 100, const int md = 50) : width(w), aspect_ratio(ar), height(static_cast<int>(w/ar)), samples_per_pixel(spp), max_depth(md) {}
    const int width;
    const int height;
    const int samples_per_pixel;
    const double aspect_ratio;
    const int max_depth;
    camera cam;
};

// Functions

color ray_color(const ray& r, const hittable& objects, int depth) {
    // Hit details
    hit_record rec;
    // Depth limiting
    if (depth <= 0)
        return color(0, 0, 0);
    // If the ray hits the world
    // 0.001 values gives us some tolerance for imperfect hits
    if (objects.hit(r, 0.001, infinity, rec)) {
        ray scattered;
        color attenutation;
        // If it scatters
        if (rec.mat_ptr->scatter(r, rec, attenutation, scattered))
            return attenutation * ray_color(scattered, objects, depth - 1);
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

std::vector<color> scanline(const int line, const scene& image, const hittable_list& objects) {
    // Store pixels
    std::vector<color> pixels;

    // From left to right
    for (int i = 0; i < image.width; ++i) {
        // Initialise colour
        color pixel(0, 0, 0);
        // Take samples of pixel
        for (int s = 0; s < image.samples_per_pixel; ++s) {
            // Proportions of way through image
            // Plus some shake for anti-aliasing
            auto u = (i + random_double()) / (image.width - 1); // Proportion across
            auto v = (line + random_double()) / (image.height - 1); // Proportion down
            // Compute ray from proportions
            ray r = image.cam.get_ray(u, v);
            // Colour from ray
            pixel += ray_color(r, objects, image.max_depth);
        }
        // Add pixel to vector
        pixels.push_back(pixel);
    }
    
    return pixels;
}

// Main

int main() {

    // Image Parameters
    struct scene image = {1920};
    hittable_list objects;
    // Set some materials
    auto material_ground = make_shared<lambertian>(color(0.8, 0.8, 0.0));
    auto material_center = make_shared<lambertian>(color(0.7, 0.3, 0.3));
    auto material_left   = make_shared<metal>(color(0.8, 0.8, 0.8), 0.3);
    auto material_right  = make_shared<metal>(color(0.8, 0.6, 0.2), 1.0);
    // Create objects
    objects.add(make_shared<sphere>(point3(0.0, -100.5, -1.0), 100.0, material_ground));
    objects.add(make_shared<triangle>(point3(0.0, 0.25, -1.0), point3(-0.8, -0.8, -1.5), point3(-0.1, -1.0, -1.0), material_left));
    objects.add(make_shared<triangle>(point3(0.0, 0.25, -1.0), point3(1.0, -1.0, -1.5), point3(-0.1, -1.0, -1.0), material_left));

    // Pixels
    std::future<std::vector<color>> pixels[image.height];

    // Render
    // Setup file
    // P3 - Colours are ASCII
    // image_width image_height - Image dimensions
    // 255 - Max colour
    std::cout << "P3\n" << image.width << " " << image.height << "\n255\n";

    // From bottom up
    for (int j = image.height - 1; j >= 0; --j) {
        pixels[j] = std::async(scanline, j, image, objects);
    }

    // From bottom up
    for (int j = image.height - 1; j >= 0; --j) {
        std::vector<color> line = pixels[j].get();
        // From left to right
        for (int i = 0; i < image.width; ++i) {
            write_color(std::cout, line[i], image.samples_per_pixel);
        }
    }

    // Ding!
    std::cerr << "Done!\n";
}