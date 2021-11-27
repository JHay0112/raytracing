#include "color.h"
#include "vec3.h"

#include <iostream>

int main() {

    // Making ppm image

    // Image Parameters
    const int image_width = 256;
    const int image_height = 256;

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
            // Colour components
            color pixel_color(double(i) / (image_width - 1), double(j) / (image_width - 1), 0.25);
            // Write value
            write_color(std::cout, pixel_color);
        }
    }

    // Ding!
    std::cerr << "\nDone!\n";
}