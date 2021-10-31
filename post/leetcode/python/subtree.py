# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/subtree
@Language: Python
@Datetime: 15-08-02 07:34
'''

"""
Definition of TreeNode:
class TreeNode:
    def __init__(self, val):
        self.val = val
        self.left, self.right = None, None
"""
class Solution:
    # @param T1, T2: The roots of binary tree.
    # @return: True if T2 is a subtree of T1, or false.
    def isSubtree(self, T1, T2):
        # write your code here
        if T2==None:
            return True

        if T1==None and T2!=None:
            return False

        if self.dp(T1, T2):
            return True

        if self.isSubtree(T1.left, T2):
            return True

        if self.isSubtree(T1.right, T2):
            return True

        return False

    def dp(self, T1, T2):
        if T1==None and T2==None:
            return True
        if T1!=None and T2==None:
            return False
        if T1==None and T2!=None:
            return False
        else:
            if T1.val != T2.val:
                return False
            else:
                if self.dp(T1.left, T2.left) and self.dp(T1.right, T2.right):
                    return True

        return False

