# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/construct-binary-tree-from-preorder-and-inorder-traversal
@Language: Python
@Datetime: 15-07-26 06:06
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
    @param preorder : A list of integers that preorder traversal of a tree
    @param inorder : A list of integers that inorder traversal of a tree
    @return : Root of a tree
    """
    def buildTree(self, preorder, inorder):
        # write your code here
        n=len(inorder)
        if n<1:
            return None
            
        if n==1:
            return TreeNode(inorder[0])
            
        r=preorder[0]
        root=TreeNode(r)
        i=inorder.index(r)
        
        left,right=None,None
        if i>0:
            in_l=inorder[0:i] 
            pre_l=preorder[1:i+1]
            left=self.buildTree(pre_l,in_l)
            
        if i<n-1:
            in_r=inorder[i+1:]
            pre_r=preorder[i+1:]
            right=self.buildTree(pre_r,in_r)
        
        root.left=left
        root.right=right
        
        return root
        