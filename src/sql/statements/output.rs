use crate::dbs;
use crate::dbs::Executor;
use crate::dbs::Options;
use crate::dbs::Runtime;
use crate::err::Error;
use crate::sql::comment::shouldbespace;
use crate::sql::value::{value, Value};
use nom::bytes::complete::tag_no_case;
use nom::IResult;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct OutputStatement {
	pub what: Value,
}

impl fmt::Display for OutputStatement {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "RETURN {}", self.what)
	}
}

impl dbs::Process for OutputStatement {
	fn process(
		&self,
		ctx: &Runtime,
		opt: &Options,
		exe: &mut Executor,
		doc: Option<&Value>,
	) -> Result<Value, Error> {
		// Ensure futures are processed
		let opt = &opt.futures(true);
		// Process the output value
		self.what.process(ctx, opt, exe, doc)
	}
}

pub fn output(i: &str) -> IResult<&str, OutputStatement> {
	let (i, _) = tag_no_case("RETURN")(i)?;
	let (i, _) = shouldbespace(i)?;
	let (i, v) = value(i)?;
	Ok((
		i,
		OutputStatement {
			what: v,
		},
	))
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn output_statement() {
		let sql = "RETURN $param";
		let res = output(sql);
		assert!(res.is_ok());
		let out = res.unwrap().1;
		assert_eq!("RETURN $param", format!("{}", out));
	}
}
