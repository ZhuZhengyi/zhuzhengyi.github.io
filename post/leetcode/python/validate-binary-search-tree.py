# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/validate-binary-search-tree
@Language: Python
@Datetime: 15-08-19 13:42
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
    @return: True if the binary tree is BST, or false
    """  
    def isValidBST(self, root):
        # write your code here
        if root==None:
            return True
        
        nums=[]
        midTravelBST(root, nums)
        
        last=nums[0]
        for n in nums[1:]:
            if n<=last:
                return False
            last=n
                
        return True
        
def midTravelBST(root, nums):
    if root.left!=None:
        midTravelBST(root.left, nums)
    nums.append(root.val)
    if root.right!=None:
        midTravelBST(root.right, nums)

            
