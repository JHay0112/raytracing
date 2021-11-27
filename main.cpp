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
        // From left to right
        for (int i = 0; i < image_width; ++i) {
            // Colour components
            auto r = double(i) / (image_width - 1); // Approaches 1
            auto g = double(j) / (image_height - 1); // Approaches 0
            auto b = 0.25; // Const

            int ir = static_cast<int>(255.999 * r);
            int ig = static_cast<int>(255.999 * g);
            int ib = static_cast<int>(255.999 * b);

            // Write values
            std::cout << ir << " " << ig << " " << ib << "\n";
        }
    }
}