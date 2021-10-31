# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/binary-tree-paths
@Language: Python
@Datetime: 15-12-19 08:10
'''

"""
Definition of TreeNode:
class TreeNode:
    def __init__(self, val):
        this.val = val
        this.left, this.right = None, None
"""
class Solution:
    # @param {TreeNode} root the root of the binary tree
    # @return {List[str]} all root-to-leaf paths
    def binaryTreePaths(self, root):
        # Write your code here
        # null node
        if not root:
            return []
              
        # leave node    
        if not root.left and not root.right:
            return [ str(root.val) ]
            
        return [ '{}->{}'.format(root.val, p) \
                 for subtree in (root.left, root.right ) if subtree \
                  for p in self.binaryTreePaths(subtree) ]
        

        
