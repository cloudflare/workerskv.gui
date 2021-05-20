extern crate redis;

const KEYLIST: &str = "##__INTERNAL::keylist";
const LASTSYNC: &str = "##__INTERNAL::lastsync";

pub type Connection = redis::Connection;

// TODO: user, password, ssl, ssh, etc
pub fn connect(host: String, port: u16, tls: bool) -> redis::Connection {
	let scheme = if tls { "rediss" } else { "redis" };

	// rediss?://[<username>][:<passwd>@]<hostname>[:port][/<db>]
	let addr = format!("{}://{}:{}", scheme, host, port);
	println!("~> connecting to {}", addr);

	let client = redis::Client::open(addr).expect("invalid connection string");
	client.get_connection().expect("failed to establish connection")
}

pub fn disconnect() {
	println!("~> disconnecting");
}

pub fn run<T: redis::FromRedisValue, A: redis::ToRedisArgs>(conn: &mut Connection, name: &str, args: A) -> redis::RedisResult<T> {
	redis::cmd(name).arg(args).query(conn)
}

pub fn keys(conn: &mut Connection) -> Vec<String> {
	redis::cmd("SMEMBERS").arg(KEYLIST).query(conn).expect(
		"unable to run 'SMEMBERS' for key list"
	)
}

pub struct Key {
	name: String,
	lastseen: String, // Date.now()
	expires: Option<String>, // Date.now()
	metadata: Option<String>, // JSON string
}

pub fn set(conn: &mut Connection, key: Key) {
	let keyname = key.name;
	let mut args: Vec<&str> = Vec::new();

	let _: () = redis::cmd("SADD").arg(&[KEYLIST, &keyname]).query(conn).expect(
		&format!("unable to add '{}' to key list", &keyname)
	);

	args.push(&keyname);

	args.push("lastseen");
	args.push(&key.lastseen);

	if let Some(expires) = &key.expires {
		args.push("expires");
		args.push(expires);
	}

	if let Some(metadata) = &key.metadata {
		args.push("metadata");
		args.push(metadata);
	}

	redis::cmd("HSET").arg(args).query(conn).expect(
		&format!("unable to run 'HSET {:?}' command", &keyname)
	)
}
