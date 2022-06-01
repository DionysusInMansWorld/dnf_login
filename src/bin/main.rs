use std::io::{Stdin, Write, Stdout};
use std::{process::Command, os::windows::process::CommandExt};

use rsa::{self, RsaPrivateKey, pkcs1::DecodeRsaPrivateKey,};

use base64;

use hex;

use md5;

use mysql::{self, Pool, Opts, prelude::Queryable};
use mysql::{OptsBuilder, PooledConn, params};

fn main() {
    let private_key = get_private_key();
    let conn_pool = mysql_init();
    let mut conn = conn_pool.get_conn().expect("Failed to connect");

    let user = {
        let input = std::io::stdin();
        let mut out = std::io::stdout();
        let input = get_line(&input, &mut out, "登录输入L, 注册输入R, 其他字符退出: ")
            .expect("Stdin has a error");

        if input == "L" {
            get_user(&mut conn).expect("Login")
        } else if input == "R" {
            create_user(&mut conn).expect("Register")
        } else {
            return
        }
    };

    println!("{}", user.id);

    let data = format!("{:>08x}010101010101010101010101010101010101010101010101010101010101010155914510010403030101", user.id);
    let data = hex::decode(data).unwrap();

    let enc_data = private_key.sign(rsa::PaddingScheme::PKCS1v15Sign { hash: None }, &data)
        .unwrap();
    let enc_data = base64::encode(enc_data);

    Command::new("./dnf")
        .arg(enc_data)
        .creation_flags(0x08000000)
        .output()
        .expect("Failed to start");
}

fn get_private_key() -> RsaPrivateKey {
    let private_key = "-----BEGIN RSA PRIVATE KEY-----
MIIEpAIBAAKCAQEAuu7QIp5PjUtNGhudTjMH7wfO8ZzTj3qbzLBrGU6xayYp4AUS
6Av4/MmzVSijzexVQoaA3tQX8/DQJ4XleK9+mguJ3Lq0ITq/wB0T7FSkxeU2hASE
afXek5QJgBIEq3UHlaef70jPJ38CjNBgy0TzSJCsyjuQQMcZeMeoErbq2XOkisoL
/23CjelqzSTb/s2gK2xsgynzPkuhkyEhOtNyrMnbu03c6pyrcVtoUgwvnHXsyx6E
zXJjutAJhBH4Q14dLFof221+EkujwWERS1GH0ed+fyGrEZlUMReErb14w+33bKk6
1+mldqdh6G6fn84n8y5YF+cGI5zU8oSeg49NSQIDAQABAoIBAGuIXWrMru6U1rGi
GQeXC4VRdJZApOLwoRdKlRFl12HP/l7EDHA4Eu84CFWAn1oiDZnLTe7hCzZk3Rkf
STX3nlh3MsMrE9vZs9yL3Z4hwvekN4wSHSnnKjay/hQSSWVoWQiZ+MLpm2EZCxp9
9HB6JYkk0IE1anIZFmoIUIMTfl2/zxlHGlj6nlV0bfGpv1OlPcog1aD1TyvhqeBx
qbTurtgm+dQoqd1nZdQw9PeqGRgDtSf1Ay0YM3kJBz8J0tR9wyx3napVVopIYjF4
/8Qui6MEaT5uBjTgjFCsRSyPkwZqI5u4nXHx+ftAykvIz9Dn9pMZNF08s0I9wBsd
gamvrEECgYEA9XKhiDQpk5qr/zez6SbcuACEwUEfC/77ziphldNsG5x2bP/a7utM
xy7bbYbZ0KmrcnWSfxh1hECW33402idv4DoQMTu0w7zr2kccKxs3UcjcNTmuewVR
tujJWTFjxx3/t+zA9xze4xI8mJ/DsiWcJaKESGpLL9tgZwqJES62eM0CgYEAwvgs
gpksZPKD0+XlhTWCdQ9mPqAxTykIkYcJaPjoJJPwjNVYKVvl6VeBFd6tmU4FoNoU
82msHfflxoddLD6jrmLbBBh9X5amuvzeWt3zC4Q4+HkJAgQjwLFx2RI5X8vQP2fQ
MPi6/FHtpC1LbfDk2Ivuz7kCbH5/nJdo5nRtVm0CgYA5O9uy2QcA5kZJIwIO0gMR
3P0X20mUEIdDEdrjhwNkhN2QmTDCGZgzshd0uMc9wvK5o/TfMiLlDfKgdtt4K04J
KUDxWgzSv9D3ezF0U8pYhc/jkWnAQgNF9Y5OABhWLAafKtPsS40lwfDjXg3SErcQ
h471G+QgarVWEbzYht4B2QKBgQCbXGUjgGlG25U+wiA+IOCe5TGFT+NbAAiq5l8S
Pd3GX/i8ULka4/b2FNtxEOtmkSyc+4rcWGVl0AdSRsVxH00RBgceYWFuTT75G80X
vWsRz0ASh2gtKh1PTFa7MfF0K5X7IH9etqVRsPtb6xgDOIUzJXacIgITcE3B+0kE
8tu5lQKBgQCTKFn+VsX8uTpYz5O80G73V4eZVFwFs4JDa8gOwwrPEDUQLjrCg3Kw
uG9+Cj56scQ/Py+PTUDMYczVzCYHxbrwww9fUjqvgnqlRpn3EKzZSGXGMAV91oaJ
WaAy4jnu8GGIqyeXeqkZCl2ZnEShWbIf5pirpV42ywTmVOC0CEL6vg==
-----END RSA PRIVATE KEY-----";

    RsaPrivateKey::from_pkcs1_pem(private_key).unwrap()
}

fn mysql_init() -> Pool {
    let mut builder = OptsBuilder::new();
    builder = builder.user(Some("game"))
        .pass(Some("123456"))
        .ip_or_hostname(Some("192.168.200.131"))
        .tcp_port(3306);
    let opts = Opts::from(builder);

    Pool::new(opts).expect("connect")
}

fn login(username: &str, password: &str, conn: &mut PooledConn) -> Result<User, mysql::Error> {
    let row = conn.query_map(
        format!("select UID, accountname, password from d_taiwan.accounts where accountname='{}' and password='{}'", username, password),
        |r: (i64, String, String)| r
    )?;

    if row.len() == 0 {
        return Err(mysql::Error::IoError(std::io::Error::new(std::io::ErrorKind::Other, "no user")));
    }

    Ok(User { id: row[0].0, name: row[0].1.clone(), pass: row[0].2.clone() })
}

fn register(username: &str, password: &str, conn: &mut PooledConn) -> Result<User, mysql::Error> {
    conn.exec_iter(r"insert into d_taiwan.accounts (accountname, password)
        values (:username, :password)", params! {username, password})?;
    let user = login(username, password, conn)?;
    conn.exec_iter("insert into d_taiwan.limit_create_character (m_id) VALUES (:uid)", params! { "uid" => user.id })?;
    conn.exec_iter("insert into d_taiwan.member_info (m_id, user_id) VALUES (:uid, :uid)", params! { "uid" => user.id })?;
    conn.exec_iter("insert into d_taiwan.member_join_info (m_id) VALUES (:uid)", params! { "uid" => user.id })?;
    conn.exec_iter("insert into d_taiwan.member_miles (m_id) VALUES (:uid)", params! { "uid" => user.id })?;
    conn.exec_iter("insert into d_taiwan.member_white_account (m_id) VALUES (:uid)", params! { "uid" => user.id })?;
    conn.exec_iter("insert into taiwan_login.member_login (m_id) VALUES (:uid)", params! { "uid" => user.id })?;
    conn.exec_iter("insert into taiwan_login.member_game_option VALUES (:uid, 0x48000000789C63646064F85FCFCC90028408F0BF9E9181112C0382CC50B117CC20F114A038023042210009AC0C9B, '', '', 0x10020000789C636018058319686115D5C62AAA83555417ABA81E56517D06003C02010C);", params! { "uid" => user.id })?;
    conn.exec_iter("insert into taiwan_billing.cash_cera (account,cera,reg_date,mod_date) VALUES (:uid,1200,now(),now())", params! { "uid" => user.id })?;
    conn.exec_iter("insert into taiwan_billing.cash_cera_point (account,cera_point,reg_date,mod_date) VALUES (:uid,0,now(),now())", params! { "uid" => user.id })?;
    conn.exec_iter("insert into taiwan_login.member_play_info (occ_date,m_id,server_id) VALUES (now(),:uid,'1')", params! { "uid" => user.id })?;
    conn.exec_iter("insert into taiwan_login.allow_proxy_user(m_id) values(:uid)", params! { "uid" => user.id })?;

    Ok(user)
}

fn get_user(conn: &mut PooledConn) -> Result<User, String> {
    let input = std::io::stdin();
    let mut out = std::io::stdout();

    let username = get_line(&input, &mut out, "请输入账号").map_err(|e| format!("{:?}", e))?;
    let password = get_line(&input, &mut out, "请输入密码").map_err(|e| format!("{:?}", e))?;
    let password = format!("{:x}", md5::compute(password));

    let user = login(&username, &password, conn).map_err(|e| format!("{:?}", e))?;

    Ok(user)
}

fn create_user(conn: &mut PooledConn) -> Result<User, String> {
    let input = std::io::stdin();
    let mut out = std::io::stdout();

    let username = get_line(&input, &mut out, "请输入账号").map_err(|e| format!("{:?}", e))?;
    let password = get_line(&input, &mut out, "请输入密码").map_err(|e| format!("{:?}", e))?;
    let password = format!("{:x}", md5::compute(password));

    if login(&username, &password, conn).is_ok() {
        return Err("已注册.".to_string());
    }

    let user = register(&username, &password, conn).map_err(|e| format!("{:?}", e))?;

    Ok(user)
}

fn get_line(input: &Stdin, out: &mut Stdout, msg: &str) -> Result<String, std::io::Error>{
    print!("{msg}: ");
    out.flush().expect("flush");

    let mut line = String::new();
    input.read_line(&mut line)?;
    Ok(line.trim().to_string())
}

struct User {
    id: i64,
    name: String,
    pass: String,
}