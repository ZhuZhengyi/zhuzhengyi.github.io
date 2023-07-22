/*
 * @lc app=leetcode.cn id=219 lang=rust
 *
 * [219] 存在重复元素 II
 *
 * https://leetcode.cn/problems/contains-duplicate-ii/description/
 *
 * algorithms
 * Easy (44.26%)
 * Likes:    610
 * Dislikes: 0
 * Total Accepted:    238.5K
 * Total Submissions: 538.2K
 * Testcase Example:  '[1,2,3,1]\n3'
 *
 * 给你一个整数数组 nums 和一个整数 k ，判断数组中是否存在两个 不同的索引 i 和 j ，满足 nums[i] == nums[j] 且
 * abs(i - j) <= k 。如果存在，返回 true ；否则，返回 false 。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：nums = [1,2,3,1], k = 3
 * 输出：true
 *
 * 示例 2：
 *
 *
 * 输入：nums = [1,0,1,1], k = 1
 * 输出：true
 *
 * 示例 3：
 *
 *
 * 输入：nums = [1,2,3,1,2,3], k = 2
 * 输出：false
 *
 *
 *
 *
 *
 * 提示：
 *
 *
 * 1 <= nums.length <= 10^5
 * -10^9 <= nums[i] <= 10^9
 * 0 <= k <= 10^5
 *
 *
 */

// @lc code=start
impl Solution {
    /// ## 解题思路
    /// - hashset + 滑动窗口
    /// 1. 设置set作为滑窗, 滑窗(set)初始大小为空;
    /// 2. 从左至右遍历nums[i];
    /// 3. 如果i > k, 则删除set中最左边的元素nums[i-k-1];
    /// 4. 检查滑窗中是否存在和当前元素nums[i]相等的元素,
    ///    如果存在,则返回说明nums中存在距离k以内的重复元素, 返回true;
    /// 5. 否则将nums[i]加入到滑窗中;
    /// 6. 如果遍历nums中所有元素后, 一直不存在距离小于k的重复元素, 则返回false;
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        if nums.len() == 0 || k == 0 {
            return false;
        }
        let k = k as usize;
        use std::collections::HashSet;
        let mut tmp_win: HashSet<i32> = HashSet::with_capacity(k);
        for (i, &n) in nums.iter().enumerate() {
            if i > k {
                tmp_win.remove(&nums[i - k - 1]);
            }
            if tmp_win.contains(&n) {
                return true;
            }
            tmp_win.insert(n);
        }

        return false;
    }
}
// @lc code=end

struct Solution;
