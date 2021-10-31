# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/minimum-depth-of-binary-tree
@Language: Python
@Datetime: 15-08-23 03:24
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
    @return: An integer
    """ 
    def minDepth(self, root):
        # write your code here
        if root==None:
            return 0
        
        if root.left!=None and root.right!=None:
            return min(self.minDepth(root.left),self.minDepth(root.right))+1
        elif root.left==None:
            return self.minDepth(root.right)+1
        elif root.right==None:
            return self.minDepth(root.left)+1
        
        return 1