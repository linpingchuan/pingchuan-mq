# pingchuan-mq 设计

## 协议设计
### pingchuan 消息格式
| 字段 | 是否必须 | 类型 | 可选参数 | 备注 |
| ---- | ---- | ---- |  ---- | ---- |
| pingchuan | 是 | 无 | pingchuan | 无 |
| len | 是 | u32 | 0 | 必须为大于等于0，建议为512K字节 |
| gzip | 是 | u32 | 0/1 | 0表示为非压缩，1表示为压缩 | 
| content | 是 | byte | 无 | 无 |

### http 消息格式

### 二进制流消息格式

### 


## 相关资料
1.[MIT6.824 分布式系统](https://github.com/feixiao/Distributed-Systems)
