学习zkwasm 在此测试

2024/05/22 测试经验

 cargo run --release -- --params params name  setup --host default --wasm ../bin/zk.wasm

 git 不要在windows 分区做

 wsl+ubuntu22.04 通过

# setup 测试结果  
三子棋简单逻辑  
## CPP 失败  
## AssemblyScript 失败  
## C语言 成功  
## Rust 成功  

# provf 测试结果
## CPP 成功
## Rust 成功
```
# test1——heelo 的test数据
# 检测 public 和 private 相等
# prove 命令行
cargo run --release -- --params params name  prove  --wasm ../bin/zk_test1.wasm --output ../p2  --public 0:i64 --private 0:i64
```
Hello 成功
三字棋 成功

```
# testc1_game的test数据
# private 落子数据，0步，如果有很多步，每一步一个int64，内部编码 x y 颜色
# public 结果数据，长度1，结果 平局
# prove 命令行
cargo run --release -- --params params name  prove  --wasm ../bin/zk_testc1.wasm --output ../p2  --public 1:i64,0:i64 --private 0:i64
```

# 后续问题
## 1.选择什么语言开发逻辑
首选assemblyscript，暂时跑不起。 （语法贴近ts）  
次选cpp，暂时跑不起。           （熟悉的程序员多，可以表达一定面向对象逻辑）  
三选c语言，目前测试正常。       （熟悉的程序员多）  
四选rust，目前测试正常。        （rust编译器限制多，语法新颖，在我们的技能树上没有，基于我们的情况开发效率最低。）  



## 2.基础的环境怎么提供
现在的zkwasm 是一个功能，还不是一个服务。

WASM 有了 谁来部署，怎么跑Prove，怎么调verify 总不能命令行，得有个服务器基建，这部分如何处理。

## 3.进一步测试

选取 两个实际业务点 确定语言 后 进行开发实测



