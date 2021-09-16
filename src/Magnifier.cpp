#include "Magnifier.h"

#include <magnification.h>
#include <windows.h>

#include <algorithm>

const float Magnifier::DEFAULT_MAG_LEVEL = 1.5;

Magnifier::Magnifier(float mag_level, int dead_zone)
    : mag_level(mag_level), dead_zone(dead_zone), magnified(false) {
    // Need some exception
    MagInitialize();

    int width = GetSystemMetrics(SM_CXSCREEN);
    int height = GetSystemMetrics(SM_CYSCREEN);

    float mul_x = width * (1 - 1 / mag_level) / (width - 2 * dead_zone);
    float mul_y = height * (1 - 1 / mag_level) / (height - 2 * dead_zone);
    float sub_x = mul_x * dead_zone;
    float sub_y = mul_y * dead_zone;
    int max_x = static_cast<int>(width * (1 - 1 / mag_level));
    int max_y = static_cast<int>(height * (1 - 1 / mag_level));

    transform_x = [mul_x, sub_x, max_x](int x) {
        return std::clamp(static_cast<int>(x * mul_x - sub_x), 0, max_x);
    };
    transform_y = [mul_y, sub_y, max_y](int y) {
        return std::clamp(static_cast<int>(y * mul_y - sub_y), 0, max_y);
    };
}

Magnifier::~Magnifier() {
    // Need some exception
    MagUninitialize();
}

void Magnifier::magnify() const {
    if (!magnified) return;

    POINT p{};
    // Need some exception
    GetCursorPos(&p);

    // Need some exception
    int x = transform_x(p.x);
    int y = transform_y(p.y);
    MagSetFullscreenTransform(mag_level, x, y);

    RECT rcDest;
    rcDest.left = 0;
    rcDest.top = 0;
    rcDest.right = GetSystemMetrics(SM_CXSCREEN);
    rcDest.bottom = GetSystemMetrics(SM_CYSCREEN);

    RECT rcSource;
    rcSource.left = x;
    rcSource.top = y;
    rcSource.right = rcSource.left + (int)(rcDest.right / mag_level);
    rcSource.bottom = rcSource.top + (int)(rcDest.bottom / mag_level);

    // Need some exception
    MagSetInputTransform(TRUE, &rcSource, &rcDest);
}

void Magnifier::unmagnify() const {
    // Need some exception
    MagSetFullscreenTransform(1.0, 0, 0);
}