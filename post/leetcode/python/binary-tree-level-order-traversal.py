# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/binary-tree-level-order-traversal
@Language: Python
@Datetime: 15-07-26 04:46
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
    @return: Level order in a list of lists of integers
    """
    def levelOrder(self, root):
        # write your code here
        q1=[]
        q2=[]
        res1=[]
        res2=[]
        
        if root==None:
            return res1
            
        p=root
        q1.append(p)
        while len(q1)>0 or len(q2)>0:
            #
            while len(q1)>0:
                p=q1.pop(0)
                res2.append(p.val)
                if p.left!=None:
                    q2.append(p.left)
                if p.right !=None:
                    q2.append(p.right)
                    
            res1.append(res2)
            res2=[]
            if len(q2)>0:
                q1=q2
                q2=[]
                
        return res1
                
