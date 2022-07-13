# app rocket

# 功能列表
- [] 全局配置

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

