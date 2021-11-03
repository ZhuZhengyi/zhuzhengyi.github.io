#
# @lc app=leetcode.cn id=1219 lang=python3
#
# [1219] 黄金矿工
#
# https://leetcode-cn.com/problems/path-with-maximum-gold/description/
#
# algorithms
# Medium (63.03%)
# Likes:    97
# Dislikes: 0
# Total Accepted:    12.2K
# Total Submissions: 19.4K
# Testcase Example:  '[[0,6,0],[5,8,7],[0,9,0]]'
#
# 你要开发一座金矿，地质勘测学家已经探明了这座金矿中的资源分布，并用大小为 m * n 的网格 grid
# 进行了标注。每个单元格中的整数就表示这一单元格中的黄金数量；如果该单元格是空的，那么就是 0。
# 
# 为了使收益最大化，矿工需要按以下规则来开采黄金：
# 
# 
# 每当矿工进入一个单元，就会收集该单元格中的所有黄金。
# 矿工每次可以从当前位置向上下左右四个方向走。
# 每个单元格只能被开采（进入）一次。
# 不得开采（进入）黄金数目为 0 的单元格。
# 矿工可以从网格中 任意一个 有黄金的单元格出发或者是停止。
# 
# 
# 
# 
# 示例 1：
# 
# 输入：grid = [[0,6,0],[5,8,7],[0,9,0]]
# 输出：24
# 解释：
# [[0,6,0],
# ⁠[5,8,7],
# ⁠[0,9,0]]
# 一种收集最多黄金的路线是：9 -> 8 -> 7。
# 
# 
# 示例 2：
# 
# 输入：grid = [[1,0,7],[2,0,6],[3,4,5],[0,3,0],[9,0,20]]
# 输出：28
# 解释：
# [[1,0,7],
# ⁠[2,0,6],
# ⁠[3,4,5],
# ⁠[0,3,0],
# ⁠[9,0,20]]
# 一种收集最多黄金的路线是：1 -> 2 -> 3 -> 4 -> 5 -> 6 -> 7。
# 
# 
# 
# 
# 提示：
# 
# 
# 1 <= grid.length, grid[i].length <= 15
# 0 <= grid[i][j] <= 100
# 最多 25 个单元格中有黄金。
# 
# 
#

# @lc code=start
class Solution:
    '''
    ## 解题思路
    ### 遍历表格，求每一个单元格开始的最大路径和
    '''
    def getMaximumGold(self, grid: List[List[int]]) -> int:
        if len(grid) == 0 or len(grid[0]) == 0: 
            return 0

        def _getMaxGoldsFrom(grid, i, j) -> int:
            '''
            从i,j开始的最大黄金路径
            '''
            # 退出条件：遍历到达边界，或遇到0
            if i < 0 or i >= m or j < 0 or j >= n or grid[i][j] == 0:
                return 0

            gold = grid[i][j]   #保存旧值，以便遍历回溯时恢复
            grid[i][j] = 0      #遍历过当前单元格，遍历后为0，以防止遍历时走重复路
            # 依次往4个方向递归遍历时的最大路径和
            max_gold = max(
                _getMaxGoldsFrom(grid, i-1, j), 
                _getMaxGoldsFrom(grid, i+1, j), 
                _getMaxGoldsFrom(grid, i, j-1), 
                _getMaxGoldsFrom(grid, i, j+1)
            )
            grid[i][j] = gold   #遍历返回，恢复当前单元格
            return grid[i][j] + max_gold   #当前节点路径和

        m, n = len(grid), len(grid[0])
        max_golds = 0
        for i in range(m):
            for j in range(n):
                max_golds = max(max_golds, _getMaxGoldsFrom(grid, i, j))

        return max_golds

# @lc code=end

