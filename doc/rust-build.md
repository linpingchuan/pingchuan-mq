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