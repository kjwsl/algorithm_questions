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
    ListNode* del_rec(ListNode* left, ListNode* right)
    {
        if (right == nullptr) {
            left->next = nullptr;
            return left;
        }
        if (left->val == right->val) {
            while (right != nullptr && right->val == left->val) {
                right = right->next;
            }
            if (right == nullptr)
                return nullptr;
            return del_rec(right, right->next);
        }

        left->next = del_rec(right, right->next);

        return left;
    }
    ListNode* deleteDuplicates(ListNode* head)
    {
        if (head == nullptr || head->next == nullptr) {
            return head;
        }
        return del_rec(head, head->next);
    }
};
