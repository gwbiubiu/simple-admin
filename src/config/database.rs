use super::Mysql;
use mysql::prelude::*;
use mysql::*;

use anyhow::Result;

fn init_mysql(mysql_config: Mysql) -> Result<Pool> {
    let opts = OptsBuilder::new()
        .user(Some(mysql_config.user))
        .pass(Some(mysql_config.password))
        .ip_or_hostname(Some(mysql_config.host))
        .tcp_port(mysql_config.port as u16)
        .db_name(Some(mysql_config.database));

    let pool = Pool::new(opts)?;
    let mut conn = pool.get_conn()?;
    conn.query_drop("SELECT 1")?;
    Ok(pool)
}
