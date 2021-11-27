#include "color.h"
#include "ray.h"
#include "vec3.h"

#include <iostream>

// Functions

double hit_sphere(const point3& center, double radius, const ray& r) {
    // Computes whether the ray hits a sphere by solving vector quadratic
    vec3 oc = r.origin() - center; // Distance vector between origin and sphere center
    // at^2 + bt + c = 0
    auto a = dot(r.direction(), r.direction()); // If a point is on the sphere then this == (radius)^2
    auto half_b = dot(oc, r.direction()); 
    auto c = oc.length_squared() - radius*radius;
    auto discriminant = half_b*half_b - a*c;
    
    if (discriminant < 0) {
        // If there are no roots
        // i.e. no intersection
        return -1.0;
    } else {
        // There is an intersection
        // Return t value
        return (-half_b - sqrt(discriminant)) / a;
    }
}

color ray_color(const ray& r) {
    // Computes the background color
    auto t = hit_sphere(point3(0, 0, -1), 0.5, r);
    // If the ray hits the sphere
    if (t > 0.0) {
        // Calculate normal
        // r.at(t) is the point where the intersects the sphere
        // The sphere origin is then subtracted to give the normal
        vec3 N = unit_vector(r.at(t) - vec3(0, 0, -1));
        return 0.5 * color(N.x() + 1, N.y() + 1, N.z() + 1);
    }
    // Else
    // Get the direction the ray points
    vec3 unit_direction = unit_vector(r.direction());
    // Use it to generate a gradient
    t = 0.5*(unit_direction.y() + 1.0);
    // Linear interpolation (LERP) blendedValue = (1 - t) * startValue + t * endValue
    return (1.0 - t) * color(1.0, 1.0, 1.00) + t * color(0.5, 0.7, 1.0);
}

// Main

int main() {

    // Image Parameters
    const auto aspect_ratio = 16.0 / 9.0;
    const int image_width = 400;
    const int image_height = static_cast<int>(image_width / aspect_ratio);

    // Camera Parameters
    auto viewport_height = 2.0;
    auto viewport_width = aspect_ratio * viewport_height;
    auto focal_length = 1.0;
    // Camera Position
    auto origin = point3(0, 0, 0);
    // Camera Orientation
    auto horizontal = vec3(viewport_width, 0, 0);
    auto vertical = vec3(0, viewport_height, 0);
    auto lower_left_corner = origin - horizontal/2 - vertical/2 - vec3(0, 0, focal_length);

    // Render
    // Setup file
    // P3 - Colours are ASCII
    // image_width image_height - Image dimensions
    // 255 - Max colour
    std::cout << "P3\n" << image_width << " " << image_height << "\n255\n";

    // From bottom up
    for (int j = image_height - 1; j >= 0; --j) {
        // Progress bar
        std::cerr << "\rScanlines remaining: " << j << std::flush;
        // From left to right
        for (int i = 0; i < image_width; ++i) {
            // Proportions of way through image
            auto u = double(i) / (image_width - 1); // Proportion across
            auto v = double(j) / (image_height - 1); // Proportion down
            // Compute ray from proportions
            ray r(origin, lower_left_corner + u*horizontal + v*vertical - origin);
            // Colour from ray
            color pixel_color = ray_color(r);
            // Write value
            write_color(std::cout, pixel_color);
        }
    }

    // Ding!
    std::cerr << "\nDone!\n";
}