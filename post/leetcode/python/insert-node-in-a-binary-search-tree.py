# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/insert-node-in-a-binary-search-tree
@Language: Python
@Datetime: 15-07-26 04:56
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
    @param root: The root of the binary search tree.
    @param node: insert this node into the binary search tree.
    @return: The root of the new binary search tree.
    """
    def insertNode(self, root, node):
        # write your code here
        r=root
        if root==None:
            r=node
        
        p=root
        while p!=None:
            if node.val<p.val:
                if p.left==None:
                    p.left=node
                    break
                p=p.left
            elif node.val>p.val:
                if p.right==None:
                    p.right=node
                    break
                p=p.right
            
        return r 
