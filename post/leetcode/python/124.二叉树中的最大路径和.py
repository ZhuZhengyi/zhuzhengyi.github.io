#
# @lc app=leetcode.cn id=124 lang=python3
#
# [124] 二叉树中的最大路径和
#
# https://leetcode-cn.com/problems/binary-tree-maximum-path-sum/description/
#
# algorithms
# Hard (44.52%)
# Likes:    1264
# Dislikes: 0
# Total Accepted:    165.9K
# Total Submissions: 372.7K
# Testcase Example:  '[1,2,3]'
#
# 路径 被定义为一条从树中任意节点出发，沿父节点-子节点连接，达到任意节点的序列。同一个节点在一条路径序列中 至多出现一次 。该路径 至少包含一个
# 节点，且不一定经过根节点。
# 
# 路径和 是路径中各节点值的总和。
# 
# 给你一个二叉树的根节点 root ，返回其 最大路径和 。
# 
# 
# 
# 示例 1：
# 
# 
# 输入：root = [1,2,3]
# 输出：6
# 解释：最优路径是 2 -> 1 -> 3 ，路径和为 2 + 1 + 3 = 6
# 
# 示例 2：
# 
# 
# 输入：root = [-10,9,20,null,null,15,7]
# 输出：42
# 解释：最优路径是 15 -> 20 -> 7 ，路径和为 15 + 20 + 7 = 42
# 
# 
# 
# 
# 提示：
# 
# 
# 树中节点数目范围是 [1, 3 * 10^4]
# -1000 
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
    def __init__(self):
        self.maxSum = float("-inf")

    def maxPathSum(self, root: TreeNode) -> int:
        '''
        ## 解题思路
        对于任意节点n，经过该节点的路径和有三个：
        1. maxPath(n.left) + n.val + maxPath(n.right);
        2. maxPath(n.left) + n.val + maxPath(n.parent);
        3. maxPath(n.right) + n.val + maxPath(n.parent);
        '''

        def _maxPathSumEndWithNode(node: TreeNode) -> int:
            '''
            以node为端点的最大路径和
            * 退出条件：node为空
            * 递推公式：
                * 左子树最大路径和，如果>0，加入到总路径和中；
                * 右子树最大路径和，如果>0, 加入到总路径和中；
                * node节点总路径和为：root.val 
            '''
            if not node:
                return 0

            #以左子节点为端点的最大有效路径和
            leftValidEndSum = max(0, _maxPathSumEndWithNode(node.left))     #左子树最大路径和，>0时才有用
            #以右子节点为端点的最大有效路径和
            rightValidEndSum = max(0, _maxPathSumEndWithNode(node.right))     #右子树最大路径和，>0时才有用

            #经过node的路径和
            curThroughPathSum = node.val + leftValidEndSum + rightValidEndSum

            #最大路径和为: 所有经过各节点路径和中的最大值
            self.maxSum = max(curThroughPathSum, self.maxSum)

            #以当前node为端点的路径和
            return node.val + max(leftValidEndSum, rightValidEndSum)

        _maxPathSumEndWithNode(root)

        return self.maxSum
# @lc code=end

