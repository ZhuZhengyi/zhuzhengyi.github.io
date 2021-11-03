#
# @lc app=leetcode.cn id=1026 lang=python3
#
# [1026] 节点与其祖先之间的最大差值
#
# https://leetcode-cn.com/problems/maximum-difference-between-node-and-ancestor/description/
#
# algorithms
# Medium (67.17%)
# Likes:    99
# Dislikes: 0
# Total Accepted:    10.1K
# Total Submissions: 15.1K
# Testcase Example:  '[8,3,10,1,6,null,14,null,null,4,7,13]'
#
# 给定二叉树的根节点 root，找出存在于 不同 节点 A 和 B 之间的最大值 V，其中 V = |A.val - B.val|，且 A 是 B
# 的祖先。
# 
# （如果 A 的任何子节点之一为 B，或者 A 的任何子节点是 B 的祖先，那么我们认为 A 是 B 的祖先）
# 
# 
# 
# 示例 1：
# 
# 
# 
# 
# 输入：root = [8,3,10,1,6,null,14,null,null,4,7,13]
# 输出：7
# 解释： 
# 我们有大量的节点与其祖先的差值，其中一些如下：
# |8 - 3| = 5
# |3 - 7| = 4
# |8 - 1| = 7
# |10 - 13| = 3
# 在所有可能的差值中，最大值 7 由 |8 - 1| = 7 得出。
# 
# 
# 示例 2：
# 
# 
# 输入：root = [1,null,2,null,0,3]
# 输出：3
# 
# 
# 
# 
# 提示：
# 
# 
# 树中的节点数在 2 到 5000 之间。
# 0 
# 
# 
#

# @lc code=start
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    '''
    解题思路：
    * 遍历树各节点
    * 对于每一个节点，其与祖先最大差值就是该节点值与祖先最大值和最小值差值绝对值的

    '''
    def __init__(self):
        self.res = 0

    def maxAncestorDiff(self, root: TreeNode) -> int:

        def preOrder(node: TreeNode, ancestorUp, ancestorLow: int):
            '''
            先序遍历树节点，根据最大祖先节点和最小祖先节点，计算该节点与祖先节点最大差值
            '''
            if not node:
                return 
            
            self.res = max(self.res, abs(node.val-ancestorUp), abs(node.val-ancestorLow))
            ancestorUp = max(node.val, ancestorUp)
            ancestorLow = min(node.val, ancestorLow)
            preOrder(node.left, ancestorUp, ancestorLow)
            preOrder(node.right, ancestorUp, ancestorLow)

        preOrder(root, root.val, root.val)

        return self.res

# @lc code=end

