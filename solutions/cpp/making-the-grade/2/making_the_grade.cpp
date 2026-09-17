#include <array>
#include <iomanip>
#include <iostream>
#include <sstream>
#include <string>
#include <vector>

// Round down all provided student scores.
std::vector<int> round_down_scores(std::vector<double> student_scores) {
    std::vector<int> rounded_scores{};
    for (int i = 0; i < student_scores.size(); ++i) {
        rounded_scores.push_back(static_cast<int>(student_scores.at(i)));
    }
    return rounded_scores;
}

// Count the number of failing students out of the group provided.
int count_failed_students(std::vector<int> student_scores) {
    int failed_students = 0;
    for (int i = 0; i < student_scores.size(); ++i) {
        if (student_scores.at(i) <= 40) ++failed_students;
    }
    return failed_students;
}

// Create a list of grade thresholds based on the provided highest grade.
std::array<int, 4> letter_grades(int highest_score) {
    std::array<int, 4> cuttoffs{41, 0, 0, 0};
    const int step = (highest_score - 41) / 4;
    const int exceptions = (highest_score - 41) % 4;
    for (int i = 0; i < 3; ++i) {
        cuttoffs[i + 1] = cuttoffs[i] + step + (i < exceptions);
    }
    return cuttoffs;
}

// Organize the student's rank, name, and grade information in ascending order.
std::vector<std::string> student_ranking(
    std::vector<int> student_scores, std::vector<std::string> student_names) {
    std::vector<std::string> ranking{};
    for (int i = 0; i < student_names.size(); ++i) {
        ranking.push_back(std::to_string(i + 1) + ". " + student_names[i] +
                          ": " + std::to_string(student_scores[i]));
    }
    return ranking;
}

// Create a string that contains the name of the first student to make a perfect
// score on the exam.
std::string perfect_score(std::vector<int> student_scores,
                          std::vector<std::string> student_names) {
    for (int i = 0; i < student_scores.size(); ++i) {
        if (student_scores[i] == 100) {
            return student_names[i];
        }
    }
    return "";
}
