# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/binary-tree-postorder-traversal
@Language: Python
@Datetime: 15-07-26 03:13
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
    @return: Postorder in ArrayList which contains node values.
    """
    def postorderTraversal(self, root):
        # write your code here
        s=[]
        res=[]
        
        if root==None:
            return res
            
        p=root
        last=None
        s.append(root)
        while len(s)>0:
            p=s[-1]
            if (p.left==None and p.right==None) \
            or (last!=None and (p.left==last or p.right==last )):
                res.append(p.val)
                s.pop()
                last=p
            else:
                if p.right!=None:
                    s.append(p.right)
                if p.left !=None:
                    s.append(p.left)
        return res
