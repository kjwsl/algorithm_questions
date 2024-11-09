#include <stdio.h>
#include <stdlib.h>
#include <string.h>

struct ListNode {
    int val;
    struct ListNode* next;
};

struct ListNode* addTwoNumbers(struct ListNode* l1, struct ListNode* l2)
{
    // Create a dummy head to simplify the list construction
    struct ListNode* dummyHead = (struct ListNode*)malloc(sizeof(struct ListNode));
    if (dummyHead == NULL) {
        // Handle memory allocation failure
        return NULL;
    }
    dummyHead->val           = 0;
    dummyHead->next          = NULL;

    struct ListNode* current = dummyHead;
    int carry                = 0;

    // Loop until both lists are exhausted and there is no carry-over
    while (l1 != NULL || l2 != NULL || carry != 0) {
        int sum = carry;

        // Add value from l1 if available
        if (l1 != NULL) {
            sum += l1->val;
            l1 = l1->next;
        }

        // Add value from l2 if available
        if (l2 != NULL) {
            sum += l2->val;
            l2 = l2->next;
        }

        // Update carry and the digit to store
        carry     = sum / 10;
        int digit = sum % 10;

        // Create a new node for the digit
        struct ListNode* newNode = (struct ListNode*)malloc(sizeof(struct ListNode));
        if (newNode == NULL) {
            // Handle memory allocation failure
            // Ideally, free all previously allocated nodes before returning
            // For simplicity, we return NULL here
            // In production code, you should free all allocated nodes to prevent memory leaks
            return NULL;
        }
        newNode->val  = digit;
        newNode->next = NULL;

        // Link the new node to the current node
        current->next = newNode;
        current       = current->next;
    }

    // The first node is a dummy node, so the actual result starts from dummyHead->next
    struct ListNode* result = dummyHead->next;
    free(dummyHead); // Free the dummy node to prevent memory leaks
    return result;
}

void ListNode_free(struct ListNode* l)
{
    struct ListNode* tmp = l;
    while (l != NULL) {
        tmp = l;
        l   = l->next;
        free(tmp);
    }
}

void ListNode_push(struct ListNode* l, int val)
{
    struct ListNode* new_node = (struct ListNode*)malloc(sizeof(struct ListNode));
    new_node->val             = val;
    new_node->next            = l->next;
    l->next                   = new_node;
}

struct ListNode* ListNode_pop(struct ListNode* l)
{
    struct ListNode* tmp = l->next;
    l->next              = l->next->next;
    return tmp;
}

struct ListNode* ListNode_new(int val)
{
    struct ListNode* l = (struct ListNode*)malloc(sizeof(struct ListNode));
    memset(l, 0, sizeof(struct ListNode));
    l->val = val;
    return l;
}

void ListNode_print(struct ListNode* l)
{
    struct ListNode* tmp = l;
    while (tmp != NULL) {
        printf("%d -> ", tmp->val);
        tmp = tmp->next;
    }
    printf("NULL\n");
}

int main(int argc, char* argv[])
{
    struct ListNode* l1 = ListNode_new(2);
    ListNode_push(l1, 4);
    ListNode_push(l1, 3);
    ListNode_print(l1);

    struct ListNode* l2 = ListNode_new(5);
    ListNode_push(l2, 6);
    ListNode_push(l2, 4);
    ListNode_print(l2);

    struct ListNode* l3 = addTwoNumbers(l1, l2);
    ListNode_print(l3);

    ListNode_free(l1);
    ListNode_free(l2);
    ListNode_free(l3);

    return 0;
}
