#ifndef RTWEEKEND_H
#define RTWEEKEND_H

// Named rtweekend for ray tracing in a weekend
// Just in case I forget

#include<cmath>
#include<limits>
#include<memory>
#include<random>

using std::shared_ptr;
using std::make_shared;
using std::sqrt;

// Constants

const double infinity = std::numeric_limits<double>::infinity();
const double pi = 3.1415926535897932385;

// Utility Functions

inline double degrees_to_radians(double degrees) {
    return degrees * pi / 180.0;
}

inline double random_double() {
    // Set distribution
    static std::uniform_real_distribution<double> distribution(0.0, 1.0);
    // Use MT19937 generator
    static std::mt19937 generator;
    // Get number from distribution and return
    return distribution(generator);
}

inline double random_double(double min, double max) {
    // Returns a random real in [min,max).
    return min + (max-min)*random_double();
}

inline double clamp(double x, double min, double max) {
    // Ensures a value is within a range
    if (x < min) return min;
    if (x > max) return max;
    return x;
}

// Common header files

#include "ray.h"
#include "vec3.h"

#endif