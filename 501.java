import java.util.HashMap;
class Solution {
    HashMap<Integer, Integer> map = new HashMap<>();
    public int[] findMode(TreeNode root) {
      
        traverse(root);
        int max = -1;
        int count = 0;
        for(int i : map.keySet()) {
            if(map.get(i) > max) {
                max = map.get(i);
                count = 1;
            }
            else if(map.get(i) == max) count++;
        }
        int[] sol = new int[count];
        count = 0;
        for(int i : map.keySet()) {
            if(map.get(i) == max) {
                sol[count] = i;
                count++;
            }
        }
        return sol;

    }

    private void traverse(TreeNode node) {
        if(node == null) return;
        if(map.containsKey(node.val)) {
            map.put(node.val, map.get(node.val) + 1);
        }
        else {
            map.put(node.val, 1);
        }
        traverse(node.left);
        traverse(node.right);
 
    }
}

  class TreeNode {
      int val;
      TreeNode left;
      TreeNode right;
      TreeNode() {}
      TreeNode(int val) { this.val = val; }
      TreeNode(int val, TreeNode left, TreeNode right) {
          this.val = val;
          this.left = left;
          this.right = right;
      }
  }
