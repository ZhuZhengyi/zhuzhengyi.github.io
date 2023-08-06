/*
 * @lc app=leetcode.cn id=4 lang=rust
 *
 * [4] 寻找两个正序数组的中位数
 *
 * https://leetcode-cn.com/problems/median-of-two-sorted-arrays/description/
 *
 * algorithms
 * Hard (41.03%)
 * Likes:    4789
 * Dislikes: 0
 * Total Accepted:    570.3K
 * Total Submissions: 1.4M
 * Testcase Example:  '[1,3]\n[2]'
 *
 * 给定两个大小分别为 m 和 n 的正序（从小到大）数组 nums1 和 nums2。请你找出并返回这两个正序数组的 中位数 。
 *
 * 算法的时间复杂度应该为 O(log (m+n)) 。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：nums1 = [1,3], nums2 = [2]
 * 输出：2.00000
 * 解释：合并数组 = [1,2,3] ，中位数 2
 *
 *
 * 示例 2：
 *
 *
 * 输入：nums1 = [1,2], nums2 = [3,4]
 * 输出：2.50000
 * 解释：合并数组 = [1,2,3,4] ，中位数 (2 + 3) / 2 = 2.5
 *
 *
 * 示例 3：
 *
 *
 * 输入：nums1 = [0,0], nums2 = [0,0]
 * 输出：0.00000
 *
 *
 * 示例 4：
 *
 *
 * 输入：nums1 = [], nums2 = [1]
 * 输出：1.00000
 *
 *
 * 示例 5：
 *
 *
 * 输入：nums1 = [2], nums2 = []
 * 输出：2.00000
 *
 *
 *
 *
 * 提示：
 *
 *
 * nums1.length == m
 * nums2.length == n
 * 0 <= m <= 1000
 * 0 <= n <= 1000
 * 1 <= m + n <= 2000
 * -10^6 <= nums1[i], nums2[i] <= 10^6
 *
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    /// ## 解题思路
    /// - 二分查找法
    /// 1. 对于nums1, nums2, 排序后的中位数按合并后数组(设为nums)总长度(l1+l2)可以分为两种：
    ///     a. 总长度为偶数, 中位数为(nums[(l1+l2)/2]+nums2[]/2);
    ///     b. 总长度为奇数，中位数为中间数(nums[(l1+l2)/2])；
    /// 2. 由于nums1，nums2都为有序数组，合并后nums1,nums2各元素的先后次序不变；
    /// 3. 合并后数组中位数处将nums1,nums2分别切为前后两个部分，设切开元素的索引分别为i，j：
    ///     nums1[0], nums1[1], ..., nums1[i-1], | nums1[i], ..., nums1[m]
    ///     nums2[0], nums2[1], ..., nums2[j-1], | nums2[j], ..., nums2[n]
    ///     显然, 必须满足：
    ///         nums1[i-1] <= nums2[j]    ......... (1)
    ///         nums2[j-1] <= nums1[i]  ......... (2)
    ///     可以证明，(1)可推导出(2)
    /// 4. 如果总数组长度为偶数，则：
    ///     len(nums1[0..i-1]) + len(nums2[0..j_1]) = len(nums1[i..]) + len(nums2[j..]),
    ///    即：
    ///      i+j = len(nums1) -i + len(nums2) -j
    ///    => i + j = (len(nums1) + len(nums2) ) / 2
    ///      此时中位数为:
    ///             (max(nums1[i-1], nums2[j-1]) + min(nums1[i], nums2[j]) ) / 2
    ///    如果总长度为奇数，则：
    ///     len(nums1[0..i-1]) + len(nums2[0..j-1]) = len(nums1[i..]) + len(nums2[j..]) + 1
    ///    即：
    ///     i+j = len(nums1) -i + len(nums2) -j  + 1
    ///    =>  i + j = (nums1.len() + nums2.len() + 1) / 2
    ///    此时中位数为:
    ///             max(nums[i-1], nums[j-1])
    ///    将上述两种情况综合一下，i,j 满足如下：
    ///       i + j = (nums1.len() + nums2.len() + 1) / 2
    ///    =>  
    ///       j =  (nums1.len() + nums2.len() + 1) / 2 - i
    /// 5. 因此，只需要找到满足条件的最大i, 使nums1[i-1] <= nums2[j] (j = (n1+n2+1)/2 -i) 即可；
    /// 6. 由于nums1，nums2都是有序的，可以使用二分查找来确定i;
    /// 7. 处理边界情况
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        // 将nums1，nums2中数组长度短的置前
        let (nums1, nums2) = if nums1.len() <= nums2.len() {
            (nums1, nums2)
        } else {
            (nums2, nums1)
        };

        // 二分法查找nums1[i], 使得nums1[i] > nums2[j] && nums1[i-1]<=nums2[j]) (j=(l1+l2+1)/2-i)
        let (l1, l2) = (nums1.len(), nums2.len());
        let (mut l, mut r) = (0, l1);
        while l <= r {
            let i = (l + r) / 2;
            let j = (l1 + l2 + 1) / 2 - i;

            let num1_i_1 = *nums1.get(i - 1).unwrap_or(&i32::MIN);
            let num2_j = *nums2.get(j).unwrap_or(&i32::MAX);
            if num1_i_1 <= num2_j {
                l = i + 1;
            } else {
                r = i - 1;
            }
        }
        // 找到i

        // 根据i, 中间中位数
        let i = (l + r) / 2;
        let j = (l1 + l2 + 1) / 2 - i;

        let num1_i_1 = *nums1.get(i - 1).unwrap_or(&i32::MIN);
        let num2_j_1 = *nums2.get(j - 1).unwrap_or(&i32::MIN);
        let num1_i = *nums1.get(i).unwrap_or(&i32::MAX);
        let num2_j = *nums2.get(j).unwrap_or(&i32::MAX);

        //根据总长度奇偶性，计算中位数
        match (l1 + l2) % 2 {
            0 => {
                // 总长度为偶数, 中位数为
                (std::cmp::max(num1_i_1, num2_j_1) + std::cmp::min(num1_i, num2_j)) as f64 / 2.0_f64
            }
            // 总长度为奇数, 中位数为nums1[i-1], nums2[j-1]中大的那个
            _ => (std::cmp::max(num1_i_1, num2_j_1)) as f64,
        }
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::find_median_sorted_arrays([1, 3].into(), [2].into()),
            2.0_f64
        );
    }
    #[test]
    fn test2() {
        assert_eq!(
            Solution::find_median_sorted_arrays([1, 2].into(), [3, 4].into()),
            2.5_f64
        );
    }
}
