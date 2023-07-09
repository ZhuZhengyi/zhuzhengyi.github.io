/*
 * @lc app=leetcode.cn id=739 lang=rust
 *
 * [739] 每日温度
 *
 * https://leetcode.cn/problems/daily-temperatures/description/
 *
 * algorithms
 * Medium (68.90%)
 * Likes:    1535
 * Dislikes: 0
 * Total Accepted:    439.8K
 * Total Submissions: 638.6K
 * Testcase Example:  '[73,74,75,71,69,72,76,73]'
 *
 * 给定一个整数数组 temperatures ，表示每天的温度，返回一个数组 answer ，其中 answer[i] 是指对于第 i
 * 天，下一个更高温度出现在几天后。如果气温在这之后都不会升高，请在该位置用 0 来代替。
 *
 *
 *
 * 示例 1:
 *
 *
 * 输入: temperatures = [73,74,75,71,69,72,76,73]
 * 输出: [1,1,4,2,1,1,0,0]
 *
 *
 * 示例 2:
 *
 *
 * 输入: temperatures = [30,40,50,60]
 * 输出: [1,1,1,0]
 *
 *
 * 示例 3:
 *
 *
 * 输入: temperatures = [30,60,90]
 * 输出: [1,1,0]
 *
 *
 *
 * 提示：
 *
 *
 * 1 <= temperatures.length <= 10^5
 * 30 <= temperatures[i] <= 100
 *
 *
 */

// @lc code=start
impl Solution {
    /// ## 解题思路
    /// - 单调栈
    /// 1. 设置单调递减栈stack, 用于暂存温度降低的日期;
    /// 2. 从左->右遍历温度列表, 如果温度在降低, 则无需计算, 将当天日期入栈;
    /// 3. 如果温度升高, 则依次计算暂存栈中低于当天温度的日期的升温时间间隔;
    /// 4. 将当天日期入栈;
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut stack: Vec<usize> = Vec::with_capacity(temperatures.len());
        let mut res = vec![0; temperatures.len()];
        for (i, &t) in temperatures.iter().enumerate() {
            // 如果当前温度 > 栈顶那天的温度, 则栈中保存那天的温度在今天会升高
            while !stack.is_empty() && t > temperatures[*(stack.last().unwrap())] {
                // 取出栈顶温度低的那天
                if let Some(j) = stack.pop() {
                    res[j] = (i - j) as i32; //计算栈顶温度低的那天到今天升温间隔的天数
                }
            }
            //
            stack.push(i);
        }

        res
    }
}
// @lc code=end

struct Solution;
