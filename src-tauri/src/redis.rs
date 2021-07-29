extern crate redis;

use std::collections::HashMap;

const KEYLIST: &str = "##__INTERNAL::keylist";
const NAMESPACES: &str = "##__INTERNAL::namespaces";
const LASTSYNC: &str = "##__INTERNAL::lastsync";

pub type Connection = redis::Connection;

// TODO: ssl, ssh, etc
pub fn connect(host: String, port: u16, tls: bool, password: Option<String>, user: Option<String>) -> redis::Connection {
	let scheme = if tls { "rediss" } else { "redis" };

	let mut prefix = "".to_owned();

	if let Some(user) = &user {
		prefix.push_str(&user);
	}

	if let Some(password) = &password {
		if prefix.len() > 0 {
			prefix.push_str(":");
		}
		prefix.push_str(&password);
	}

	if prefix.len() > 0 {
		prefix.push_str("@");
	}

	// rediss?://[<username>][:<passwd>@]<hostname>[:port][/<db>]
	let addr = format!("{}://{}{}:{}", scheme, prefix, host, port);
	println!("~> connecting to {}", addr);

	let client = redis::Client::open(addr).expect("invalid connection string");
	client.get_connection().expect("failed to establish connection")
}

pub fn disconnect() {
	println!("~> disconnecting");
}

pub fn select(conn: &mut Connection, namespaceid: String) -> String {
	let iter: redis::Iter<(String, String)> = redis::cmd("HSCAN").arg(&[&NAMESPACES, "0"]).clone().iter(conn).expect(
		&format!("unable to lookup existing namespaces")
	);

	let hash: HashMap<String, String> = iter.collect();

	if let Some(ident) = hash.get(&namespaceid) {
		redis::cmd("SELECT").arg(ident).query(conn).expect(
			&format!("unable to 'SELECT {}' database", &ident)
		)
	} else {
		// redis databases are 0-based
		let nextid = hash.len().to_string();
		let _: () = redis::cmd("HSET").arg(&[NAMESPACES, &namespaceid, &nextid]).query(conn).expect(
			"unable to update namespaces list"
		);

		redis::cmd("SELECT").arg(nextid).query(conn).expect(
			&format!("unable to 'SELECT {}' database", hash.len().to_string())
		)
	}
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
pub fn lastsync(conn: &mut Connection) -> String {
	redis::cmd("GET").arg(LASTSYNC).query(conn).unwrap_or_default()
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
	lastupdate: Option<String>, // value touched
	mimetype: Option<String>, // how to render
	value: Option<String>, // <any>
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
	let iter: redis::Iter<String> = redis::cmd("SSCAN").arg(KEYLIST).cursor_arg(0).arg("MATCH").arg(&pattern)
		.clone().iter(conn).expect(
			&format!("unable to run 'KEYS {}' operation", &pattern)
		);

	iter.collect()
}

pub fn sort(conn: &mut Connection, to_desc: bool) -> Vec<String>  {
	let dir = if to_desc { "DESC" } else { "ASC" };
	redis::cmd("SORT").arg(&[KEYLIST, "ALPHA", dir]).query(conn).expect(
		&format!("unable to run 'SORT ALPHA {}' operation", &dir)
	)
}

pub fn details(conn: &mut Connection, key: String) -> HashMap<String, String>  {
	let iter: redis::Iter<(String, String)> = redis::cmd("HSCAN").arg(&[&key, "0"]).clone().iter(conn).expect(
		&format!("unable to run 'HSCAN {}' operation", key)
	);

	iter.collect()
}

pub fn value(conn: &mut Connection, key: String, value: String, timestamp: String, mimetype: Option<String>) {
	let mut args: Vec<&str> = Vec::new();

	args.push(&key);

	args.push("value");
	args.push(&value);

	args.push("lastupdate");
	args.push(&timestamp);

	if let Some(known) = &mimetype {
		args.push("mimetype");
		args.push(known);
	}

	redis::cmd("HSET").arg(args).query(conn).expect(
		&format!("unable to run 'HSET {:?}' command", &key)
	)
}
