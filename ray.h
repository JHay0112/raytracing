#ifndef RAY_H
#define RAY_H

#include "vec3.h"

class ray {
    /*
        Ray/Line
    */
    public:
        // Initilisation
        ray() {}
        ray(const point3& origin, const vec3& direction) 
            : orig(origin), dir(direction)
        {}

        // Values
        point3 origin() const {return orig;}
        vec3 direction() const {return dir;}

        // Calculates the point at a distance along the ray
        point3 at(double t) const {
            return orig + t*dir;
        }
    
    public:
        point3 orig;
        vec3 dir;
};

#endif