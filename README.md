# autotransfile
ATF

## 语义定义
* 通过多叉树记录目录结构，通过Map集合记录文件。简称文件集记录FSR
* 两个FSR之间差异为DV
* 目录标记为DM


## 核心功能
* 实现文件增删变化同步
  * client启动时获取server端的最新FSR，然后clien生成FSR并与server FSR对比，产生DV
  * client将DV发送到server
  * server解析DV，处理本地对应目录
* server端多线程处理不同目录
  * 可以配置多个处理目录
  * 
    
## 报文结构
### FSR


### DV
