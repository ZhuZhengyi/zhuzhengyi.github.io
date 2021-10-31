#
# @lc app=leetcode.cn id=543 lang=python3
#
# [543] 二叉树的直径
#
# https://leetcode-cn.com/problems/diameter-of-binary-tree/description/
#
# algorithms
# Easy (47.16%)
# Likes:    209
# Dislikes: 0
# Total Accepted:    21.3K
# Total Submissions: 44.9K
# Testcase Example:  '[1,2,3,4,5]'
#
# 给定一棵二叉树，你需要计算它的直径长度。一棵二叉树的直径长度是任意两个结点路径长度中的最大值。这条路径可能穿过根结点。
# 
# 示例 :
# 给定二叉树
# 
# 
# ⁠         1
# ⁠        / \
# ⁠       2   3
# ⁠      / \     
# ⁠     4   5    
# 
# 
# 返回 3, 它的长度是路径 [4,2,1,3] 或者 [5,2,1,3]。
# 
# 注意：两结点之间的路径长度是以它们之间边的数目表示。
# 
#

# @lc code=start
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def diameterOfBinaryTree(self, root: TreeNode) -> int:
        self.ans = 1 #节点数
        def depth(node: TreeNode) -> int:
            '''
            树深度：根节点到所有节点最长路径节点数。
            递归计算为所有子树最大深度+1
            '''
            if not node:
                return 0
            L = depth(node.left)    #左子树深度 
            R = depth(node.right)   #右子树深度
            # 直径为左右子树深度之和+1的最大值
            self.ans = max(self.ans, L + R + 1) 
            return max(L, R) + 1

        depth(root)

        #直径为节点数-1
        return self.ans - 1
        
# @lc code=end

