#include <vector>
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
};

class Solution {
public:
    ListNode* reverseKGroup(ListNode* head, int k)
    {
        std::vector<int> stack {};
        ListNode* dummy { head };
        ListNode* revHead { head };

        while (dummy != nullptr) {
            stack.push_back(dummy->val);
            dummy = dummy->next;
            if (stack.size() == k) {
                while (stack.size() > 0) {
                    revHead->val = stack.back();
                    stack.pop_back();
                    revHead = revHead->next;
                }
            }
        }

        return head;
    }
};
