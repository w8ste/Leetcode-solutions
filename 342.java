class Solution {
    public boolean isPowerOfFour(int n) {
        for(int i = 0; i <= Math.ceil(Math.sqrt(Math.sqrt(n))); i++) {
            if(Math.pow(4, i) == n) return true;
        }
        return false;
    }
}
