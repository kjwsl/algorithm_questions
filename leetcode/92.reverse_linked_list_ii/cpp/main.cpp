#include "test.hpp"
#include <algorithm>
#include <initializer_list>
#include <variant>

struct ListNode {
    int val;
    ListNode* next;
    ListNode()
        : val(0)
        , next(nullptr)
    {
    }
    ListNode(int x)
        : val(x)
        , next(nullptr)
    {
    }
    ListNode(int x, ListNode* next)
        : val(x)
        , next(next)
    {
    }
    ListNode(std::initializer_list<int> vals)
    {
        ListNode* current { this };
        for (const auto& val : vals) {
            current = current->push(val);
        }
    }

    friend bool operator==(ListNode& a, ListNode& b)
    {
        ListNode* current_a { &a };
        ListNode* current_b { &b };

        while (current_a != nullptr && current_b != nullptr) {
            if (current_a->val != current_b->val) {
                return false;
            }

            current_a = current_a->next;
            current_b = current_b->next;
        }

        return current_a == nullptr && current_b == nullptr;
    }

private:
    ListNode* push(int val)
    {
        ListNode* new_node { new ListNode() };
        new_node->val  = val;
        new_node->next = nullptr;
        this->next     = new_node;

        return this->next;
    }
};

class Solution {
public:
    ListNode* reverseBetween(ListNode* head, int left, int right)
    {
        ListNode* current { head };

        for (int i { 1 }; i < left; i++) {
            current = current->next;
        }

        ListNode* left_node { current };

        std::vector<int> values {};

        for (int i { left }; i <= right; i++) {
            values.push_back(current->val);
            current = current->next;
        }

        std::reverse(values.begin(), values.end());

        current = left_node;

        for (const auto& val : values) {
            current->val = val;
            current      = current->next;
        }

        return head;
    }
};

int main(int argc, char* argv[])
{
    Test test {};
    ListNode* head { new ListNode({ 1, 2, 3, 4, 5 }) };
    Solution sol {};
    test.validate(sol.reverseBetween(head, 2, 4), new ListNode({ 1, 4, 3, 2, 5 }));

    return 0;
}
