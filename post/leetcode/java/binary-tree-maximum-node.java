/*
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/binary-tree-maximum-node
@Language: Java
@Datetime: 17-03-10 14:14
*/

public class Solution {
    /**
     * @param root the root of binary tree
     * @return the max ndoe
     */
    public TreeNode maxNode(TreeNode root) {
        // Write your code here
        if (root == null) {
            return null;
        }
        TreeNode t = root;
        if (root.left != null) {
            TreeNode l = this.maxNode(root.left);
            if (l.val > t.val) {
                t = l;
            }
        }
        if (root.right != null) {
            TreeNode l = this.maxNode(root.right);
            if (l.val > t.val) {
                t = l;
            }
        }
        return t;
        
    }
}