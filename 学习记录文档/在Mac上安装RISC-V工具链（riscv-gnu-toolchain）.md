# 在Mac上安装RISC-V工具链（riscv-gnu-toolchain）

实验环境：macOS Big Sur 11.3.1

实验设备：MacBook Pro (intel, 16-inch, 2019)

前言：GDB 需要支持 riscv64 架构才能够对 rCore 进行 debug。

目的：使用 GDB 对 rCore 进行 debug

由于中国的GFW的原因，大家在连接github的时候，需要在macos上复制终端代理命令，使得终端可以走代理的流量（这种方法只在当前终端窗口有效，关闭或新建终端窗口后将失效）；操作如下图1所示，如果想长期有效，可参考该链接：https://github.com/FatliTalk/blog/issues/131

![image-20210605213402958](/Users/dali/Library/Application Support/typora-user-images/image-20210605213402958.png)

图1



![image-20210605213125974](/Users/dali/Library/Application Support/typora-user-images/image-20210605213125974.png)



完成了网络环境的初始化之后，接下来我们开始进如`riscv64-unknown-elf-gdb`的安装，有如下几个步骤：

1. 输入如下命令，把riscv-gnu-toolchain clone到本地

```text
git clone --recursive http://www.github.com/riscv/riscv-gnu-toolchain
```

2. 用brew安装依赖项

   ```text
   brew install python3 gawk gnu-sed gmp mpfr libmpc isl zlib expat
   ```

   3. 

