use crate::dbs;
use crate::dbs::Executor;
use crate::dbs::Options;
use crate::dbs::Runtime;
use crate::err::Error;
use crate::sql::comment::shouldbespace;
use crate::sql::value::{value, Value};
use nom::bytes::complete::tag_no_case;
use nom::combinator::opt;
use nom::multi::separated_list0;
use nom::IResult;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct IfelseStatement {
	pub exprs: Vec<(Value, Value)>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub close: Option<Value>,
}

impl fmt::Display for IfelseStatement {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(
			f,
			"{}",
			self.exprs
				.iter()
				.map(|(ref cond, ref then)| format!("IF {} THEN {}", cond, then))
				.collect::<Vec<_>>()
				.join(" ELSE ")
		)?;
		if let Some(ref v) = self.close {
			write!(f, " ELSE {}", v)?
		}
		write!(f, " END")?;
		Ok(())
	}
}

impl dbs::Process for IfelseStatement {
	fn process(
		&self,
		ctx: &Runtime,
		opt: &Options,
		exe: &mut Executor,
		doc: Option<&Value>,
	) -> Result<Value, Error> {
		for (ref cond, ref then) in &self.exprs {
			let v = cond.process(ctx, opt, exe, doc)?;
			if v.is_truthy() {
				return then.process(ctx, opt, exe, doc);
			}
		}
		match self.close {
			Some(ref v) => v.process(ctx, opt, exe, doc),
			None => Ok(Value::None),
		}
	}
}

pub fn ifelse(i: &str) -> IResult<&str, IfelseStatement> {
	let (i, exprs) = separated_list0(split, exprs)(i)?;
	let (i, close) = opt(close)(i)?;
	let (i, _) = shouldbespace(i)?;
	let (i, _) = tag_no_case("END")(i)?;
	Ok((
		i,
		IfelseStatement {
			exprs,
			close,
		},
	))
}

fn exprs(i: &str) -> IResult<&str, (Value, Value)> {
	let (i, _) = tag_no_case("IF")(i)?;
	let (i, _) = shouldbespace(i)?;
	let (i, cond) = value(i)?;
	let (i, _) = shouldbespace(i)?;
	let (i, _) = tag_no_case("THEN")(i)?;
	let (i, _) = shouldbespace(i)?;
	let (i, then) = value(i)?;
	Ok((i, (cond, then)))
}

fn close(i: &str) -> IResult<&str, Value> {
	let (i, _) = shouldbespace(i)?;
	let (i, _) = tag_no_case("ELSE")(i)?;
	let (i, _) = shouldbespace(i)?;
	let (i, then) = value(i)?;
	Ok((i, then))
}

fn split(i: &str) -> IResult<&str, ()> {
	let (i, _) = shouldbespace(i)?;
	let (i, _) = tag_no_case("ELSE")(i)?;
	let (i, _) = shouldbespace(i)?;
	Ok((i, ()))
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn ifelse_statement_first() {
		let sql = "IF this THEN that END";
		let res = ifelse(sql);
		assert!(res.is_ok());
		let out = res.unwrap().1;
		assert_eq!(sql, format!("{}", out))
	}

	#[test]
	fn ifelse_statement_close() {
		let sql = "IF this THEN that ELSE that END";
		let res = ifelse(sql);
		assert!(res.is_ok());
		let out = res.unwrap().1;
		assert_eq!(sql, format!("{}", out))
	}

	#[test]
	fn ifelse_statement_other() {
		let sql = "IF this THEN that ELSE IF this THEN that END";
		let res = ifelse(sql);
		assert!(res.is_ok());
		let out = res.unwrap().1;
		assert_eq!(sql, format!("{}", out))
	}

	#[test]
	fn ifelse_statement_other_close() {
		let sql = "IF this THEN that ELSE IF this THEN that ELSE that END";
		let res = ifelse(sql);
		assert!(res.is_ok());
		let out = res.unwrap().1;
		assert_eq!(sql, format!("{}", out))
	}
}
