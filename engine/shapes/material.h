#ifndef MATERIAL_H
#define MATERIAL_H

#include "../../utils/vec3.h"

struct Material {
    Vec3 color;
    float k_ambient;
    float k_diffuse;
    float k_specular;
    float e;
    Material (): color(Vec3(1.0, 1.0, 1.0)), k_ambient(0.2), k_diffuse(0.7), k_specular(0.3), e(5.0) {};
    Material (Vec3 color) : color(color), k_ambient(0.2), k_diffuse(0.7), k_specular(0.3), e(5.0) {};
    Material (Vec3 color, float k_ambient, float k_diffuse, float k_specular, float e):
        color(color), k_ambient(k_ambient), k_diffuse(k_diffuse), k_specular(k_specular), e(e) {};
};

#endif