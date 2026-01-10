# Arbitrum Rust 共学项目

## Task 1: Hello Web3 - 连接 Arbitrum Sepolia 测试网

### Part 1: MetaMask 配置

#### 1. 切换到 Arbitrum Sepolia 测试网
- 网络名称：Arbitrum Sepolia
- RPC URL：https://sepolia-rollup.arbitrum.io/rpc
- 链 ID：421614

![MetaMask 网络配置](task1/img/Metamask已切换网络至Arbitrum-Sepolia测试网.png)

#### 2. 领取测试币
通过 Alchemy 水龙头领取测试 ETH，并转入测试钱包。

![领水交易](task1/img/由另一个在主网持有ETH的钱包在alchemy领水后发送给测试钱包.png)

#### 3. 验证收到测试币
测试钱包成功收到 0.1 ETH。

![钱包余额](task1/img/测试钱包在Arbitrum-Sepolia测试网收到0.1ETH.png)

### Part 2: Hello Web3 程序

#### 1. 环境准备
- 代码路径：**/task1/hello-web3/**
- 安装 Rust 环境（版本 1.91.0）
- 创建新项目：`cargo new hello-web3`
- 引入依赖：ethers-rs、tokio、dotenv

#### 2. 程序功能
- 连接 Arbitrum Sepolia 测试网
- 获取链 ID、最新区块号、网络信息
- 显示区块详细信息

#### 3. 运行结果

程序成功连接并显示链上信息。

![程序运行截图](task1/img/task1程序成功运行截图.png)

---

## Task 2: 余额查询 - 查询 Arbitrum Sepolia 测试网地址余额

### 1. 环境准备
- 代码路径：**/level2-balance-query/**
- 创建新项目：`cargo new balance-query`
- 引入依赖：ethers-rs、tokio、dotenv

### 2. 程序功能
- 连接 Arbitrum Sepolia 测试网
- 查询指定地址的 ETH 余额
- 将余额从 wei 转换为 ETH 格式（1 ETH = 10^18 wei）
- 显示原始余额和格式化后的余额

### 3. 代码结构
- `src/main.rs` - 主程序入口，调用余额查询函数
- `src/balance.rs` - 余额查询模块，包含 `query_balance()` 函数
- `.env` - RPC 配置文件

### 4. 运行结果

程序成功查询地址 `0xd78677EFed3b87f8f421E68dA3F984ad8Ef76439` 的余额。

![程序运行截图](level2-balance-query/img/task2程序成果运行截图.png)

**查询结果：**
- 原始余额：99,999,564,586,000,000 wei
- 格式化余额：0.099999564586 ETH (约 0.1 ETH)

---

## Task 3
TODO
## Task 4
TODO
## Task 5
TODO