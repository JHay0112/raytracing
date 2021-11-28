#ifndef HITTABLE_LIST_H
#define HITTABLE_LIST_H

#include "hittable.h"

#include<memory> // For shared pointers
#include<vector> // Generic arrays

using std::shared_ptr;
using std::make_shared;

class hittable_list : public hittable {
    /*
        Stores a list of hittables
    */
    public:
        // Initialisation
        hittable_list() {}
        hittable_list(shared_ptr<hittable> object) {add(object);}
        // Object Management
        void clear() {objects.clear();}
        void add(shared_ptr<hittable> object) {objects.push_back(object);}
        // Check for ray intersection
        virtual bool hit(const ray &r, double t_min, double t_max, hit_record& rec) const override;

    public:
        std::vector<shared_ptr<hittable>> objects;
};

// Check for ray intersection for all objects
bool hittable_list::hit(const ray &r, double t_min, double t_max, hit_record& rec) const {
    // Recording keeping
    hit_record temp_rec; // Record holder
    bool hit_anything = false; // If there has been a ray intersection
    auto closest_so_far = t_max; // The closest intersection so far
    // For all objects
    for (const auto& object : objects) {
        // If the ray intersects
        if (object->hit(r, t_min, closest_so_far, temp_rec)) {
            // Then we've hit something
            hit_anything = true;
            // Update records
            closest_so_far = temp_rec.t;
            rec = temp_rec;
        }
    }
    // Return result
    return hit_anything;
}

#endif