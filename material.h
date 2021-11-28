#ifndef MATERIAL_H
#define MATERIAL_H

#include "rtweekend.h"
#include "hittable.h"

// Generic

class material {
    /*
        Describes a material and how it causes light to scatter
    */
    public:
        virtual bool scatter(
            const ray& r_in, const hit_record& rec, color& attentuation, ray& scattered
        ) const = 0;
};

// Classes

class lambertian : public material {
    /*
        Lambertian/Diffuse Materials
    */
    public:
        // Initialisation
        lambertian(const color& a) : albedo(a) {}

        virtual bool scatter(
            const ray& r_in, const hit_record& rec, color& attentuation, ray& scattered
        ) const override {
            // Choose the scatter direction randomly.
            // Note: This could cause trouble if the random unit vector is near opposi
            auto scatter_direction = rec.normal + random_unit_vector(); 

            // Catch for aformentioned case
            if (scatter_direction.near_zero())
                scatter_direction = rec.normal;

            scattered = ray(rec.p, scatter_direction); // Create a scattered ray
            attentuation = albedo;
            return true;
        }

    public:
        color albedo;
};

class metal : public material {
    /*
        Metal/Reflective Materials
    */
    public:
        // Initialisation
        metal(const color& a) : albedo(a) {}

        // Scattering
        virtual bool scatter(
            const ray& r_in, const hit_record& rec, color& attenuation, ray& scattered
        ) const override {
            // Calculate the reflected direction
            vec3 reflected = reflect(unit_vector(r_in.direction()), rec.normal);
            // Create a ray from it
            scattered = ray(rec.p, reflected);
            attenuation = albedo;
            return (dot(scattered.direction(), rec.normal) > 0);
        }
    public:
        color albedo;
};

#endif