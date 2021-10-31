# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/binary-tree-inorder-traversal
@Language: Python
@Datetime: 15-08-23 03:07
'''

"""
Definition of TreeNode:
class TreeNode:
    def __init__(self, val):
        self.val = val
        self.left, self.right = None, None
"""


class Solution:
    """
    @param root: The root of binary tree.
    @return: Inorder in ArrayList which contains node values.
    """
    def inorderTraversal(self, root):
        # write your code here
        res = []
        s = []
        if root is None:
            return res
        p = root
        while len(s) > 0 or p is not None:
            while p is not None:
                s.append(p)
                p = p.left
            if len(s) > 0:
                p = s.pop()
                res.append(p.val)
                p = p.right
        return res
