extern crate redis;

// pub type Connection = redis::Connection;

// TODO: user, password, ssl, ssh, etc
pub fn connect(host: String, port: u16, tls: bool) -> redis::Connection {
	let scheme = if tls { "rediss" } else { "redis" };
	// rediss?://[<username>][:<passwd>@]<hostname>[:port][/<db>]
	let addr = format!("{}://{}:{}", scheme, host, port);
	println!("~> connecting to {}", addr);

	let client = redis::Client::open(addr).expect("invalid connection string");
	client.get_connection().expect("failed to establish connection")
}

pub fn run<T: redis::FromRedisValue, A: redis::ToRedisArgs>(conn: &mut dyn redis::ConnectionLike, name: &str, args: A) -> redis::RedisResult<T> {
	redis::cmd(name).arg(args).query(conn)
}

// pub fn list(conn: &mut redis::Connection) -> redis::RedisResult<usize> {
// 	redis::cmd("MEMORY").arg("USAGE").arg("hello").query(conn)
// }
