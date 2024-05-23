
#include <stdint.h>

enum Side {
    None = 0,
    Red = 1,
    Blue = 2,
};

struct GridStep
{
    public:
    int x;
    int y;
    Side color;
    static GridStep FromI64(uint64_t num)
    {
        GridStep step;
        step.color = (Side)(num & 0xff);
        step.x = ((num >> 8) & 0xff) ;
        step.y = ((num >> 16) & 0xff) ;
        return step;
    }
};

Side Line(int* grid, int x,int y, int x2,int y2, int x3,int  y3) {
    auto i1 = grid[y * 3 + x];
    auto i2 = grid[y2 * 3 + x2];
    auto i3 = grid[y3 * 3 + x3];
    if (i1 == 0 || i2 == 0 || i3 == 0)
        return Side::None;
    if (i1 == i2 && i1 == i3)
        return (Side)i1;
    return Side::None;
}
Side IsWin(int* grid) {
    for (int i= 0; i < 3; i++) {
        auto sideH = Line(grid,0, i, 1, i, 2, i);
        if (sideH != Side::None)
            return sideH;

        auto sideV = Line(grid,i, 0, i, 1, i, 2);
        if (sideV != Side::None)
            return sideV;
    }
    auto sidec1 = Line(grid,0, 0, 1, 1, 2, 2);
    if (sidec1 != Side::None)
        return sidec1;
    auto sidec2 = Line(grid,2, 0, 1, 1, 0, 2);
    if (sidec2 != Side::None)
        return sidec2;
    return Side::None;
}
void Game(int incount,uint64_t* inarr, int* outcount,uint64_t** outarr)
{
    int grid[9];//棋盘

     //移植一段下棋逻辑
    for (int i = 0; i < incount; i++) {
        GridStep step = GridStep::FromI64(inarr[i]);
        grid[step.y * 3 + step.x] = step.color;
    }
    //判断胜利者作为战果
    Side winner  = IsWin(grid);
    outcount[0]=1;
    outarr[0] = new uint64_t[1];
    outarr[0][0]=winner;

}