use mysql::{OptsBuilder, Pool, prelude::*, PooledConn};
use user::User;

fn main() {
    let config = Config::new("game".to_string(),
        "123456".to_string(),
        "192.168.200.131".to_string(),
        3306);

    let conn_pool = mysql_init(&config)
        .expect("mysql error");
    let mut conn = conn_pool.get_conn()
        .expect("failed to connect, error");

    let user = User::by_id(0, &mut conn)
        .expect("mysql error");
    if let Some(user) = user {
        println!("{}", user.acccountname);
    } else {
        println!("There is no user who with the id");
    }

    let users = User::all(&mut conn)
        .expect("mysql error");
    for u in users {
        println!("{}", u.uid.unwrap());
    }
}

struct Config {
    user: String,
    pass: String,
    addr: String,
    port: u16,
}

impl Config {
    fn new(user: String, pass: String, addr: String, port: u16)-> Self {
        Self { user, pass, addr, port }
    }
}

fn mysql_init(config: &Config) -> Result<Pool, mysql::Error> {
    let opts = OptsBuilder::new()
        .user(Some(&config.user))
        .pass(Some(&config.pass))
        .ip_or_hostname(Some(&config.addr))
        .tcp_port(config.port);
    
    Pool::new(opts)
}

mod user {
    use mysql::{prelude::*, PooledConn};

    use crate::Select;

    type Row = (i64, String, String);

    #[derive(Clone)]
    pub struct User {
        pub uid: Option<i64>,
        pub acccountname: String,
        pub password: Option<String>,
        pub password_encrypted: Option<String>,
    }

    impl Select<Row> for User {
        fn select() -> String {
            String::from("SELECT uid, accountname, password FROM d_taiwan.accounts ")
        }

        fn query_row(stmt: &str, conn: &mut PooledConn) -> Result<Vec<Row>, mysql::Error> {
            Ok(conn.query_map(
                stmt,
                |r: Row| r)?)
        }

        fn from_row(row: Vec<Row>) -> Vec<Self> {
            let mut users = Vec::new();
            if row.len() != 0 {
                for r in row {
                    users.push(Self {
                        uid: Some(r.0),
                        acccountname: r.1,
                        password: None,
                        password_encrypted: Some(r.2)
                    })
                }
            }

            users
        }
    }
}

pub trait Select<R>
    where Self: Sized + Clone {
    fn select() -> String;

    fn query_row(stmt: &str, conn: &mut PooledConn) -> Result<Vec<R>, mysql::Error>;

    fn from_row(row: Vec<R>) -> Vec<Self>;

    fn all(conn: &mut PooledConn) -> Result<Vec<Self>, mysql::Error> {
        let row = Self::query_row(&Self::select(), conn)?;
            
        Ok(Self::from_row(row))
    }

    fn by_where(cond: &str, conn: &mut PooledConn) -> Result<Vec<Self>, mysql::Error> {
        let mut stmt = Self::select() + "where ";
        stmt.push_str(cond);
        let row = Self::query_row(&stmt, conn)?;

        Ok(Self::from_row(row))
    }

    fn by_id(id: i64, conn: &mut PooledConn) -> Result<Option<Self>, mysql::Error> {
        let targets = Self::by_where(&format!("uid='{}'", id), conn)?;

        if targets.len() == 0 {
            Ok(None)
        } else {
            Ok(Some(targets[0].clone()))
        }
    }
}

pub trait Insert {

}

pub trait Update {

}

pub trait Delete {

}

pub trait Save {

}