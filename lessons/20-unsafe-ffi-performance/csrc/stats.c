#include <stddef.h>

double c_mean(const double *values, size_t len) {
    if (len == 0) return 0.0;
    double sum = 0.0;
    for (size_t i = 0; i < len; i++) sum += values[i];
    return sum / (double)len;
}
