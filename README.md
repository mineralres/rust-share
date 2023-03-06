### rust编写的一些交易工具集合
* crates/ctp-futures 期货CTP接口的封装
* crates/rust-share-util 公共的实用函数集合

### 说明
* 编译过程中会用到clang, 需要手动下载安装对应的依赖, [下载](https://github.com/llvm/llvm-project/releases)

### 编译
```
cargo build
```

### 示例
```
cargo run --example ctp-query
```