// Definition for singly-linked list.
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
    ListNode* rotateRight(ListNode* head, int k)
    {
        ListNode* dummy { head };
        ListNode* newHead {};
        int i { 0 };
        while (!dummy) {
            if (!dummy->next && i < k) {
                dummy->next = head;
            }
            if (i == k) {
                newHead = dummy->next;
                dummy   = newHead;
            } else {
                dummy = dummy->next;
            }
            i++;
        }

        return newHead;
    }
};
