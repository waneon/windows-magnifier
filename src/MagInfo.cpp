#include "maginfo.h"

#include <windows.h>

const int DEFAULT_MAGNIFICATION_FACTOR = 2;
const int DEFAULT_DEAD_ZONE = 200;

MagInfo::MagInfo() {
    int x = GetSystemMetrics(SM_CXSCREEN);
    int y = GetSystemMetrics(SM_CYSCREEN);
    int k = DEFAULT_DEAD_ZONE;
    f = DEFAULT_MAGNIFICATION_FACTOR;

    mul_x = x * (1 - 1 / f) / (x - 2 * k);
    mul_y = y * (1 - 1 / f) / (y - 2 * k);
    sub_x = mul_x * k;
    sub_y = mul_y * k;
    max_x = x * (1 - 1 / f);
    max_y = y * (1 - 1 / f);
}

int MagInfo::TransformX(int x) const {
    int tx = x * mul_x - sub_x;

    if (tx < 0)
        tx = 0;
    else if (tx > max_x)
        tx = max_x;

    return tx;
}

int MagInfo::TransformY(int y) const {
    int ty = y * mul_y - sub_y;

    if (ty < 0)
        ty = 0;
    else if (ty > max_y)
        ty = max_y;

    return ty;
}

float MagInfo::GetMagFactor() const { return f; }