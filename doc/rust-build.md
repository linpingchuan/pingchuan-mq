# pingchuan-mq 环境构建开发文档

## 开发环境搭建命令

```shell
cargo install racer

rustup update rustup component add rls-preview rust-analysis rust-src

cargo install rustfmt

cargo install rustsym

rustup self update

rustup update nightly

rustup component add rls –toolchain nightly

rustup component add rust-analysis –toolchain nightly

rustup component add rust-src –toolchain nightly

rustup component add rust-src –toolchain stable
```

## Rust 学习文档
1.[基本语法](http://rooat.com/custom_types.html)
2.[官方中文教程学习文档](https://kaisery.github.io/trpl-zh-cn/ch20-01-single-threaded.html)
3.[Rust 入门教程](https://rustcc.gitbooks.io/rustprimer/content/)