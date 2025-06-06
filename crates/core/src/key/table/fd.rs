//! Stores a DEFINE FIELD config definition
use crate::key::category::Categorise;
use crate::key::category::Category;
use crate::kvs::{KeyEncode, impl_key};
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Fd<'a> {
	__: u8,
	_a: u8,
	pub ns: &'a str,
	_b: u8,
	pub db: &'a str,
	_c: u8,
	pub tb: &'a str,
	_d: u8,
	_e: u8,
	_f: u8,
	pub fd: &'a str,
}
impl_key!(Fd<'a>);

pub fn new<'a>(ns: &'a str, db: &'a str, tb: &'a str, fd: &'a str) -> Fd<'a> {
	Fd::new(ns, db, tb, fd)
}

pub fn prefix(ns: &str, db: &str, tb: &str) -> Result<Vec<u8>> {
	let mut k = super::all::new(ns, db, tb).encode()?;
	k.extend_from_slice(b"!fd\x00");
	Ok(k)
}

pub fn suffix(ns: &str, db: &str, tb: &str) -> Result<Vec<u8>> {
	let mut k = super::all::new(ns, db, tb).encode()?;
	k.extend_from_slice(b"!fd\xff");
	Ok(k)
}

impl Categorise for Fd<'_> {
	fn categorise(&self) -> Category {
		Category::TableField
	}
}

impl<'a> Fd<'a> {
	pub fn new(ns: &'a str, db: &'a str, tb: &'a str, fd: &'a str) -> Self {
		Self {
			__: b'/',
			_a: b'*',
			ns,
			_b: b'*',
			db,
			_c: b'*',
			tb,
			_d: b'!',
			_e: b'f',
			_f: b'd',
			fd,
		}
	}
}

#[cfg(test)]
mod tests {
	use crate::kvs::KeyDecode;
	#[test]
	fn key() {
		use super::*;
		#[rustfmt::skip]
		let val = Fd::new(
			"testns",
			"testdb",
			"testtb",
			"testfd",
		);
		let enc = Fd::encode(&val).unwrap();
		assert_eq!(enc, b"/*testns\x00*testdb\x00*testtb\x00!fdtestfd\x00");

		let dec = Fd::decode(&enc).unwrap();
		assert_eq!(val, dec);
	}

	#[test]
	fn test_prefix() {
		let val = super::prefix("testns", "testdb", "testtb").unwrap();
		assert_eq!(val, b"/*testns\0*testdb\0*testtb\0!fd\0");
	}

	#[test]
	fn test_suffix() {
		let val = super::suffix("testns", "testdb", "testtb").unwrap();
		assert_eq!(val, b"/*testns\0*testdb\0*testtb\0!fd\xff");
	}
}
