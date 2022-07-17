# app rocket
使用 rust 语言基于 rocket 框架搭建的一个静态 WEB 服务, 并将资源文件内嵌至二进制文件中。

## 功能列表
- [ ] 全局配置
- [ ] mysql 数据库, 支持简单的增删改查
- [ ] rust-embed, 内嵌静态web页面访问/内嵌静态资源文件
- [x] 多环境
- [ ] 用户注册
- [ ] 用户登录 - 返回 token
  - [ ] token 获取用户信息
  - [x] 用户密码加密/解密
  - [ ] token 访问 API
- [x] api token 对外授权


## 项目运行
- 前端
```shell
# 安装依赖
yarn install

# 编译
yarn build

# 运行前端
yarn run dev
```
- 后端
```shell
# debug 模式运行
cargo run

# debug 模式编译
cargo build

# release 模式编译
cargo run --release

# release 模式编译
cargo build --release
```


## 数据库初始化
- 创建数据库

```mysql
CREATE DATABASE  `rocket` DEFAULT CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;
```


## diesel 使用
- 安装cli
```shell
# 前置依赖安装 debain
sudo apt install default-libmysqlclient-dev libsqlite3-dev libpostgresql-ocaml-dev

cargo install diesel_cli
```
- 添加diesel 依赖
```
Cargo.toml
 
[dependencies]
diesel = { version = "1.4.8", features = ["mysql", "chrono"] }
dotenv = "0.15.0"
```
- 添加数据库环境变量, 项目使用dotenv管理环境变量
```
vim .env

DATABASE_URL=mysql://xxx:xxx@localhost/rocket
# DATABASE_URL=postgres://postgres:dalong@localhost/diesel_demo
```
- 创建数据库
```
diesel setup
```
- 创建一个实体
```
diesel migration generate create_user
```
- 添加migration 脚本
```
vim up.sql

CREATE TABLE user
(
    `id`       INT AUTO_INCREMENT COMMENT '用户ID',
    `name`     VARCHAR(32)  NOT NULL COMMENT '姓名',
    `gender`   TINYINT(1)   NOT NULL DEFAULT 1 COMMENT '性别: 0:女,1:男',
    `age`      INT(11)      NOT NULL COMMENT '年龄',
    `birth`    VARCHAR(20)  NULL COMMENT '出生日期',
    `phone`    VARCHAR(20)  NOT NULL UNIQUE COMMENT '手机号码',
    `email`    VARCHAR(50)  NULL COMMENT '邮件',
    `password` VARCHAR(20)  NOT NULL DEFAULT '888888' COMMENT '密码',
    `address`  VARCHAR(200) NULL COMMENT '居住地址',
    `avatar`   VARCHAR(20)  NULL COMMENT '头像',
    `status`   TINYINT(1)   NULL     DEFAULT 1 COMMENT '是否启用,0:禁用,1:启用',
    `created`  DATETIME     NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    `updated`  DATETIME     NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
    PRIMARY KEY (`id`)
) ENGINE = InnoDB
  DEFAULT CHARACTER SET = utf8mb4
  COLLATE = utf8mb4_unicode_ci
    COMMENT '用户';
```
```
down.sql

DROP TABLE user;
```
- schema migration


## 项目引用
- [vue3-admin-plus](https://github.com/jzfai/vue3-admin-plus)
- [RealWorld](https://github.com/gothinkster/realworld)
- [rocket+diesel+mysql学习](https://www.jianshu.com/p/95452dbe343b)
- [rust-embed](https://github.com/pyrossh/rust-embed)
