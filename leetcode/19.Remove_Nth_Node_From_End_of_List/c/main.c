#include <stdlib.h>
#include <string.h>
struct ListNode {
    int val;
    struct ListNode* next;
};
struct ListNode* removeNthFromEnd(struct ListNode* head, int n)
{
    struct ListNode** node_trace = (struct ListNode**)malloc(sizeof(struct ListNode*) * n);
    memset(node_trace, 0, sizeof(struct ListNode*) * n);
    int trace_size = 0;

    while (head != NULL) {
        if (trace_size  > n + 1) {
            for (int i = 0; i < n - 1; i++) {
                node_trace[i] = node_trace[i + 1];
            }
            node_trace[n - 1] = head;
        } 
        else {
            node_trace[trace_size] = head;
            trace_size++;
        }
        
        struct ListNode* prev_target = node_trace[0];
        struct ListNode* target = node_trace[1];
        struct ListNode* next_target = node_trace[2];

        prev_target->next = next_target;
        free(target);

        head = head->next;
    }

    return head;
};
