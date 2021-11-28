#ifndef HITTABLE_H
#define HITTABLE_H

#include "rtweekend.h"

class material;

struct hit_record {
    point3 p;
    vec3 normal;
    shared_ptr<material> mat_ptr;
    double t;
    bool front_face;

    inline void set_face_normal(const ray& r, const vec3& outward_normal) {
        front_face = dot(r.direction(), outward_normal) < 0; // If ray and outward normal are less than 90 degrees apart
        normal = front_face ? outward_normal :-outward_normal; // If its the front face then leave the normal outward, else have it point inward
    }
};

class hittable {
    public:
        virtual bool hit(const ray& r, double t_min, double t_max, hit_record& rec) const = 0;
};

#endif