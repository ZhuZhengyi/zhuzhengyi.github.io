#
# @lc app=leetcode.cn id=94 lang=python3
#
# [94] 二叉树的中序遍历
#
# https://leetcode-cn.com/problems/binary-tree-inorder-traversal/description/
#
# algorithms
# Medium (68.96%)
# Likes:    339
# Dislikes: 0
# Total Accepted:    83.1K
# Total Submissions: 120.1K
# Testcase Example:  '[1,null,2,3]'
#
# 给定一个二叉树，返回它的中序 遍历。
# 
# 示例:
# 
# 输入: [1,null,2,3]
# ⁠  1
# ⁠   \
# ⁠    2
# ⁠   /
# ⁠  3
# 
# 输出: [1,3,2]
# 
# 进阶: 递归算法很简单，你可以通过迭代算法完成吗？
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
    def inorderTraversal(self, root: TreeNode) -> List[int]:
        res = []
        if not root:
            return res

        pending = [ ] #
        cur = root
        while len(pending) > 0:
            cur = pending.pop()

            while cur: #pending current node if left exists
                pending.append(cur)
                cur = cur.left
            if not pending:
                break
                #return res
            cur = pending.pop()
            res.append(cur.val)
            cur = cur.right
        return res

    def inorderTraversal3(self, root: TreeNode) -> List[int]:
        res = []
        if not root:
            return res

        pending = [] #
        n = root
        while True:
            ## 先遍历所有左节点，遍历时将节点先入栈中
            while n:
                pending.append(n)  #将当前节点先入栈，待左子树处理完后再处理
                n = n.left
            ## 
            if not pending:
                break
            ## 依次将栈中节点弹出
            n = pending.pop()

            ## 记录结果
            res.append(n.val)

            ## 转到右子树，继续上面方式同样处理
            n = n.right
        return res

    def inorderTraversal2(self, root: TreeNode) -> List[int]:
        def inorderTraversalRec(root, res):
            if not root:
                return
            inorderTraversalRec(root.left, res)
            res.append(root.val)
            inorderTraversalRec(root.right, res)

        res = []
        inorderTraversalRec(root, res)
        return res
        
# @lc code=end

