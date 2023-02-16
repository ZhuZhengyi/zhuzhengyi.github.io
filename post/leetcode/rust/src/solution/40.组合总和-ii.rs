/*
 * @lc app=leetcode.cn id=40 lang=rust
 *
 * [40] 组合总和 II
 *
 * https://leetcode-cn.com/problems/combination-sum-ii/description/
 *
 * algorithms
 * Medium (62.12%)
 * Likes:    737
 * Dislikes: 0
 * Total Accepted:    213.4K
 * Total Submissions: 344.8K
 * Testcase Example:  '[10,1,2,7,6,1,5]\n8'
 *
 * 给定一个数组 candidates 和一个目标数 target ，找出 candidates 中所有可以使数字和为 target 的组合。
 * 
 * candidates 中的每个数字在每个组合中只能使用一次。
 * 
 * 注意：解集不能包含重复的组合。 
 * 
 * 
 * 
 * 示例 1:
 * 
 * 
 * 输入: candidates = [10,1,2,7,6,1,5], target = 8,
 * 输出:
 * [
 * [1,1,6],
 * [1,2,5],
 * [1,7],
 * [2,6]
 * ]
 * 
 * 示例 2:
 * 
 * 
 * 输入: candidates = [2,5,2,1,2], target = 5,
 * 输出:
 * [
 * [1,2,2],
 * [5]
 * ]
 * 
 * 
 * 
 * 提示:
 * 
 * 
 * 1 
 * 1 
 * 1 
 * 
 * 
 */

struct Solution;

// @lc code=start
impl Solution {
    /// ## 解题思路
    /// 本题在39.组合总和基础上，增加两个改进即可
    pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {

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
                        .filter(|(i, &c)| (*i < 1 as usize) || c != candidates[i-1] )
                        .for_each(|(i, c)| {
                            let mut s = sub.to_vec();
                            s.push(*c);
                            combine_sum(&candidates[i+1..], left-c, &s, res);
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

