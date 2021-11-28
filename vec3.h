#ifndef VEC3_H
#define VEC3_H

#include "rtweekend.h"

#include <cmath>
#include <iostream>

using std::sqrt;

class vec3 {
    /*
        3 Dimensional Vector
    */
    public:
        // Initalisation

        vec3() : e{0, 0, 0} {}
        vec3(double e0, double e1, double e2) : e{e0, e1, e2} {}

        // Aliases

        double x() const {return e[0];}
        double y() const {return e[1];}
        double z() const {return e[2];}

        // Operators

        // Negative
        vec3 operator-() const {return vec3(-e[0], -e[1], -e[2]);}
        // Indexing
        double operator[](int i) const {return e[i];}
        double& operator[](int i) {return e[i];}

        // Arithmetic Operators

        // Addition
        vec3& operator+=(const vec3 &v) {
            e[0] += v.e[0];
            e[1] += v.e[1];
            e[2] += v.e[2];
            return *this;
        }

        // Scaling

        // Multiplication
        vec3& operator*=(const double t) {
            e[0] *= t;
            e[1] *= t;
            e[2] *= t;
            return *this;
        }

        // Division
        vec3& operator/=(const double t) {
            return *this *= 1/t;
        }

        // Methods
        double length() const {
            return sqrt(length_squared());
        }

        double length_squared() const {
            return e[0]*e[0] + e[1]*e[1] + e[2]*e[2];
        }

        bool near_zero() const {
            // Check if the vector is close to zero
            const auto s = 1e-8;
            return (fabs(e[0]) < s) && (fabs(e[1]) < s) && (fabs(e[2]) < s);
        }

        // Random
        inline static vec3 random() {
            return vec3(random_double(), random_double(), random_double());
        }

        inline static vec3 random(double min, double max) {
            return vec3(random_double(min, max), random_double(min, max), random_double(min, max));
        }
    public:
        double e[3];
};

// Type Aliases
using point3 = vec3;
using color = vec3;

// Utility Functions for vec3

// Print behaviour
inline std::ostream& operator<<(std::ostream &out, const vec3 &v) {
    return out << v.e[0] << " " << v.e[1] << " " << v.e[2];
}

// Addition
inline vec3 operator+(const vec3 &u, const vec3 &v) {
    return vec3(u.e[0] + v.e[0], u.e[1] + v.e[1], u.e[2] + v.e[2]);
}

// Subtraction
inline vec3 operator-(const vec3 &u, const vec3 &v) {
    return vec3(u.e[0] - v.e[0], u.e[1] - v.e[1], u.e[2] - v.e[2]);
}

// Piecewise Multiplication
inline vec3 operator*(const vec3 &u, const vec3 &v) {
    return vec3(u.e[0] * v.e[0], u.e[1] * v.e[1], u.e[2] * v.e[2]);
}

// Factor Scaling
inline vec3 operator*(double t, const vec3 &v) {
    return vec3(t*v.e[0], t*v.e[1], t*v.e[2]);
}

// (Reversed) Factor Scaling
inline vec3 operator*(const vec3 &v, double t) {
    return t * v;
}

// Division
inline vec3 operator/(vec3 v, double t) {
    return (1/t) * v;
}

// Dot Product
inline double dot(const vec3 &u, const vec3 &v) {
    return u.e[0] * v.e[0]
         + u.e[1] * v.e[1]
         + u.e[2] * v.e[2];
}

// Cross Product
inline vec3 cross(const vec3 &u, const vec3 &v) {
    return vec3(u.e[1] * v.e[2] - u.e[2] * v.e[1],
                u.e[2] * v.e[0] - u.e[0] * v.e[2],
                u.e[0] * v.e[1] - u.e[1] * v.e[0]);
}

// Unit Vector
inline vec3 unit_vector(vec3 v) {
    return v / v.length();
}

// Random
inline vec3 random_in_unit_sphere() {
    while (true) {
        // Random vector in cube around unit circle
        auto p = vec3::random(-1, 1);
        // If length squared is greater than the radius
        if (p.length_squared() >= 1) continue; // Go around again
        // Else it's in the sphere hurrah!
        return p;
    }
}

vec3 random_unit_vector() {
    return unit_vector(random_in_unit_sphere());
}

#endif