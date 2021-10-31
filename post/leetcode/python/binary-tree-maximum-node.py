# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/binary-tree-maximum-node
@Language: Python
@Datetime: 17-03-10 14:15
'''

class Solution:
    # @param {TreeNode} root the root of binary tree
    # @return {TreeNode} the max node
    def maxNode(self, root):
        # Write your code here
        if root is None:
            return None
        max_node = root
        if root.left is not None:
            max_l = self.maxNode(root.left)
            if max_node.val < max_l.val:
                max_node = max_l
        if root.right is not None:
            max_l = self.maxNode(root.right)
            if max_node.val < max_l.val:
                max_node = max_l
        return max_node
            
            