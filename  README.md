项目目录介绍：

Rust_Project目录：为5月初学习Rust语法的笔记，该目录下的子目录src_youtube_backup为学习的源码；

rustlings目录：rustlings 4.5.0练习





## 2021.07.02 

回顾看过的Rust和rcore项目； 



## 2021.07.03 

由于在MacOS上，vm(virtual machine)的机器不能跟host(host machine)机器共用一个代理，导致需要重复在vm上连一个新的代理，为了方便（偷懒）一步到位地解决网络问题，通过几个月的不懈努力，终于解决了这个心头之恨；配置网络，在得益于大佬萧洛元的帮助，成功解决了vm不能和走host一个代理端口的问题；



## 2021.07.05

在中国大学慕课上，开始观看《计算机组成与设计：RISC-V》课程；



## 2021.07.07

《计算机组成与设计：RISC-V》课程观看结束；

感受：全英文课件，可以学习到原设计者的原汁原味的（Untranslated）知识；



## 2021.07.08

观看riscv汇编英文的介绍课程；

学习视频来源于[Youtube](https://www.youtube.com/channel/UC8t99gp5IN-FTf5rGVaRevw) (点击即可查看链接)

感受：与arm和x86不同，riscv架构讲究高性能，单单从汇编指令的精简性，就能看出riscv架构在这几种架构优势点；riscv 的指令集具有简洁高效，开源开放，是其主要的优势；



## 2021.07.08

Rustlings开始；

Rustlings版本： 4.5.0（20210708当前的最新版本）



## 2021.07.10

Rustlings结束；

感受：有了前面知识学习的铺垫，发现Rust并不是很难，甚至觉得比C++更好用（编译器自带的纠错功能真好用/狗头）；

自己的答案md写到第9个内容modules就放弃在md里写了，直接在rustlings里的源文件里写注释了；

答案参考这个[网站](https://blog.frankel.ch/start-rust/3/)，总结归纳做得非常好，很简洁的一个归纳；

部分思路来自外国开发者Théo Pomies对于github上的rustlings4.5.0的解析，点击[链接](https://github.com/theopomies/rustlings_solutions)即可查看如下：

![rustlings](/Users/dali/Desktop/Git/ README.assets/rustlings-7449022.png)

此图留念rustlings完结；



## 2021.07.11

[Rust-quiz](https://dtolnay.github.io/rust-quiz/1)完结；

![quiz](/Users/dali/Desktop/Git/ README.assets/quiz.png)



## 2021.07.12

由于本人经常使用的编程语言是C++，经陈渝老师推荐，开始阅读[《A Guide to Porting C and C++ code to Rust》](https://locka99.gitbooks.io/a-guide-to-porting-c-to-rust/content/    )；



## 2021.07.15

看完[《A Guide to Porting C and C++ code to Rust》](https://locka99.gitbooks.io/a-guide-to-porting-c-to-rust/content/    )；

由于本人有Python、Java、C++的使用经验，对于Rust的语法，让我眼前一亮，其结合了不同语言的优缺点，作为一门安全的语言，其编译器的纠错提醒功能让我十分喜欢，希望今后能多用这门语言做出点事情；



## 2021.07.16

强推CSAPP这本书及本书作者配套的视频课；

回顾CSAPP的笔记，发现这本书及本书作者配套的视频课，让我真的受益匪浅，能让我深层次的理解计算机为什么这样设计，历代不同指令集的发展史，对比新兴起的RISCV指令集，更让我看到了RISCV指令集的活力以及无穷的潜力；

下图为5月~6月观看CSAPP视频课的笔记；

![csapp1](/Users/dali/Desktop/Git/ README.assets/csapp1.png)

![csapp2](/Users/dali/Desktop/Git/ README.assets/csapp2.png)



## 2021.07.17

看RISC-V开发者手册《RISC-V-Reader-Chinese-v2》；

其实这本书我5月初就开始阅读了，但是完全掌握书上的内容，短时间内似乎无法实现；

关于各个不同寄存器的相关用途，目前只能记住常用的寄存器的相关用法，其他生僻的内容，只能靠CTRL+F来查阅相关文档；



## 2021.07.18

发现两个月前在gitbook下提的建议被采纳了；

![gitbook1](/Users/dali/Desktop/Git/ README.assets/gitbook1.png)

![gitbook2](/Users/dali/Desktop/Git/ README.assets/gitbook2.png)



把rCore-Tutorial部署到Linux：

![image-20210727022531652](/Users/dali/Desktop/Git/ README.assets/image-20210727022531652.png)

把rCore-Tutorial部署到MacOS：

![image-20210727022459117](/Users/dali/Desktop/Git/ README.assets/image-20210727022459117.png)

MacOS上启动rcore，至此，lab0完结；

![image-20210727024912573](/Users/dali/Desktop/Git/ README.assets/image-20210727024912573.png)



## 2021.07.19

Lab1；

![lab1](/Users/dali/Desktop/Git/ README.assets/lab1.png)



## 2021.07.21

lab没做完，后续有精力再研究研究；



## 2021.07.23

玩硬件，熟悉树莓派RISCV开发板、树莓派ARM开发板、某空气质量检测仪（芯片为MXCHIP EMW3080）的电路的基本结构，观看EMW3080的开发者手册，以及Windows的调试软件（SecureCRT）、硬件（J-LINK）、驱动（J-LINK）；

观看了某大神关于智能设备的教程，希望能把该功能应用于riscv板子上，点击[链接](https://github.com/a2633063)即可查看；

阅读了[《物联网安全百科》](https://iot-security.wiki/hardware-security/firmware/reverse.html)，熟悉相关软硬件安全策略，未曾接触过此类知识，里面有关于软硬件调试的方法：





## 2021.07.24

找到一些no_std的基于rust语言的gba项目，后续会把这些项目合并移植到zcore上；

arm-embedded（可以移植）：

https://crates.io/crates/gba/versions

[no_std]的（现成轮子）：

https://github.com/ryankurte/rust-gba



## 2021.07.25

今天才开始学习git命令；

项目没能成功push到仓库；

月初的时候的遗留文件未能成功清理；



## 2021.07.26

萧洛元的指点下，发现了codechina仓库push权限关闭了，在codechina上打开权限后，成功git push成功；

现在把我从5月初到7月底的大部分工作传到本仓库；
