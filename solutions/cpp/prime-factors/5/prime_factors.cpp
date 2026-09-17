#include "prime_factors.h"

namespace prime_factors {

std::vector<long long> of(long long n) {
    std::vector<long long> factors{};

    while (n % 2 == 0) {
        factors.push_back(2);
        n /= 2;
    }

    for (long long candidate = 3; candidate <= n / candidate; candidate += 2) {
        while (n % candidate == 0) {
            n /= candidate;
            factors.push_back(candidate);
        }
    }
    if (n > 1) {
        factors.push_back(n);
    }
    return factors;
}

}  // namespace prime_factors
