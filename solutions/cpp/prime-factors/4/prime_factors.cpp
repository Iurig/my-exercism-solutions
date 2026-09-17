#include "prime_factors.h"

#include <math.h>

namespace prime_factors {

std::vector<long long> of(long long n) {
    std::vector<long long> factors{};
    long long p = 2;

    while (n != 1 && p <= sqrt(n)) {
        if (n % p == 0) {
            n /= p;
            factors.push_back(p);
        } else if (p == 2) {
            p++;
        } else
            p += 2;
    }
    if (n > 1) {
        factors.push_back(n);
    }
    return factors;
}

}  // namespace prime_factors
