/*
 * @lc app=leetcode.cn id=1219 lang=rust
 *
 * [1219] 黄金矿工
 *
 * https://leetcode-cn.com/problems/path-with-maximum-gold/description/
 *
 * algorithms
 * Medium (63.03%)
 * Likes:    97
 * Dislikes: 0
 * Total Accepted:    12.2K
 * Total Submissions: 19.4K
 * Testcase Example:  '[[0,6,0],[5,8,7],[0,9,0]]'
 *
 * 你要开发一座金矿，地质勘测学家已经探明了这座金矿中的资源分布，并用大小为 m * n 的网格 grid
 * 进行了标注。每个单元格中的整数就表示这一单元格中的黄金数量；如果该单元格是空的，那么就是 0。
 *
 * 为了使收益最大化，矿工需要按以下规则来开采黄金：
 *
 *
 * 每当矿工进入一个单元，就会收集该单元格中的所有黄金。
 * 矿工每次可以从当前位置向上下左右四个方向走。
 * 每个单元格只能被开采（进入）一次。
 * 不得开采（进入）黄金数目为 0 的单元格。
 * 矿工可以从网格中 任意一个 有黄金的单元格出发或者是停止。
 *
 *
 *
 *
 * 示例 1：
 *
 * 输入：grid = [[0,6,0],[5,8,7],[0,9,0]]
 * 输出：24
 * 解释：
 * [[0,6,0],
 * ⁠[5,8,7],
 * ⁠[0,9,0]]
 * 一种收集最多黄金的路线是：9 -> 8 -> 7。
 *
 *
 * 示例 2：
 *
 * 输入：grid = [[1,0,7],[2,0,6],[3,4,5],[0,3,0],[9,0,20]]
 * 输出：28
 * 解释：
 * [[1,0,7],
 * ⁠[2,0,6],
 * ⁠[3,4,5],
 * ⁠[0,3,0],
 * ⁠[9,0,20]]
 * 一种收集最多黄金的路线是：1 -> 2 -> 3 -> 4 -> 5 -> 6 -> 7。
 *
 *
 *
 *
 * 提示：
 *
 *
 * 1 <= grid.length, grid[i].length <= 15
 * 0 <= grid[i][j] <= 100
 * 最多 25 个单元格中有黄金。
 *
 *
 */

use super::*;

// @lc code=start
use std::cmp;

impl Solution {
    /// ## 解题思路
    /// 深度遍历
    pub fn get_maximum_gold(grid: Vec<Vec<i32>>) -> i32 {
        if grid.len() == 0 {
            return 0;
        }

        fn dig_golds_from(grid: &mut Vec<Vec<i32>>, i: usize, j: usize) -> i32 {
            let (m, n) = (grid.len(), grid[0].len());
            // 如果到达边界或遇到无法开采的单元格，则退出
            if i > m - 1 || j > n - 1 || grid[i][j] == 0 {
                return 0;
            }

            let digged_gold = grid[i][j]; //
            grid[i][j] = 0; //当前格黄金被挖了
            let left_golds = dig_golds_from(grid, i - 1, j); //向左挖
            let right_golds = dig_golds_from(grid, i + 1, j); //向右挖
            let up_golds = dig_golds_from(grid, i, j - 1); //向上移动一格，接着挖
            let down_golds = dig_golds_from(grid, i, j + 1); //向下移动一格，接着挖
            let other_golds = vec![left_golds, right_golds, up_golds, down_golds]
                .iter()
                .max()
                .unwrap()
                .clone();
            grid[i][j] = digged_gold;
            digged_gold + other_golds
        }

        let (m, n) = (grid.len(), grid[0].len());
        let mut max_golds = 0;

        let mut grid2 = grid.to_vec();
        for i in 0..m {
            for j in 0..n {
                max_golds = cmp::max(max_golds, dig_golds_from(&mut grid2, i, j))
            }
        }

        max_golds
    }
}
// @lc code=end
