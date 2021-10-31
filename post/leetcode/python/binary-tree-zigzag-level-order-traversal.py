# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/binary-tree-zigzag-level-order-traversal
@Language: Python
@Datetime: 15-07-26 05:21
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
    @return: A list of list of integer include 
             the zig zag level order traversal of its nodes' values
    """
    def zigzagLevelOrder(self, root):
        # write your code here
        # write your code here
        q1,q2=[],[]
        res1,res2=[],[]
        
        l2r=True
        
        if root==None:
            return res1
            
        q1.append(root)
        while len(q1)>0 :
            while len(q1)>0:
                p=q1.pop(0)
                if l2r==True:
                    res2.append(p.val)
                else:
                    res2.insert(0,p.val)
                if p.left!=None:
                    q2.append(p.left)
                if p.right != None:
                    q2.append(p.right)
                
            res1.append(res2)
            res2=[]
            l2r = not l2r
            
            if len(q2)>0:
                q1=q2
                q2=[]
                
        return res1