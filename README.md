### rust编写的一些交易工具集合
* crates/ctp-futures 期货CTP接口的封装
* crates/tora_stock 证券交易接口的封装
* crates/rust-share-util 公共的实用函数集合
* crates/examples 一些示例
* crates/spider 抓取一些财经网站的数据
* crates/gateway 交易网关, 完成订单执行，持仓调整
* app/log_pump 简单的消息推送

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

### gateway
* 运行网关
```
sh scripts/run_gateway.sh
```


* 网关处理账号登陆，重连，查询，下单的细节
* 网关默认提供一个http api供远程调用
* 网关提供一个持仓对齐功能，比如发送请求设定合约 SHFE:ru2409为3手多头持仓，那么post发送以下请求到网关即可:
```
curl -X POST \
  -H "Content-Type: application/json" \
  -d '{
	"broker_id": "9999",
	"account": "143650", 
  	target:{
		"exchange":"SHFE",
		"symbol": "ru2409",
		"position": 3, 
		"shift": 1,
		"close_priority": "YesterdayFirt"
	}, 
	credential:""}' \
  http://localhost:10111/api/set_contract_target

```
* 网关会自动订阅合约行情，根据最新报价偏移shift * price_tick作为发单价
* close_priority 用于指定优先平仓顺序，因为某些情况下平昨和平今的手续费不一样


### 交流

* 微信号 string_io