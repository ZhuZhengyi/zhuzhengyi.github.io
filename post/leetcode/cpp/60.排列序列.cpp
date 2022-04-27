/*
 * @lc app=leetcode.cn id=60 lang=cpp
 *
 * [60] 排列序列
 *
 * https://leetcode-cn.com/problems/permutation-sequence/description/
 *
 * algorithms
 * Hard (52.96%)
 * Likes:    659
 * Dislikes: 0
 * Total Accepted:    105.1K
 * Total Submissions: 198.4K
 * Testcase Example:  '3\n3'
 *
 * 给出集合 [1,2,3,...,n]，其所有元素共有 n! 种排列。
 * 
 * 按大小顺序列出所有排列情况，并一一标记，当 n = 3 时, 所有排列如下：
 * 
 * 
 * "123"
 * "132"
 * "213"
 * "231"
 * "312"
 * "321"
 * 
 * 
 * 给定 n 和 k，返回第 k 个排列。
 * 
 * 
 * 
 * 示例 1：
 * 
 * 
 * 输入：n = 3, k = 3
 * 输出："213"
 * 
 * 
 * 示例 2：
 * 
 * 
 * 输入：n = 4, k = 9
 * 输出："2314"
 * 
 * 
 * 示例 3：
 * 
 * 
 * 输入：n = 3, k = 1
 * 输出："123"
 * 
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * 1 
 * 1 
 * 
 * 
 */

#include <string>

using namespace std;

// @lc code=start 
class Solution {
public:
    /*
    * ## 解题思路
    * * 数学推导
    * 1. n个字符的全排列总数为n!;
    * 2. 以任意一个字符为首字符的排列个数均(n-1)!；
    * 3. 因此，k = c1 * (n-1)! + x, 其中，c1为首字符；
    *    则第一个字符 c1 = [k / (n-1)!]， []为下取底；
    *    f(n, k) = c1 + f(n-1, k-1)
    * 4. 解题时，可以用一个字母表"123456789"来记录待取的字符；
    * 5. 将1..k转换为0..(k-1), 可以和上面的字母表统一，且能很好的计算
    */
    string getPermutation(int n, int k) {
        string letters = string("123456789").substr(0, n);
        string result = "";
        --k;  //将1..k转化为0..(k-1)
        while (k>0) {
            int i = k / getFactor(n-1) ;
            result.push_back(letters[i]);
            letters.erase(letters.begin()+i);
            k %= getFactor(n-1);
            n--;
        }

        return result+letters;
    }

    int getFactor(int n) {
        if (n==0) {
            return 1;
        }
        return n*getFactor(n-1);
    }
};
// @lc code=end

