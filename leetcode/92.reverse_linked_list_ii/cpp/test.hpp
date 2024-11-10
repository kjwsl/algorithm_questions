#ifndef TEST_HPP_
#define TEST_HPP_

#include <iostream>

class Test {
public:
    template <typename T>
    bool validate(const T& actual, const T& expected) const noexcept
    {
        mTestCount++;
        bool success { actual == expected };
        std::cout << "Test #" << mTestCount << ": " << (success ? "Passed" : "Failed") << std::endl;
        if (!success) {
            std::cout << "Expected: " << expected << std::endl;
            std::cout << "Actual: " << actual << std::endl;
        }
        return success;
    }

private:
    mutable int mTestCount { 0 };
};

#endif // TEST_HPP_
