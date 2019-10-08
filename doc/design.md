# pingchuan-mq 设计

## 协议设计
### pingchuan 消息格式
| 字段 | 是否必须 | 类型 | 可选参数 | 备注 |
| ---- | ---- | ---- |  ---- | ---- |
| pingchuan | 是 | 无 | pingchuan | 消息魔数，用于标记为该消息为 pingchuan 消息格式 |
| 消息ID | 是 | u32 | 无 | |
| topic_len | 是 | u32 | 0 | 必须大于等于0，建议为512字节 |
| content_len | 是 | u32 | 0 | 必须为大于等于0，建议为512K字节 |
| role | 是 | u32 | 0/1 | 0表示为发布者，1表示为订阅者 |
| order | 是 | u32 | 0.. | 0表示为起始地址，-1表示为终止符号 |
| gzip | 是 | u32 | 0/1 | 0表示为非压缩，1表示为压缩 | 
| crc | 是 | u32 | 无 | 用于校验整个消息的消息整体内容 |
| topic | 是 | String | 无 | 话题 |
| content | 是 | byte | 无 | 内容 |


### http 消息格式

### 二进制流消息格式

### 


## 相关资料
1.[MIT6.824 分布式系统](./distributed-systems/README.md)
2.[为 Kafka 写驱动](https://cwiki.apache.org/confluence/display/KAFKA/Writing+a+Driver+for+Kafka)
3.[日志系统 事务性](https://www.confluent.io/blog/transactions-apache-kafka/)
