/*
 * @lc app=leetcode.cn id=164 lang=rust
 *
 * [164] 最大间距
 *
 * https://leetcode.cn/problems/maximum-gap/description/
 *
 * algorithms
 * Hard (60.44%)
 * Likes:    530
 * Dislikes: 0
 * Total Accepted:    78K
 * Total Submissions: 129.1K
 * Testcase Example:  '[3,6,9,1]'
 *
 * 给定一个无序的数组 nums，返回 数组在排序之后，相邻元素之间最大的差值 。如果数组元素个数小于 2，则返回 0 。
 * 
 * 您必须编写一个在「线性时间」内运行并使用「线性额外空间」的算法。
 * 
 * 
 * 
 * 示例 1:
 * 
 * 
 * 输入: nums = [3,6,9,1]
 * 输出: 3
 * 解释: 排序后的数组是 [1,3,6,9], 其中相邻元素 (3,6) 和 (6,9) 之间都存在最大差值 3。
 * 
 * 示例 2:
 * 
 * 
 * 输入: nums = [10]
 * 输出: 0
 * 解释: 数组元素个数小于 2，因此返回 0。
 * 
 * 
 * 
 * 提示:
 * 
 * 
 * 1 <= nums.length <= 10^5
 * 0 <= nums[i] <= 10^9
 * 
 * 
 */

// @lc code=start
impl Solution {
    /// ## 解题思路
    /// - 基数排序
    /// 1. 先排序，再计算排序后每两个相邻数grap中的最大值；
    pub fn maximum_gap(mut nums: Vec<i32>) -> i32 {
        Self::radix_sort_base10(&mut nums);
        nums.windows(2)
            .map(|w| w[1] - w[0])
            .max()
            .unwrap_or(0)
    }

    fn radix_sort_base10(nums: &mut [i32]) {
        let mut buckets = vec![vec![]; 10];

        for i in 0..10 {
            //将倒数第i位按数字依次放入buckets对应的桶中
            nums.iter()
                .for_each(|&n| buckets[((n/10_i32.pow(i)) % 10) as usize].push(n) );

            //
            buckets.iter()
                .flat_map(|b| b.iter())  //展开每个桶内的
                .zip(nums.iter_mut())    //
                .for_each(|(&x, y)| *y = x); //

            // 清理buckets
            buckets.iter_mut()
                .for_each(|b|b.clear());
        }

    }
}
// @lc code=end

