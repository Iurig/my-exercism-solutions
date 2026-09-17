#include "reverse_string.h"

#include <string>

namespace reverse_string {

std::string reverse_string(std::string input) {
    for (std::string::size_type i = 0; i < input.length() / 2; ++i) {
        std::swap(input[i], input[input.length() - 1 - i]);
    }
    return input;
}

}  // namespace reverse_string
