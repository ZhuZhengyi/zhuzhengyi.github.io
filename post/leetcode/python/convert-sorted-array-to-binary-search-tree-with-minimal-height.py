# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/convert-sorted-array-to-binary-search-tree-with-minimal-height
@Language: Python
@Datetime: 15-08-02 13:42
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
    @param A: a list of integer
    @return: a tree node
    """
    def sortedArrayToBST(self, A):
        # write your code here
        n=len(A)
        if n<1:
            return None
        if n==1:
            return TreeNode(A[0])
            
        m=n/2
        
        root=TreeNode(A[m])
        if m>0:
            root.left=self.sortedArrayToBST(A[:m])
        if m<n-1:
            root.right=self.sortedArrayToBST(A[m+1:])
        
        return root
        
        
        

