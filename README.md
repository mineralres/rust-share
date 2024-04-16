### rust编写的一些交易工具集合
* crates/ctp-futures 期货CTP接口的封装
* crates/tora_stock 证券交易接口的封装
* crates/rust-share-util 公共的实用函数集合
* crates/examples 一些示例
* crates/spider 抓取一些财经网站的数据
* crates/gateway 交易网关, 完成订单执行，持仓调整

### 说明
* 编译过程中会用到clang(15.0.7), 需要手动下载安装对应的依赖, [下载](https://github.com/llvm/llvm-project/releases/tag/llvmorg-15.0.7)
* 仅在ubuntu 20.04 和 windows11 上编译测试过

### 编译
```
cargo build
```

### 示例
```
cargo run --example ctp-query
```

### 交流

* 微信号 string_io