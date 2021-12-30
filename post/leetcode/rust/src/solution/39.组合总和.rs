/*
 * @lc app=leetcode.cn id=39 lang=rust
 *
 * [39] 组合总和
 *
 * https://leetcode-cn.com/problems/combination-sum/description/
 *
 * algorithms
 * Medium (71.68%)
 * Likes:    1070
 * Dislikes: 0
 * Total Accepted:    183.8K
 * Total Submissions: 256.4K
 * Testcase Example:  '[2,3,6,7]\n7'
 *
 * 给定一个无重复元素的数组 candidates 和一个目标数 target ，找出 candidates 中所有可以使数字和为 target 的组合。
 * 
 * candidates 中的数字可以无限制重复被选取。
 * 
 * 说明：
 * 
 * 
 * 所有数字（包括 target）都是正整数。
 * 解集不能包含重复的组合。 
 * 
 * 
 * 示例 1：
 * 
 * 输入：candidates = [2,3,6,7], target = 7,
 * 所求解集为：
 * [
 * ⁠ [7],
 * ⁠ [2,2,3]
 * ]
 * 
 * 
 * 示例 2：
 * 
 * 输入：candidates = [2,3,5], target = 8,
 * 所求解集为：
 * [
 * [2,2,2,2],
 * [2,3,3],
 * [3,5]
 * ]
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * 1 <= candidates.length <= 30
 * 1 <= candidates[i] <= 200
 * candidate 中的每个元素都是独一无二的。
 * 1 <= target <= 500
 * 
 * 
 */

// @lc code=start
impl Solution {
    /// ## 解题思路
    /// * 迭代法
    /// 1. 将数组排序；
    /// 2. 依次从candidates中取出一个数，如果该数小于当前target，则加入临时数组中；
    /// 3. 
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {

        /// 
        fn combine_sum(candidates: &[i32], left: i32, sub: &[i32], res: &mut Vec<Vec<i32>>) {
            match left {
                n if n < 0 => return,
                0 => {
                    res.push(sub.to_vec());
                    return
                }
                _ => {
                    candidates.iter()
                        .filter(|&c| *c <= left)
                        .enumerate()
                        .for_each(|(i, c)| {
                            let mut s = sub.to_vec();
                            s.push(*c);
                            combine_sum(&candidates[i..], left-c, &s, res);
                        });

                }
            }
        }

        let mut res: Vec<Vec<i32>> = vec![];
        let mut c = candidates.to_vec();
        c.sort();
        combine_sum(&c, target, &vec![], &mut res);
        res
    }
}
// @lc code=end

