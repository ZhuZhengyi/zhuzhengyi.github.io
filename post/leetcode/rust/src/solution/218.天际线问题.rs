/*
 * @lc app=leetcode.cn id=218 lang=rust
 *
 * [218] 天际线问题
 *
 * https://leetcode-cn.com/problems/the-skyline-problem/description/
 *
 * algorithms
 * Hard (54.24%)
 * Likes:    591
 * Dislikes: 0
 * Total Accepted:    33.7K
 * Total Submissions: 62.3K
 * Testcase Example:  '[[2,9,10],[3,7,15],[5,12,12],[15,20,10],[19,24,8]]'
 *
 * 城市的天际线是从远处观看该城市中所有建筑物形成的轮廓的外部轮廓。给你所有建筑物的位置和高度，请返回由这些建筑物形成的 天际线 。
 *
 * 每个建筑物的几何信息由数组 buildings 表示，其中三元组 buildings[i] = [lefti, righti, heighti]
 * 表示：
 *
 *
 * lefti 是第 i 座建筑物左边缘的 x 坐标。
 * righti 是第 i 座建筑物右边缘的 x 坐标。
 * heighti 是第 i 座建筑物的高度。
 *
 *
 * 天际线 应该表示为由 “关键点” 组成的列表，格式 [[x1,y1],[x2,y2],...] ，并按 x 坐标 进行 排序
 * 。关键点是水平线段的左端点。列表中最后一个点是最右侧建筑物的终点，y 坐标始终为 0
 * ，仅用于标记天际线的终点。此外，任何两个相邻建筑物之间的地面都应被视为天际线轮廓的一部分。
 *
 * 注意：输出天际线中不得有连续的相同高度的水平线。例如 [...[2 3], [4 5], [7 5], [11 5], [12 7]...]
 * 是不正确的答案；三条高度为 5 的线应该在最终输出中合并为一个：[...[2 3], [4 5], [12 7], ...]
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：buildings = [[2,9,10],[3,7,15],[5,12,12],[15,20,10],[19,24,8]]
 * 输出：[[2,10],[3,15],[7,12],[12,0],[15,10],[20,8],[24,0]]
 * 解释：
 * 图 A 显示输入的所有建筑物的位置和高度，
 * 图 B 显示由这些建筑物形成的天际线。图 B 中的红点表示输出列表中的关键点。
 *
 * 示例 2：
 *
 *
 * 输入：buildings = [[0,2,3],[2,5,3]]
 * 输出：[[0,3],[5,0]]
 *
 *
 *
 *
 * 提示：
 *
 *
 * 1
 * 0 i < righti
 * 1 i
 * buildings 按 lefti 非递减排序
 *
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    /// ## 解题思路
    /// - 分治+归并
    /// 1.
    pub fn get_skyline(buildings: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        // 获取指定范围[l, r)内的天际线
        fn get_skyline_by_range(buildings: &[Vec<i32>], l: usize, r: usize) -> Vec<Vec<i32>> {
            if l >= r {
                return vec![];
            } else if l + 1 == r {
                return vec![
                    vec![buildings[l][0], buildings[l][2]],
                    vec![buildings[l][1], 0],
                ];
            }

            // r - l > 1
            let mut res: Vec<Vec<i32>> = vec![];
            let mid = (l + r) >> 1;
            let left_res = get_skyline_by_range(buildings, l, mid);
            let right_res = get_skyline_by_range(buildings, mid, r);
            let (mut x, mut yl, mut yr) = (0, 0, 0);
            let (mut i, mut j) = (0, 0);
            // merge left and right skylines
            while i < left_res.len() || j < right_res.len() {
                // 如果
                if (i < left_res.len() && left_res[i][0] < right_res[j][0]) || j == right_res.len()
                {
                    x = left_res[i][0];
                    yl = left_res[i][1];
                    i += 1;
                } else {
                    x = right_res[j][0];
                    yr = right_res[j][1];
                    j += 1;
                }
                let y = yl.max(yr);
                let len = res.len();
                // 如果y坐标和前一个相同, 则跳过
                if len > 0 && res[len - 1][1] == y {
                    continue;
                }
                // 如果x坐标和前一个相同, 则更新上一个y坐标;
                if len > 0 && res[len - 1][0] == x {
                    res[len - 1][1] = y;
                } else {
                    res.push(vec![x, y]);
                }
            }

            res
        }

        get_skyline_by_range(&buildings, 0, buildings.len())
    }
}
// @lc code=end
