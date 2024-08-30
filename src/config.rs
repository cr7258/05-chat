use std::{env, fs::File};

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerConfig {
    pub port: u16,
}

impl AppConfig {
    pub fn load() -> Result<Self> {
        // read from ./app.yml or /etc/config/app.yml, or from env CHAT_CONFIG
        let ret = match (
            File::open("app.yml"),
            File::open("/etc/config/app.yml"),
            env::var("CHAT_CONFIG"),
        ) {
            // 使用了一个 match 语句，根据以上三种可能性的返回结果来决定读取哪个配置文件：
            // 如果 File::open("app.yml") 成功 (Ok(reader))，则使用该文件的 reader 进行读取
            (Ok(reader), _, _) => serde_yaml::from_reader(reader),
            // 如果第一个文件不存在，但 /etc/config/app.yml 文件存在，则使用第二个文件的 reader 进行读取
            (_, Ok(reader), _) => serde_yaml::from_reader(reader),
            // 如果两个文件都不存在，但 CHAT_CONFIG 环境变量存在并指向一个文件路径，则尝试打开此路径并使用它
            (_, _, Ok(path)) => serde_yaml::from_reader(File::open(path)?),
            // 如果以上三种情况都失败，代码将返回一个错误
            _ => bail!("Config file not found"),
        };
        Ok(ret?)
    }
}