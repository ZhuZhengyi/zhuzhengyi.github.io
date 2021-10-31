# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/flatten-binary-tree-to-linked-list
@Language: Python
@Datetime: 17-02-05 13:13
'''

"""
Definition of TreeNode:
class TreeNode:
    def __init__(self, val):
        this.val = val
        this.left, this.right = None, None
"""
class Solution:
    # @param root: a TreeNode, the root of the binary tree
    # @return: nothing
    def flatten(self, root):
        # write your code here
        # m, l, r
        if root is None:
            return
        
        self.flatten(root.left)
        self.flatten(root.right)
        
        if root.left is not None:
            p = root.left
            while p.right is not None:
                p = p.right
            p.right = root.right
            
        if root.left is not None:
            root.right = root.left
            root.left = None
        