#include <cstdio>
#include <cstdlib>
#include <string>
#include <unordered_map> // Use unordered_map for better performance
#include <vector>
using namespace std;

// Definition for a binary tree node.
struct TreeNode {
    int val;
    TreeNode* left;
    TreeNode* right;
    TreeNode()
        : val(0)
        , left(nullptr)
        , right(nullptr)
    {
    }
    TreeNode(int x)
        : val(x)
        , left(nullptr)
        , right(nullptr)
    {
    }
    TreeNode(int x, TreeNode* left, TreeNode* right)
        : val(x)
        , left(left)
        , right(right)
    {
    }
};

class Solution {
public:
    // Recursive function to build the tree
    TreeNode* buildTreeRec(vector<int>& preorder, int& preorder_idx, unordered_map<int, int>& inorder_map, int inorder_start, int inorder_end)
    {
        // Base case: if there are no elements to construct the subtree
        if (inorder_start > inorder_end) {
            return nullptr;
        }

        // Get the root value from the current preorder index
        int root_val   = preorder[preorder_idx];
        TreeNode* root = new TreeNode(root_val);

        // Increment preorder index for the next call
        preorder_idx++;

        // If there is only one element, no need to recurse further
        if (inorder_start == inorder_end) {
            return root;
        }

        // Find the index of the root in inorder traversal
        int inorder_root_idx = inorder_map[root_val];

        // Build left subtree
        root->left = buildTreeRec(preorder, preorder_idx, inorder_map, inorder_start, inorder_root_idx - 1);

        // Build right subtree
        root->right = buildTreeRec(preorder, preorder_idx, inorder_map, inorder_root_idx + 1, inorder_end);

        return root;
    }

    TreeNode* buildTree(vector<int>& preorder, vector<int>& inorder)
    {
        // Check if preorder and inorder are valid
        if (preorder.empty() || inorder.empty() || preorder.size() != inorder.size()) {
            return nullptr;
        }

        // Create a map to store value -> index relations for inorder traversal
        unordered_map<int, int> inorder_map;
        for (int i = 0; i < inorder.size(); i++) {
            inorder_map[inorder[i]] = i;
        }

        int preorder_idx = 0; // Start from the first element in preorder
        return buildTreeRec(preorder, preorder_idx, inorder_map, 0, inorder.size() - 1);
    }
};

// Function to generate a string representation of the tree for printing
string getPrintTreeStr(TreeNode* root, string prefix, bool isLeft)
{
    if (!root) {
        return "";
    }
    string str = "";
    str += prefix;
    str += (isLeft) ? "├──" : "└──";
    str += to_string(root->val);
    str += "\n";
    string next_prefix = prefix;
    next_prefix += (isLeft) ? "│   " : "    ";
    if (root->left) {
        str += getPrintTreeStr(root->left, next_prefix, true);
    }
    if (root->right) {
        str += getPrintTreeStr(root->right, next_prefix, false);
    }
    return str;
}

// Function to print the tree
void printTree(TreeNode* root)
{
    printf("%s", getPrintTreeStr(root, "", false).c_str());
}

int main(int argc, char* argv[])
{
    Solution s {};

    // Test Case 1
    vector<int> preorder1 { 3, 9, 20, 15, 7 };
    vector<int> inorder1 { 9, 3, 15, 20, 7 };
    TreeNode* root1 = s.buildTree(preorder1, inorder1);
    printf("Test Case 1:\n");
    printTree(root1);
    printf("\n");

    // Test Case 2
    vector<int> preorder2 { 1, 2 };
    vector<int> inorder2 { 1, 2 };
    TreeNode* root2 = s.buildTree(preorder2, inorder2);
    printf("Test Case 2:\n");
    printTree(root2);
    printf("\n");

    // Test Case 3 (Invalid: Duplicate values in inorder)
    // Uncommenting the following lines will cause incorrect behavior due to duplicate '1's
    /*
    vector<int> preorder3 { 1, 2, 3 };
    vector<int> inorder3  { 3, 1, 1 };
    TreeNode* root3 = s.buildTree(preorder3, inorder3);
    printf("Test Case 3:\n");
    printTree(root3);
    printf("\n");
    */

    // Test Case 4
    vector<int> preorder4 { 3, 2, 1, 4 };
    vector<int> inorder4 { 1, 2, 3, 4 };
    TreeNode* root4 = s.buildTree(preorder4, inorder4);
    printf("Test Case 4:\n");
    printTree(root4);
    printf("\n");

    return 0;
}
