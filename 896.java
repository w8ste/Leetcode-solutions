class Solution {
    public boolean isMonotonic(int[] nums) {
        boolean inc = true;
        boolean mode = false;
        for(int i = 0; i < nums.length - 1; i++) {
            if(mode) {
                if(inc && nums[i] > nums[i + 1]) return false;
                else if(!inc && nums[i] < nums[i + 1]) return false;
            }
            else {
                if(nums[i] > nums[i + 1]) {
                    mode = true;
                    inc = false;
                }
                else if(nums[i] < nums[i + 1]) {
                    mode = true;
                }
            }
        }
        return true;
    }
}
