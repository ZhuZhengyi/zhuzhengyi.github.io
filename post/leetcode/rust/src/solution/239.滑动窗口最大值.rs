/*
 * @lc app=leetcode.cn id=239 lang=rust
 *
 * [239] 滑动窗口最大值
 *
 * https://leetcode-cn.com/problems/sliding-window-maximum/description/
 *
 * algorithms
 * Hard (49.73%)
 * Likes:    1350
 * Dislikes: 0
 * Total Accepted:    225.3K
 * Total Submissions: 453K
 * Testcase Example:  '[1,3,-1,-3,5,3,6,7]\n3'
 *
 * 给你一个整数数组 nums，有一个大小为 k 的滑动窗口从数组的最左侧移动到数组的最右侧。你只可以看到在滑动窗口内的 k
 * 个数字。滑动窗口每次只向右移动一位。
 *
 * 返回滑动窗口中的最大值。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：nums = [1,3,-1,-3,5,3,6,7], k = 3
 * 输出：[3,3,5,5,6,7]
 * 解释：
 * 滑动窗口的位置                最大值
 * ---------------               -----
 * [1  3  -1] -3  5  3  6  7       3
 * ⁠1 [3  -1  -3] 5  3  6  7       3
 * ⁠1  3 [-1  -3  5] 3  6  7       5
 * ⁠1  3  -1 [-3  5  3] 6  7       5
 * ⁠1  3  -1  -3 [5  3  6] 7       6
 * ⁠1  3  -1  -3  5 [3  6  7]      7
 *
 *
 * 示例 2：
 *
 *
 * 输入：nums = [1], k = 1
 * 输出：[1]
 *
 *
 * 示例 3：
 *
 *
 * 输入：nums = [1,-1], k = 1
 * 输出：[1,-1]
 *
 *
 * 示例 4：
 *
 *
 * 输入：nums = [9,11], k = 2
 * 输出：[11]
 *
 *
 * 示例 5：
 *
 *
 * 输入：nums = [4,-2], k = 2
 * 输出：[4]
 *
 *
 *
 * 提示：
 *
 *
 * 1
 * -10^4
 * 1
 *
 *
 */

// @lc code=start
use std::collections::BinaryHeap;

impl Solution {
    /// ## 解题思路
    /// - 优先队列/二叉堆
    /// 1. 对于窗口内的元素,维持一个二叉堆(大顶堆), 则堆顶元素即为当前窗口内的最大元素;
    /// 2. 考虑到窗口滑动时,
    ///    会同时有元素进入和离开窗口,在维持堆结构时,需保证堆中元素最大个数不超过窗口大小;
    /// 3. 为此,可使用(val, index)作为堆的键;
    /// 4. 窗口移动时,每增加一个元素到堆中, 同时判断堆顶元素的index是否落入当前窗口范围内;
    /// 5. 如果堆顶元素的index<i-k, 则堆顶元素不在窗口范围内, 则移除该堆顶元素;
    /// 6. 依次输出合法的堆顶元素val即为最终结果;
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut ret_vec = vec![];
        let k = k as usize;
        if k == 0 {
            return ret_vec;
        }
        let mut heap = BinaryHeap::new();
        //遍历各个元素
        nums.iter().enumerate().for_each(|(i, &n)| {
            // 将当前元素和index push到优先队列中
            heap.push((n, i));
            // 依次移除所有不在窗口之内的堆顶元素
            while let Some(&(_, l)) = heap.peek() {
                // 如果堆顶元素index
                if i - l > k - 1 {
                    heap.pop();
                } else {
                    break;
                }
            }
            // 当i>=k-1时, 说明已经生成完整窗口
            if i >= k - 1 {
                //获取堆顶元素, 为当前窗口的最大值
                if let Some(&(top, _)) = heap.peek() {
                    ret_vec.push(top);
                }
            }
        });

        ret_vec
    }
}
// @lc code=end

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::max_sliding_window(vec![1, 3, -1, -3, 5, 3, 6, 7], 3),
            vec![3, 3, 5, 5, 6, 7]
        );
    }
}
