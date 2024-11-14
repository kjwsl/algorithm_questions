#include <deque>
#include <queue>
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
    ListNode* removeNthFromEnd(ListNode* head, int n)
    {
        ListNode* dummy = new ListNode(0);
        dummy->next     = head;
        std::queue<ListNode*> node_trace {};
        while (dummy) {
            node_trace.push(dummy);
            if (node_trace.size() > n + 1) {
                node_trace.pop();
            }
            dummy = dummy->next;
        }
        ListNode* prev_target = node_trace.front();
        node_trace.pop();

        ListNode* target = node_trace.front();
        node_trace.pop();

        ListNode* next_target = node_trace.front();

        prev_target->next     = next_target;

        return head;
    }
};
