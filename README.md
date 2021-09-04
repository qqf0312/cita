## 项目介绍

本项目扩展了CITA区块链系统，其核心扩展功能为：

1. 智能合约两阶段并发执行

   ​		本项目针对通用的“先共识-后执行”智能合约执行范式，应用了一种兼顾主节点执行和验证节点回放的并发执行协议——2PX 协议。在执行阶段的主阶段生成含有每一笔交易的一致性读集的交易依赖图，并且在第二阶段——回放阶段时根据交易依赖图无回滚地回放每一笔交易。

2. 基于纠删码的系统

   ​		基于纠删码技术将全部区块数据编码成多个数据块和冗余块，每个节点仅保留部分数据块和冗余块，全网节点可组合出完整的编码块信息，使得各个节点在减少存储空间占用的同时，又确保了原始区块数据不丢失。

## 安装步骤

#### CITA环境配置与部署（Ubuntu18.04）

**1.安装Rust** 
`sudo apt-get install curl` 
`curl https://sh.rustup.rs -sSf | sh`  
`sudo apt-get install rustc`  
检查rustc安装是否成功  
`rustc --version`  
**2.下载项目源码**  
`git clone https://github.com/qqf0312/cita.git`  
`cd cita`  
`git submodule init`  

>遇到Submodule 'cita-bft'(...)registered for path 'cita-bft'报错时：  
>`git submodule sync`   
>`git submodule update --init --recursive`

`git submodule update`  
**3.编译**  
调试模式编译`./env.sh make debug`  

>遇到docker find no command报错  
>`sudo apt install docker.io`  
>再次编译遇到get permission denied报错  
>`sudo groupadd docker` #添加docker用户组  
>`sudo gpasswd -a 用户名 docker`#用户名为当前用户用户名  
>`newgrp docker`  
>`docker ps`  

*发行模式编译`./env.sh make release`* #未测试  

**4.配置环境**  
`cita-cli key create`  

>若cita-cli命令不存在，下载cita-cli安装包  
>https://docs.citahub.com/zh-CN/cita/getting-started/setup 下载在cita文件夹中  

进入编译好的结果目录cita/target/install，初始化链  
`bin/cita create --super_admin "0x37d1c7449bfe76fe9c445e626da06265e9377601" --nodes "127.0.0.1:4000,127.0.0.1:4001,127.0.0.1:4002,127.0.0.1:4003" --chain_name "test-chain01"`

**5.启动系统**  
第一次启动系统时需要使用 setup 命令初始化每个节点，之后重新启动不需要再次操作。(也在install下操作)  
`bin/cita setup test-chain01/0` #test-chain01/节点编号  
`bin/cita start test-chain01/0`  
启动0-3四个节点后，检查节点是否运行正常  
`bin/cita top test-chain01/0` #test-chain01/节点编号 
通过端口localhost:15672访问rabbitmq通信可视化

#### CITA在vscode中运行问题

**cargo build error:failed to run custom build command for 'openssl-sys v0.9.39'**  
`sudo apt install libssl-dev`安装最新的libssl包
`sudo rm -rf Cargo.lock target/`清除之前build failed的文件

## 实验测试

1. 智能合约两阶段并发执行

   ​		在正确性测试方面，基于图生成算法所生成的冲突图和交易依赖图以及交易依赖图传递的信息，对所有交易进行回放执行。同时与串行执行结果进行对比，验证交易了并发是否正确。 

   ​		在性能测试方面，在验证回放阶段分别让串行和并发分别执行，分别得出处 理相同交易所需要的时间，并计算执行效率的比值。在单客户端每区块交易数为300、并发线程数为 4 的设置下，低冲突率时，并行方案比串行方案约高3倍； 在中冲突率时，并行方案效率比串行方案效率约高 2 倍。

2. 基于纠删码的区块链存储系统

   ​		功能测试部分主要围绕本地查询测试和多节点查询测试这两个主要的测试 方向进行。本地查询测试主要验证基于纠删码的数据存储功能。多节点查询测试 主要验证纠删码节点间通信功能，基于纠删码的数据恢复功能，以及动态加入节 点（扩容）和删除节点（缩容）引发的数据同步功能。 

   ​		项目通过查询非本地区块，对节点间的通信做了具体测试。通过将节点本 地数据库中的数据块删除以模拟数据丢失、暂停节点以模拟节点掉线，对纠删码 的解码恢复功能进行测试。通过添加和删除节点来模拟实际应用中“扩容”和“缩容”，测试了系统的数据自动同步。

   ​		性能方面的测试主要围绕加入纠删码后系统的存储消耗，查询延迟时间，吞吐量以及网络消耗展开。与原来每个块的总体存储消耗为O(n)相比，本系统将每个块的存储消耗降低到 O(1)。

   ​		初步性能测试表明，产生50 个区块时基于纠删码的系统数据库文件大小为44.6KB，原系统的数据库文件大小为96.7KB，存储消耗降低了约 0.5 倍。

#### 测试报告

[智能合约两阶段并发执行测试报告](./test_report/occ.md)

[基于纠删码的CITA区块链存储系统测试报告](./test_report/ec1.md)



