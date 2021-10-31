# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/construct-binary-tree-from-inorder-and-postorder-traversal
@Language: Python
@Datetime: 15-07-26 06:00
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
    @param inorder : A list of integers that inorder traversal of a tree
    @param postorder : A list of integers that postorder traversal of a tree
    @return : Root of a tree
    """
    def buildTree(self, inorder, postorder):
        # write your code here
        n=len(inorder)
        if n<1:
            return None
            
        if n==1:
            return TreeNode(inorder[0])
            
        r=postorder[-1]
        root=TreeNode(r)
        i=inorder.index(r)
        
        left,right=None,None
        if i>0:
            in_l=inorder[0:i] 
            post_l=postorder[0:i]
            left=self.buildTree(in_l,post_l)
            
        if i<n-1:
            in_r=inorder[i+1:]
            post_r=postorder[i:-1]
            right=self.buildTree(in_r,post_r)
        
        root.left=left
        root.right=right
        
        return root
        