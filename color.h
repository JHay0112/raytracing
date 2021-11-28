#ifndef COLOR_H
#define COLOR_H

#include "vec3.h"

#include <iostream>

void write_color(std::ostream &out, color pixel_color, int samples_per_pixel) {
    /*
        Takes a pixel and outputs it per PPM.
        If multiple samples have been taken of the pixel then it is scaled.
    */

    // Get colours of pixel
    auto r = pixel_color.x();
    auto g = pixel_color.y();
    auto b = pixel_color.z();

    // Scale by the amount of samples that have been taken on the pixel
    // Gamma correct by raising to 1/gamma
    auto scale = 1.0/samples_per_pixel;
    r = sqrt(r * scale);
    g = sqrt(g * scale);
    b = sqrt(b * scale);

    // Write out a color translated to [0, 255]
    out << static_cast<int>(255.999 * clamp(r, 0.0, 0.999)) << " "
        << static_cast<int>(255.999 * clamp(g, 0.0, 0.999)) << " "
        << static_cast<int>(255.999 * clamp(b, 0.0, 0.999)) << "\n";
}

#endif