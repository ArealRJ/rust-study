# rust-study
rust程序设计语言学习


## Cargo
### Cargo 常用指令

- `cargo build` 编译项目
- `cargo build --release` 编译项目（优化）
- `cargo run` 运行项目
- `cargo check` 检查项目
- `cargo fmt` 格式化代码
- `cargo test` 运行测试
- `cargo doc` 生成文档

### Cargo.toml 和 Cargo.lock
Cargo.toml 和 Cargo.lock 是 cargo 的核心文件，它的所有活动均基于此二者。

- `Cargo.toml` 是 cargo 特有的项目数据描述文件。它存储了项目的所有元配置信息，如果 Rust 开发者希望 Rust 项目能够按照期望的方式进行构建、测试和运行，那么，必须按照合理的方式构建 Cargo.toml。(类似package.json)

- `Cargo.lock` 文件是 cargo 工具根据同一项目的 toml 文件生成的项目依赖详细清单，因此我们一般不用修改它，只需要对着 Cargo.toml 文件撸就行了。

```toml
[package]
name = "world_hello"
version = "0.1.0"
edition = "2021"

#三种写法
#1.基于rust官方的crates.io，通关版本来描述
#2.通过项目源代码git仓库地址指定依赖
#3.基于本地路径指定依赖
[dependencies]
rand = "0.3"
hammer = { version = "0.5.0"}
color = { git = "https://github.com/bjz/color-rs" }
geometry = { path = "crates/geometry" }
```
