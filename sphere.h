#ifndef SPHERE_H
#define SPHERE_H

#include "hittable.h"
#include "vec3.h"

class sphere : public hittable {
    public:
        // Initilisation
        sphere() {}
        sphere(point3 cen, double r) : center(cen), radius(r) {};

        virtual bool hit(const ray& r, double t_min, double t_max, hit_record& rec) const override;

    public:
        point3 center;
        double radius;
};

bool sphere::hit(const ray& r, double t_min, double t_max, hit_record& rec) const {
    // Distance vec from origin to centre
    vec3 oc = r.origin() - center;
    // at^2 + bt + c = 0
    auto a = dot(r.direction(), r.direction()); // If a point is on the sphere then this == (radius)^2
    auto half_b = dot(oc, r.direction()); 
    auto c = oc.length_squared() - radius*radius;
    // Discriminant of quadratic
    auto discriminant = half_b*half_b - a*c;
    // No roots, did not hit
    if (discriminant < 0) return false;
    auto sqrtd = sqrt(discriminant);

    // Find a root in acceptable range
    auto root = (-half_b - sqrtd) / a;
    // If first root outside range
    if (root < t_min || t_max < root) {
        // Get second root
        root = (-half_b + sqrtd) / a;
        // If also outside then do not register hit
        if (root < t_min || t_max < root) return false;
    }

    // Record hit data
    rec.t = root;
    rec.p = r.at(rec.t);
    vec3 outward_normal = (rec.p - center) / radius; // A normal facing away from the surface
    rec.set_face_normal(r, outward_normal); // Set the normal
    // Yes it did hit
    return true;
}

#endif