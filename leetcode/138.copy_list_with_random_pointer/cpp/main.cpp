
// Definition for a Node.
#include <cstddef>
#include <unordered_map>
class Node {
public:
    int val;
    Node* next;
    Node* random;

    Node(int _val)
    {
        val    = _val;
        next   = NULL;
        random = NULL;
    }
};

class Solution {
public:
    Node* copyRandomList(Node* head)
    {
        if (head == NULL) {
            return NULL;
        }

        if (memo.find(head) != memo.end()) {
            return memo[head];
        }

        Node* new_node   = new Node(head->val);
        memo[head]       = new_node;
        new_node->next   = copyRandomList(head->next);
        new_node->random = copyRandomList(head->random);

        return new_node;
    }

private:
    std::unordered_map<Node*, Node*> memo;
};
