### 模块
* crates/ctp-futures 期货CTP接口的封装
* crates/rust-share-util 公共的实用函数集合
* crates/examples 一些示例
* crates/spider 抓取一些财经网站的数据
* crates/gateway 订单执行网关

## 网关功能

网关提供一些预设策略用于完成订单执行任务

### 持仓较准
* 如 set("SHFE", "ru2409", 7, 3), 会将SHFE:ru2409 调整至 7 手多头持仓, 3表示根据当前ask-bid + 3个price_tick下单