# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/binary-tree-level-order-traversal-ii
@Language: Python
@Datetime: 15-07-26 05:16
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
    @return: buttom-up level order in a list of lists of integers
    """
    def levelOrderBottom(self, root):
        # write your code here
        q1,q2=[],[]
        res1,res2=[],[]

        if root==None:
            return res1

        q1.append(root)
        while len(q1)>0 :
            while len(q1)>0:
                p=q1.pop(0)
                res2.append(p.val)
                if p.left!=None:
                    q2.append(p.left)
                if p.right != None:
                    q2.append(p.right)

            res1.insert(0,res2)
            res2=[]

            if len(q2)>0:
                q1=q2
                q2=[]

        return res1

