#include "prime_factors.h"

#include <math.h>

#include <vector>

namespace prime_factors {

std::vector<int> of(int n) {
    std::vector<int> factors{};
    int p = 2;

    while (n != 1 && p <= sqrt(n)) {
        if (n % p == 0) {
            n /= p;
            factors.push_back(p);
        }
        p++;
    }
    if (n > 1) {
        factors.push_back(n);
    }
    return factors;
}

}  // namespace prime_factors
