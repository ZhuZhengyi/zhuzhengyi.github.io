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

// @lc code=start
impl Solution {
    /// ## 解题思路
    /// 1. 先按两个数组的长度对数组进行区分，明确；
    /// 2. 
    /// 
    /// 
    ///              m_total
    ///   [ 4, 7, 10, (23), 40, 85, 88]
    ///        [3, 5, (8), 10, 15]
    ///               m1
    /// 
    /// 
    ///              m_total
    ///   [ 4, 7, 10, (23), 40, 85, 88]
    ///        [3, 5, (28), 30, 45]
    ///               m1
    /// 
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        // 按数组长度
        let (nums1, nums2) = if nums1.len() < nums2.len() {
            (nums1, nums2) 
        } else { 
            (nums2, nums1) 
        };

        let (l1, l2) = (nums1.len(), nums2.len());
        let (mut l, mut h) = (0, l1);

        //
        let m_total = (l1+l2-1)/2;     //总数组中点
        let mut m1 = (l + h) / 2;      //短数组中点
        //
        while l < h {
            m1 = (l + h) / 2;
            if m1 > m_total-1 || nums1[m1] >= nums2[m_total-m1-1] {
                h = m1;
            } else { 
                l = m1 + 1;
            }
        }
        m1 = l;

        let s1 = &nums1[m1..(m1+2)];
        let s2 = &nums2[(m_total-m1)..(m_total-m1+2)];
        let mut newfew = vec![s1, s2].concat();
        newfew.sort();
        
        let x = (l1+l2) % 2 ;
        let mut res = newfew[0] as f64;
        if x == 1 {
            res = ( newfew[0] + newfew[1] )  as f64/ 2.0_f64 ;
        }
        res
    }
}
// @lc code=end

