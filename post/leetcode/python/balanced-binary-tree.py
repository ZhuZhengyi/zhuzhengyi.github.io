# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/balanced-binary-tree
@Language: Python
@Datetime: 15-08-17 13:45
'''

"""
Definition of TreeNode:
class TreeNode:
    def __init__(self, val):
        self.val = val
        self.left, self.right = None, None
"""
def getTreeDepth(root):
    if root==None:
        return 0
    
    return max(getTreeDepth(root.left), getTreeDepth(root.right))+1

class Solution:
    """
    @param root: The root of binary tree.
    @return: True if this Binary tree is Balanced, or false.
    """
    def isBalanced(self, root):
        # write your code here
        if root==None:
            return True
            
        if root.left==None and root.right==None:
            return True
            
        ldepth=getTreeDepth(root.left)
        rdepth=getTreeDepth(root.right)
        if abs(ldepth-rdepth)>1:
            return False
        elif self.isBalanced(root.left) and self.isBalanced(root.right):
            return True
            
        return False
        
        

