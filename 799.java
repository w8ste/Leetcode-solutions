class Solution {
    public double champagneTower(int poured, int query_row, int query_glass) {
        double[][] tower = new double[101][101];
        tower[0][0] = poured;
        for(int i = 0; i <= query_row; i++) {
            for(int j = 0; j <= i; j++) {
                if(tower[i][j] > 1) {
                    double share = (tower[i][j] - 1) / 2.0;
                    System.out.println(share);
                    tower[i + 1][j + 1] += share;
                    tower[i + 1][j] += share;
                    tower[i][j] = 1;
                }
            }
        }
        return tower[query_row][query_glass];
    }
}
