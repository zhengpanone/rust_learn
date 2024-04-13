# actix-web-async-graphql

## 创建项目

```shell
shellcargo new actix-web-async-graphql --vcs none
cargo new backend --bin
cargo new frontend-handlebars --bin
cargo new frontend-wasm --bin
```

### 编辑toml

```toml
[package]
name = "actix-web-async-graphql"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[workspace]
members = [
    "./backend",
    "./frontend-handlebars",
    "./frontend-wasm"
]

resolver = "2"
[profile.dev]
split-debuginfo="unpacked"

[dependencies]

```

> **注1**：`resolver = "2"` 是 Rust 1.51.0 中 cargo 工具新支持的设定项，主要用于解决依赖项管理的难题。目前，Cargo 的默认行为是：在依赖关系图中，当单个包被多次引用时，合并该包的特性。而启用 `resolver` 后，则可避免合并包，启用所有特性。但这种新的解析特性，可能会导致一些 crate 编译不止一次。具体参阅 [Rust 1.51.0 已正式发布，及其新特性详述](http://mp.weixin.qq.com/s?__biz=MzIwNzkyMDE0NA==&mid=2247484240&idx=2&sn=3f3cdf5c790c9b9bc0aa5e0d3ccb6d12&chksm=970a4e2fa07dc739e734a4eeb231f3fd791d0161fcbbb8d755f78dbf4b93e7de14e101bf4c15&scene=21#wechat_redirect)，或者 [Rust 2021 版本特性预览，以及工作计划](http://mp.weixin.qq.com/s?__biz=MzIwNzkyMDE0NA==&mid=2247484435&idx=1&sn=6a5227d45e7265599e41f47ca3ab885f&chksm=970a496ca07dc07a2b4fd73f0dfef0adf041f64d595498c6c6be5710a82bddbc1ce7922bfb52&scene=21#wechat_redirect) 中的 `Cargo resolver` 小节。

> **注2**：`[profile.dev]` 是 Rust 1.51.0 中的关于“拆分调试信息”的设定，这个主要是 macOS 上的改进。

### 创建项目

```shell
cargo new backend --bin --vcs none
cargo new frontend-handlebars --bin --vcs none
cargo new frontend-wasm --bin --vcs none
```



### 安装工具链

```shell
cargo install cargo-edit
cargo install cargo-watch

# cargo watch 使用
cargo watch -x "run"

```

- cargo-edit，包含 `cargo add`、`cargo rm`，以及 `cargo upgrade`，可以让我们方便地管理 crate。
- cargo-watch，监视项目的源代码，以了解其更改，并在源代码发生更改时，运行 Cargo 命令。

### 添加依赖create

```shell
cd backend
cargo add actix-web async-graphql rbatis
```

### 依赖项支持特性（features）


https://mp.weixin.qq.com/s/VRVKaRYRLXT2cS0cFJQUpw

https://mp.weixin.qq.com/s?__biz=MzIwNzkyMDE0NA==&mid=2247484501&idx=2&sn=0a678fb4a016c163d468d0a1dba32bac&chksm=970a492aa07dc03ca7d10b5d11d259e9887d987eed55b985baf1d4efa20026b697deb9e61ea1&scene=178&cur_album_id=1825691062082813955#rd

