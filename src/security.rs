use actix_web::web::Data;
use itertools::Itertools;
use sea_orm::{
    ConnectionTrait, DatabaseBackend::MySql as Backend, DatabaseConnection, DbErr, Statement,
    Value, Values,
};

const IP_TABLE: &str = "iplog"; // time bigint, ip text
const BLOCKED_IP_TABLE: &str = "blocked_ip"; // ip text, cause text

pub async fn ratelimit(database: Data<DatabaseConnection>, cur_ip: String) -> i16 {
    let ctime: u64 = match std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH) {
        Err(_) => return 500,
        Ok(x) => x.as_secs(),
    };

    match database
        .get_ref()
        .query_all(Statement {
            sql: format!("select * from {} WHERE ip = ? ", BLOCKED_IP_TABLE),
            values: Some(Values(vec![Value::String(Some(Box::new(cur_ip.clone())))])),
            db_backend: Backend,
        })
        .await
    {
        Err(x) => println!("A error has occured while reading blocked ip's: {x}"),
        Ok(x) => {
            let reason: Vec<String> = x.iter().map(|y| y.try_get_by("cause").unwrap()).collect();
            if !reason.is_empty() {
                return 403;
            }
        }
    };

    let table: Vec<u64> = match database
        .query_all(Statement {
            sql: format!(
                "SELECT time, ip FROM {} WHERE ip = ? ORDER BY -time",
                IP_TABLE
            ),
            values: Some(Values(vec![Value::String(Some(Box::new(cur_ip.clone())))])),
            db_backend: Backend,
        })
        .await
    {
        Err(DbErr::Query(x)) => panic!("a querying error has occured: {x}"),
        Err(x) => todo!(
            "error '{x}' has occured in the ratelimit while fetching ip's, add a check for this"
        ),
        Ok(x) => {
            let rows: Vec<u64> = x
                .iter()
                .map(|result| result.try_get_by("time").unwrap())
                .collect();
            rows.iter().map(|time| ctime - time).collect()
        }
    };
    let mut is_safe: bool = true;
    if !table.is_empty() {
        let recent: Vec<(usize, u64)> = table
            .iter()
            .filter(|&row| *row < 60)
            .map(|x| x.to_owned())
            .dedup_with_count()
            .collect();
        for (amount, _) in recent {
            if amount > 7 {
                match database
                    .execute(Statement {
                        sql: format!("INSERT INTO {} VALUES ( ? , ? );", BLOCKED_IP_TABLE),
                        values: Some(Values(vec![
                            Value::String(Some(Box::new(String::from(&cur_ip)))),
                            Value::String(Some(Box::new(String::from(
                                "Multiple connections under a second",
                            )))),
                        ])),
                        db_backend: Backend,
                    })
                    .await
                {
                    Err(_) => panic!(
                        "{ctime}: FAILURE BLOCKING IP: {cur_ip}, ip connect {amount} times under a second"
                    ),
                    Ok(_) => {
                        is_safe = false;
                    }
                }
            }
        }
        return if is_safe { 200 } else { 429 };
    } else {
        return 200; // empty
    }
}
