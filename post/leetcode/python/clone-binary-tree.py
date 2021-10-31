# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/clone-binary-tree
@Language: Python
@Datetime: 17-02-05 08:18
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
    @param {TreeNode} root: The root of binary tree
    @return {TreeNode} root of new tree
    """
    def cloneTree(self, root):
        # Write your code here
        cRoot = None
        if root is not None:
            cRoot = TreeNode(root.val)
            cRoot.left = self.cloneTree(root.left)
            cRoot.right = self.cloneTree(root.right)
        else:
            return None
            
        return cRoot