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

        ListNode* dummy { head };
        std::queue<ListNode*> my_q {};
        unsigned int size { 0 };
        while (dummy != nullptr) {
            my_q.push(dummy);
            dummy = dummy->next;
            if (my_q.size() > n + 1) {
                my_q.pop();
            }
            size++;
        }

        if (size == 1) {
            return nullptr;
        }

        if (n == 1) {
            while (my_q.size() > 2) {
                my_q.pop();
            }
            ListNode* prev { my_q.front() };
            my_q.pop();

            ListNode* target { my_q.front() };
            prev->next = nullptr;
        } else {
            if (size == n) {
                ListNode* tmp { head };
                head = head->next;
                delete tmp;
                return head;
            }
            ListNode* prev { my_q.front() };
            my_q.pop();
            ListNode* target { my_q.front() };
            my_q.pop();
            ListNode* next { my_q.front() };
            my_q.pop();

            delete target;
            target     = nullptr;
            prev->next = next;
        }

        return head;
    }
};
