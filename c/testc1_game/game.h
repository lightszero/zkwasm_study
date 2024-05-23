
#include <stdint.h>

enum Side {
    None = 0,
    Red = 1,
    Blue = 2,
};

struct GridStep
{
   
    int x;
    int y;
    enum Side color;
   
};
struct  GridStep FromI64(uint64_t num)
    {
        struct GridStep step;
        step.color = (enum Side)(num & 0xff);
        step.x = ((num >> 8) & 0xff) ;
        step.y = ((num >> 16) & 0xff) ;
        return step;
    }
enum Side Line(int* grid, int x,int y, int x2,int y2, int x3,int  y3) {
    int i1 = grid[y * 3 + x];
    int i2 = grid[y2 * 3 + x2];
    int i3 = grid[y3 * 3 + x3];
    if (i1 == 0 || i2 == 0 || i3 == 0)
        return None;
    if (i1 == i2 && i1 == i3)
        return i1;
    return None;
}
enum Side IsWin(int* grid) {
    for (int i= 0; i < 3; i++) {
        enum Side sideH = Line(grid,0, i, 1, i, 2, i);
        if (sideH != None)
            return sideH;

         enum Side sideV = Line(grid,i, 0, i, 1, i, 2);
        if (sideV !=  None)
            return sideV;
    }
     enum Side sidec1 = Line(grid,0, 0, 1, 1, 2, 2);
    if (sidec1 !=None)
        return sidec1;
     enum Side sidec2 = Line(grid,2, 0, 1, 1, 0, 2);
    if (sidec2 != None)
        return sidec2;
    return None;
}
uint64_t outarr[1];
void Game(int incount,uint64_t* inarr, int* outcount,uint64_t** outarr)
{
    int grid[9];//棋盘

     //移植一段下棋逻辑
    for (int i = 0; i < incount; i++) {
        struct GridStep step = FromI64(inarr[i]);
        grid[step.y * 3 + step.x] = step.color;
    }
    //判断胜利者作为战果
    enum Side winner  = IsWin(grid);
    outcount[0]=1;
    outarr[0] =outarr;
    outarr[0][0]=winner;

}