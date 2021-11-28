#ifndef RTWEEKEND_H
#define RTWEEKEND_H

// Named rtweekend for ray tracing in a weekend
// Just in case I forget

#include<cmath>
#include<limits>
#include<memory>

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

// Common header files

#include "ray.h"
#include "vec3.h"

#endif