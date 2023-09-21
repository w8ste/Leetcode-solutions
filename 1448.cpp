//1448. Count Good Nodes in Binary Tree
#include <bits/stdc++.h>

//this struct was provided
struct TreeNode {
      int val;
      TreeNode *left;
      TreeNode *right;
      TreeNode() : val(0), left(nullptr), right(nullptr) {}
      TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
      TreeNode(int x, TreeNode *left, TreeNode *right) : val(x), left(left), right(right) {}
  };


class Solution {
public:

    int traverse(TreeNode* root, int max) {
        if(!root) return 0;
        int amount = 0;
        if(root->val >= max) {
            amount++;
            max = root->val;
        }

        return amount + traverse(root->left, max) + traverse(root->right, max);
    }

    int goodNodes(TreeNode* root) {
        int max = INT_MIN;
        return traverse(root, max);
    }
};
