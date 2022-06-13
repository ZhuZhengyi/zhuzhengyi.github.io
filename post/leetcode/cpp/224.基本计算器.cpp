/*
 * @lc app=leetcode.cn id=224 lang=cpp
 *
 * [224] 基本计算器
 *
 * https://leetcode.cn/problems/basic-calculator/description/
 *
 * algorithms
 * Hard (41.92%)
 * Likes:    777
 * Dislikes: 0
 * Total Accepted:    91.5K
 * Total Submissions: 218.2K
 * Testcase Example:  '"1 + 1"'
 *
 * 给你一个字符串表达式 s ，请你实现一个基本计算器来计算并返回它的值。
 * 
 * 注意:不允许使用任何将字符串作为数学表达式计算的内置函数，比如 eval() 。
 * 
 * 
 * 
 * 示例 1：
 * 
 * 
 * 输入：s = "1 + 1"
 * 输出：2
 * 
 * 
 * 示例 2：
 * 
 * 
 * 输入：s = " 2-1 + 2 "
 * 输出：3
 * 
 * 
 * 示例 3：
 * 
 * 
 * 输入：s = "(1+(4+5+2)-3)+(6+8)"
 * 输出：23
 * 
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * 1 <= s.length <= 3 * 10^5
 * s 由数字、'+'、'-'、'('、')'、和 ' ' 组成
 * s 表示一个有效的表达式
 * '+' 不能用作一元运算(例如， "+1" 和 "+(2 + 3)" 无效)
 * '-' 可以用作一元运算(即 "-1" 和 "-(2 + 3)" 是有效的)
 * 输入中不存在两个连续的操作符
 * 每个数字和运行的计算将适合于一个有符号的 32位 整数
 * 
 * 
 */

#include <stack>
#include <string>

using namespace std;

// @lc code=start
class Solution {
public:
	/*
	## 解题思路
	* 对于表达式： 1+(4—(5+2))-3+(6+8)-(10-(19-3))
	           = 1 + 4 + (-5) + (-2) + (-3) + 6 + 8 + (-10) + (--19) + (---3) 
	* 可以看出, 结果 = 各个数字与其前面的符号 的 和 组成
	* 各个数字前面的符号由嵌套括号决定，每多一层括号，
	* 如果括号前为+, 则括号中的各个数字符号不变；
	* 如果括号前符号为-, 则括号中各个数字的符号取反；
	*/
	int calculate(string s) {
		stack<int> ops;		   //每个操作数的符号
		ops.push(1);
		int sign = 1;           //符号

		int n = s.length();     
		int i = 0;
		int ret = 0;
		while(i < n) {
			switch (s[i]) {
				case ' ':  			//空格，跳过
					i++; 
					break;
				case '+': 			//+, 上级符号不编号
					sign = ops.top();
					i++;
					break;
				case '-': 			//-，上级符号变相反；
					sign = -ops.top();
					i++;
					break;
				case '(': 			//左括号, 符号入栈
					ops.push(sign);
					i++;
					break;
				case ')':			//右括号，符号出栈
					ops.pop();
					i++;
					break;
				default:			//数字，计算累加和
					long num = 0;
					while (i < n && s[i] >= '0' && s[i] <= '9') {
						num = num * 10 + s[i] - '0';
						i++;
					}
					ret += sign * num;
					break;
			}
		}
		return ret;
	}
};
// @lc code=end

