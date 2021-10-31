# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/identical-binary-tree
@Language: Python
@Datetime: 17-01-26 07:28
'''

"""
Definition of TreeNode:
class TreeNode:
    def __init__(self, val):
        this.val = val
        this.left, this.right = None, None
"""
class Solution:
    """
    @param a, b, the root of binary trees.
    @return true if they are identical, or false.
    """
    def isIdentical(self, a, b):
        # Write your code here
        if a is None and b is None:
            return True
        elif a is not None and b is not None:
            return (a.val == b.val) and self.isIdentical(a.left, b.left) and self.isIdentical(a.right, b.right)
        else:
            return False