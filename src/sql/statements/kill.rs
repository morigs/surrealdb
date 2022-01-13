use crate::dbs;
use crate::dbs::Executor;
use crate::dbs::Options;
use crate::dbs::Runtime;
use crate::err::Error;
use crate::sql::comment::shouldbespace;
use crate::sql::ident::{ident, Ident};
use crate::sql::value::Value;
use nom::bytes::complete::tag_no_case;
use nom::IResult;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct KillStatement {
	pub id: Ident,
}

impl fmt::Display for KillStatement {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "KILL {}", self.id)
	}
}

impl dbs::Process for KillStatement {
	fn process(
		&self,
		_ctx: &Runtime,
		_opt: &Options,
		_exe: &mut Executor,
		_doc: Option<&Value>,
	) -> Result<Value, Error> {
		todo!()
	}
}

pub fn kill(i: &str) -> IResult<&str, KillStatement> {
	let (i, _) = tag_no_case("KILL")(i)?;
	let (i, _) = shouldbespace(i)?;
	let (i, v) = ident(i)?;
	Ok((
		i,
		KillStatement {
			id: v,
		},
	))
}
