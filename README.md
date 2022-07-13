
<div align="center">
<h2> space-invaders Emulator</h2>
一个使用Rust编写的《太空侵略者》仿真器
<br>
<br>
<img width=300 src="https://raw.githubusercontent.com/flash-game/space-invaders-8080/master/resource/space.webp" />
</div>

## Intel 8080 介绍
>   Intel 8080为英特尔早期发布的处理器。它于1974年4月发布，是一枚8位元处理器，主频为2MHz，它集成6000只晶体管，每秒运算29万次，拥有16位地址总线和八位数据总线，包含7个八位寄存器，支持16位寻址，同时它也包含一些输入输出端口，这也是一个相当成功的设计，有效解决了外部设备在内存寻址能力不足的问题。
CPU部分使用Rust实现了Intel的 8080 CPU

本程序CPU模拟部分使用 `Rust` 语言编写。

## 太空侵略者
> 《太空侵略者》（日语：スペースインベーダー，英语：Space Invaders）为日本太东公司于1978年发行之一款街机游戏，在美国由Midway发行。常简称为《侵略者》，或翻做《宇宙入侵者》。因他厂跟风游戏采昆虫造型，而在台湾对此类游戏昵称“小蜜蜂”。<br><br> 游戏规格是基于Intel公司8位元的Intel 8080处理器而设计的。

## How to build
```
cargo build --release
```

## How to run
```
.\target\release\space-invaders-8080.exe
```
