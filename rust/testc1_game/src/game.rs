
//文件名 game 自动成为一个mode
#[derive(Copy, Clone)]
enum Side {
    None = 0,
    Red = 1,
    Blue = 2,
}
impl Side {
    fn from(value: u8) -> Side{
        match value {
            0 => Side::None,
            1 => Side::Red,
            2 => Side::Blue,
            _ => Side::None,
        }
    }
}
struct GridStep
{
    x:i32,
    y:i32,
    color:Side, 
}
fn  FromI64(num:u64) -> GridStep
    {
        let mut step:GridStep=GridStep{
            x:0,
            y:0,
            color:Side::None,
        } ;
        
        step.color = Side::from((num & 0xff) as u8);
        step.x = ((num >> 8) & 0xff) as i32;
        step.y = ((num >> 16) & 0xff) as i32 ;
        return step;
    }
  fn   Line(grid:*const Side,  x:i32, y:i32,  x2:i32, y2:i32, x3:i32,  y3:i32) ->Side {
    unsafe
    {
        let mut i1:Side = *grid.offset((y * 3 + x) as isize);
        let mut i2:Side = *grid.offset((y2 * 3 + x2) as isize);
        let mut i3:Side = *grid.offset((y3 * 3 + x3) as isize);
        if (i1 as i32 == 0 || i2 as i32 == 0 || i3 as i32 == 0)
        {
            return Side::None;
        }
        if (i1 as i32 == i2 as i32 && i1 as i32 == i3 as i32 )
        {
            return i1;
        }
        return Side::None;
    }
}
 fn  IsWin(grid:*const Side)->Side {
    unsafe
    {
        for i in 0 .. 3 {
            let sideH = Line(grid,0, i, 1, i, 2, i);
            if (sideH as i32 != Side::None as i32 )
            {
                return sideH;
            }
                let sideV = Line(grid,i, 0, i, 1, i, 2);
            if (sideV as i32 != Side::None as i32)
            {
                return sideV;
            }
        }
        let sidec1 = Line(grid,0, 0, 1, 1, 2, 2);
        if (sidec1 as i32 != Side::None as i32)
        {
            return sidec1;
        }
        let sidec2 = Line(grid,2, 0, 1, 1, 0, 2);
        if (sidec2 as i32 != Side::None as i32)
        {
            return sidec2;
        }
        return Side::None;
    }
    }
pub fn DoGame(counta:usize,inarr:*const u64, countb:&mut usize,arrb:*mut u64)
{
    //大量unsafe 逻辑
    unsafe
    {
        let mut grid: [Side; 9]=[Side::None; 9];
    //     int grid[9];//棋盘

    //     //移植一段下棋逻辑
       for n in 0 .. counta {
                let  step = FromI64(*inarr.offset(n as isize));
    //        grid[step.y * 3 + step.x] = step.color;
        }
    //    //判断胜利者作为战果
        let ptr: *const Side = grid.as_ptr();
        let winner  = IsWin(ptr);
        *countb = 1 ;
        *arrb.add(0) =winner as u64;
        //arrb[0]=winner;
    //    outcount[0]=1;
    //    outarr[0] = new uint64_t[1];
    //    outarr[0][0]=winner;
   
    }

}