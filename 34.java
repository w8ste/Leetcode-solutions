import java.util.*;

class Solution {
    public int[] searchRange(int[] nums, int target) {
        int[] def = {-1, -1};
        if(nums.length == 0 || nums[0] > target || nums[nums.length - 1] < target) {
         return def;
        }
        ArrayList<Integer> list = new ArrayList<>();
        for(int i = 0; i < nums.length; i++) {
            if(nums[i] == target) {
                list.add(i);
            }
            else if(nums[i] > target) break;
        }
        if(list.size() > 0) {
            int[] t = new int[2];
            t[0] = list.get(0);
            t[1] = list.get(list.size() - 1);
            return t;
        }
        return def;
    }
}
