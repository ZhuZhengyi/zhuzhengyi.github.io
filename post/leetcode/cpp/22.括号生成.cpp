/*
 * @lc app=leetcode.cn id=22 lang=cpp
 *
 * [22] 括号生成
 *
 * https://leetcode-cn.com/problems/generate-parentheses/description/
 *
 * algorithms
 * Medium (77.43%)
 * Likes:    2587
 * Dislikes: 0
 * Total Accepted:    490.1K
 * Total Submissions: 632.9K
 * Testcase Example:  '3'
 *
 * 数字 n 代表生成括号的对数，请你设计一个函数，用于能够生成所有可能的并且 有效的 括号组合。
 * 
 * 
 * 
 * 示例 1：
 * 
 * 
 * 输入：n = 3
 * 输出：["((()))","(()())","(())()","()(())","()()()"]
 * 
 * 
 * 示例 2：
 * 
 * 
 * 输入：n = 1
 * 输出：["()"]
 * 
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * 1 <= n <= 8
 * 
 * 
 */

#include<vector>
#include<string>

using namespace std;

// @lc code=start
class Solution {
    vector<string> result;

public:
    /*
    * ## 解题思路
    * * 回溯法
    * 1. 依次往path中放入'(',')'；
    * 2. 合法的parenthesis中，每次需保证先入'(', 
    *    因此有效的串中，每次入'(',')'中之一时，
    *    需要保证parenthesis中的'('个数大于等于')';
    * 3. 递归时，使用l, r分别记录剩余的'(',')数；
    */
    vector<string> generateParenthesis(int n) {
        if (n==0) {
            return result;
        }

        string path = "";
        dfs(path, n, n);

        return result;
    }

    void dfs(string& path, int l, int r) {
        if (l>r) {
            return;
        }
        if (l == 0 && r == 0) {
            result.push_back(path);
            return;
        }

        if (l>0) {
            path.push_back('(');
            dfs(path, l-1, r);
            path.pop_back();
        } 
        if (r>l) {
            path.push_back(')');
            dfs(path, l, r-1);
            path.pop_back();
        }
    }
};
// @lc code=end

