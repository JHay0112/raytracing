#ifndef TRIANGLE_H
#define TRIANGLE_H

#include "hittable.h"
#include "vec3.h"

class triangle : public hittable {
    public:
        // Initilisation
        triangle() {}
        triangle(point3 v1, point3 v2, point3 v3, shared_ptr<material> m) : vertices{v1, v2, v3}, mat_ptr(m) {};

        virtual bool hit(const ray& r, double t_min, double t_max, hit_record& rec) const override;

    public:
        point3 vertices[3];
        shared_ptr<material> mat_ptr;
};

bool triangle::hit(const ray& r, double t_min, double t_max, hit_record& rec) const {
    // Find the plane's normal choosing v1 as origin
    vec3 n = cross(vertices[1] - vertices[0], vertices[2] - vertices[0]);
    vec3 r_0 = vertices[0];
    // Now can produce scalar form of plane
    // p.n = r_0.n
    // Where p is a point on the plane, p = (x, y, z)
    // n is the normal, n = (a, b, c)
    // r_0 is the origin of the plane.
    // Since p is produced by the ray
    // p = o + td (o = origin, t = scalar, d = direction)
    // So (o + td).n = r_0.n
    // By the dot products distributive property
    // o.n + td.n = r_0.n
    // Hence t = (r_0.n - o.n)/d.n
    //         = (r_0 - o).n/d.n
    double t = dot((r_0 - r.origin()), n)/dot(r.direction(), n);
    // Check t in range
    if (t < t_min || t_max < t)
        return false;
    // t is in range, sweet, lets now check that the point is in the triangle
    point3 p = r.at(t);
    // First a check from v0
    vec3 v0_v1 = vertices[1] - vertices[0];
    vec3 v0_v2 = vertices[2] - vertices[0];
    vec3 v0_p = p - vertices[0];
    double v0_angle = angle_between(v0_v1, v0_v2);
    double v0_p_angle = angle_between(v0_v1, v0_p);
    if (v0_p_angle > v0_angle)
        return false;
    // Check from v1
    vec3 v1_v0 = vertices[0] - vertices[1];
    vec3 v1_v2 = vertices[2] - vertices[1];
    vec3 v1_p = p - vertices[1];
    double v1_angle = angle_between(v1_v2, v1_v0);
    double v1_p_angle = angle_between(v1_v2, v1_p);
    if (v1_p_angle > v1_angle)
        return false;
    // Check from v2
    vec3 v2_v0 = vertices[0] - vertices[2];
    vec3 v2_v1 = vertices[1] - vertices[2];
    vec3 v2_p = p - vertices[2];
    double v2_angle = angle_between(v2_v0, v2_v1);
    double v2_p_angle = angle_between(v2_v0, v2_p);
    if (v2_p_angle > v2_angle)
        return false;
    // Passed this far must be in triangle
    // Record hit data
    rec.t = t;
    rec.p = p; // Where on the ray did the intersection occur
    rec.set_face_normal(r, unit_vector(n)); // Set the normal
    rec.mat_ptr = mat_ptr; // Set material
    // Yes it did hit
    return true;
}

#endif