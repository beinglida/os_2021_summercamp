Rust编写OS 1：

https://zhuanlan.zhihu.com/p/53064186

cargo build --target thumbv7em-none-eabihf



Rust编写OS 2：

bootimage工具[[1\]](https://zhuanlan.zhihu.com/p/56433770#ref_1)——它能够自动而方便地为你的内核准备一个引导程序。

现在我们可以使用`xbuild`代替`build`重新编译：

```rust
cargo xbuild --target x86_64-blog_os.json
```

