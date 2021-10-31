# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/binary-tree-path-sum
@Language: Python
@Datetime: 17-03-12 08:32
'''

"""
Definition of TreeNode:
class TreeNode:
    def __init__(self, val):
        self.val = val
        self.left, self.right = None, None
"""
class Solution:
    # @param {TreeNode} root the root of binary tree
    # @param {int} target an integer
    # @return {int[][]} all valid paths
    def binaryTreePathSum(self, root, target):
        # Write your code here
        path = []
        result = []
        self.search(root, target, path, result)
        return result
        
    def search(self, root, target, path, result):
        if root is None:
            return
        path.append(root.val)
        if root.left is None and root.right is None and root.val == target:
            result.append(path[:])
        if root.left is not None:
            self.search(root.left, target-root.val, path, result)
        if root.right is not None:
            self.search(root.right, target-root.val, path, result)
        path.pop()