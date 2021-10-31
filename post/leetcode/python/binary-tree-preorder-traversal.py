# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/binary-tree-preorder-traversal
@Language: Python
@Datetime: 15-08-23 03:10
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
    @return: Preorder in ArrayList which contains node values.
    """
    def preorderTraversal(self, root):
        # write your code here
        res=[]
        if root == None:
            return res
        
        s=[]    
        s.append(root)
        while len(s) > 0:
            p=s.pop()
            res.append(p.val) 
            
            if(p.right!=None):
                s.append(p.right)
            if(p.left!=None):
                s.append(p.left)
            
        return res