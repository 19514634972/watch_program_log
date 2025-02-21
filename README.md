## 项目背景
日志收集通常是把服务器上的日志实时收集,存储到中心系统,再对这些日志建立索引,通过搜索即可快速找到对应的日志记录,
通过提提供一个界面友好的web页实现日志检索与展示,通常与之对应的解决方案是ELK,即Elasticsearch+Logstash+Kibana,
是一个开源的日志分析平台,它可以实时地收集、存储、分析和可视化日志数据。但是ELK的安装和配置比较复杂,而且需要消耗大
量的系统资源,对于小型项目来说,可能会显得过于庞大和复杂。
因此我们设计了一个新的解决方案：

![流程图](https://github.com/user-attachments/assets/65e5fc72-a0fa-4eec-ba0f-e897f85de945)

## 设计思路
上图橙黄色区域是系统需要开发实现的部分,log Agent文件读取日志,发送到kafka,transfer转发到es,实现日志可视化查询
用到的技术点有kafka,zookeeper,elasticsearch,kibana,etcd,influxdb等。
