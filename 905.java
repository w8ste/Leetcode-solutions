class Solution {
    public int[] sortArrayByParity(int[] nums) {
        int j = nums.length - 1;
        int i = 0;
        while(i < nums.length && j>=i) {
            if(nums[i] % 2 != 0 && nums[j] % 2 == 0){
                nums = swap(nums,i, j);
                i++;
                j--;
            }
            else if(nums[i] % 2 == 0) {
                i++;
            }
            else if(nums[j] % 2 == 1) j--;
            else {
                j--;
                i++;
            }
        }
        return nums;
    }
    private int[] swap (int[] arr, int i, int j) {
        int tmp =arr[i];
        arr[i]=arr[j];
        arr[j] = tmp;
        return arr;
    }
}
