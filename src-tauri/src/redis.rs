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

/**
 * Get all the KV KEYS on file
 */
pub fn keys(conn: &mut Connection) -> Vec<String> {
	redis::cmd("SMEMBERS").arg(KEYLIST).query(conn).expect(
		"unable to run 'SMEMBERS' for key list"
	)
}

/**
 * Get the LASTSYNC value
 * AKA: A `sync` was completed
 */
pub fn lastsync(conn: &mut Connection) {
	redis::cmd("GET").arg(LASTSYNC).query(conn).expect("unable to retrieve lastsync value")
}

/**
 * Set the LASTSYNC value
 * AKA: A `sync` was completed
 */
pub fn timestamp(conn: &mut Connection, timestamp: &str) {
	redis::cmd("SET").arg(&[LASTSYNC, timestamp]).query(conn).expect("unable to update lastsync value")
}

pub struct Key {
	name: String,
	syncd: String, // Date.now()
	expires: Option<String>, // Date.now()
	metadata: Option<String>, // JSON string
	lastupdate: Option<String>, // Date.now()
}

/**
 * Save a new KV KEY to Redis
 * Adds the KEY to the KEYLIST set
 */
pub fn set(conn: &mut Connection, name: String, syncd: String, expires: Option<String>, metadata: Option<String>) {
	let mut args: Vec<&str> = Vec::new();

	let _: () = redis::cmd("SADD").arg(&[KEYLIST, &name]).query(conn).expect(
		&format!("unable to add '{}' to key list", &name)
	);

	args.push(&name);

	args.push("syncd");
	args.push(&syncd);

	if let Some(seconds) = &expires {
		args.push("expires");
		args.push(seconds);
	}

	if let Some(json) = &metadata {
		args.push("metadata");
		args.push(json);
	}

	redis::cmd("HSET").arg(args).query(conn).expect(
		&format!("unable to run 'HSET {:?}' command", &name)
	)
}

pub fn filter(conn: &mut Connection, pattern: String) -> Vec<String>  {
	let mut keys: Vec<String> = Vec::new();

	let iter: redis::Iter<String> = redis::cmd("SSCAN").arg(&[KEYLIST, "0", "MATCH", &pattern]).clone().iter(conn).expect(
		&format!("unable to run 'SSCAN MATCH {}' operation", &pattern)
	);

	for key in iter {
		keys.push(key);
	}

	return keys;
}

pub fn sort(conn: &mut Connection, to_desc: bool) -> Vec<String>  {
	let dir = if to_desc { "DESC" } else { "ASC" };
	redis::cmd("SORT").arg(&[KEYLIST, "ALPHA", dir]).query(conn).expect(
		&format!("unable to run 'SORT ALPHA {}' operation", &dir)
	)
}
