/*
 * @lc app=leetcode.cn id=33 lang=rust
 *
 * [33] 搜索旋转排序数组
 *
 * https://leetcode.cn/problems/search-in-rotated-sorted-array/description/
 *
 * algorithms
 * Medium (43.81%)
 * Likes:    2570
 * Dislikes: 0
 * Total Accepted:    714.1K
 * Total Submissions: 1.6M
 * Testcase Example:  '[4,5,6,7,0,1,2]\n0'
 *
 * 整数数组 nums 按升序排列，数组中的值 互不相同 。
 *
 * 在传递给函数之前，nums 在预先未知的某个下标 k（0 <= k < nums.length）上进行了 旋转，使数组变为 [nums[k],
 * nums[k+1], ..., nums[n-1], nums[0], nums[1], ..., nums[k-1]]（下标 从 0 开始
 * 计数）。例如， [0,1,2,4,5,6,7] 在下标 3 处经旋转后可能变为 [4,5,6,7,0,1,2] 。
 *
 * 给你 旋转后 的数组 nums 和一个整数 target ，如果 nums 中存在这个目标值 target ，则返回它的下标，否则返回 -1 。
 *
 * 你必须设计一个时间复杂度为 O(log n) 的算法解决此问题。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：nums = [4,5,6,7,0,1,2], target = 0
 * 输出：4
 *
 *
 * 示例 2：
 *
 *
 * 输入：nums = [4,5,6,7,0,1,2], target = 3
 * 输出：-1
 *
 * 示例 3：
 *
 *
 * 输入：nums = [1], target = 0
 * 输出：-1
 *
 *
 *
 *
 * 提示：
 *
 *
 * 1 <= nums.length <= 5000
 * -10^4 <= nums[i] <= 10^4
 * nums 中的每个值都 独一无二
 * 题目数据保证 nums 在预先未知的某个下标上进行了旋转
 * -10^4 <= target <= 10^4
 *
 *
 */

// @lc code=start
impl Solution {
    /// ## 解题思路
    /// - 二分查找
    /// 1. 旋转数组的旋转点(nums[i] < nums[i-1]的点)将数组分为左右两个有序部分;
    /// 2. 二分查找时, 首先根据中间数值nums[mid]和左端点nums[l]来判断旋转点在左半区间还是右半区间;
    /// 3. 当nums[mid]>nums[l]时, 旋转点落在右半区间, 此时左半区间[l, mid]是单调递增的;
    ///    此时如果target落在[l, mid]之间, 则缩减右半区间(r=mid-1);
    ///    否则, 收缩左半区间(l=mid+1);
    /// 4. 否则,当nums[mid]<nums[l]时, 此时旋转点落在左边,右半区间[mid, r]单调递增;
    ///    此时如果 target 落在[mid, r]之间, 则可以收缩左半区间(l=mid+1);
    ///    否则, 收缩右半区间(r=mid-1);
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        if nums.len() == 1 {
            if nums[0] == target {
                return 0;
            } else {
                return -1;
            }
        }
        let (mut l, mut r) = (0, nums.len() - 1);
        while l < r {
            if nums[l] == target {
                return l as i32;
            }
            if nums[r] == target {
                return r as i32;
            }
            let mid = (l + r + 1) / 2;
            if nums[mid] == target {
                return mid as i32;
            }
            //中间数的值>左端的数的值, 则左边长, 右边短
            if nums[mid] > nums[l] {
                //target落在(l..mid)之间
                if target > nums[l] && target < nums[mid] {
                    r = mid - 1; // 缩减右边
                } else {
                    // 否则, target落在(mid..r)之间
                    l = mid + 1;
                }
            } else {
                //中间数的值<左端点数的值, 则中点左边短,右边长
                // target 落在(mid..r)之间
                if target > nums[mid] && target < nums[r] {
                    l = mid + 1;
                } else {
                    // 否则, target落在(l..mid)之间
                    r = mid - 1;
                }
            }
        }

        -1
    }
}
// @lc code=end

struct Solution;
