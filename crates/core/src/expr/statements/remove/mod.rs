mod access;
mod analyzer;
mod bucket;
mod database;
mod event;
mod field;
mod function;
mod index;
mod model;
mod namespace;
mod param;
mod sequence;
mod table;
mod user;

pub use access::RemoveAccessStatement;
pub use analyzer::RemoveAnalyzerStatement;
pub use bucket::RemoveBucketStatement;
pub use database::RemoveDatabaseStatement;
pub use event::RemoveEventStatement;
pub use field::RemoveFieldStatement;
pub use function::RemoveFunctionStatement;
pub use index::RemoveIndexStatement;
pub use model::RemoveModelStatement;
pub use namespace::RemoveNamespaceStatement;
pub use param::RemoveParamStatement;
pub use sequence::RemoveSequenceStatement;
pub use table::RemoveTableStatement;
pub use user::RemoveUserStatement;

use crate::ctx::Context;
use crate::dbs::Options;
use crate::doc::CursorDoc;
use crate::expr::Value;
use anyhow::Result;

use revision::revisioned;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display, Formatter};

#[revisioned(revision = 1)]
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Serialize, Deserialize, Hash)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
#[non_exhaustive]
pub enum RemoveStatement {
	Namespace(RemoveNamespaceStatement),
	Database(RemoveDatabaseStatement),
	Function(RemoveFunctionStatement),
	Analyzer(RemoveAnalyzerStatement),
	Access(RemoveAccessStatement),
	Param(RemoveParamStatement),
	Table(RemoveTableStatement),
	Event(RemoveEventStatement),
	Field(RemoveFieldStatement),
	Index(RemoveIndexStatement),
	User(RemoveUserStatement),
	Model(RemoveModelStatement),
	Bucket(RemoveBucketStatement),
	Sequence(RemoveSequenceStatement),
}

impl RemoveStatement {
	/// Check if we require a writeable transaction
	pub(crate) fn writeable(&self) -> bool {
		true
	}
	/// Process this type returning a computed simple Value
	pub(crate) async fn compute(
		&self,
		ctx: &Context,
		opt: &Options,
		_doc: Option<&CursorDoc>,
	) -> Result<Value> {
		match self {
			Self::Namespace(v) => v.compute(ctx, opt).await,
			Self::Database(v) => v.compute(ctx, opt).await,
			Self::Function(v) => v.compute(ctx, opt).await,
			Self::Access(v) => v.compute(ctx, opt).await,
			Self::Param(v) => v.compute(ctx, opt).await,
			Self::Table(v) => v.compute(ctx, opt).await,
			Self::Event(v) => v.compute(ctx, opt).await,
			Self::Field(v) => v.compute(ctx, opt).await,
			Self::Index(v) => v.compute(ctx, opt).await,
			Self::Analyzer(v) => v.compute(ctx, opt).await,
			Self::User(v) => v.compute(ctx, opt).await,
			Self::Model(v) => v.compute(ctx, opt).await,
			Self::Bucket(v) => v.compute(ctx, opt).await,
			Self::Sequence(v) => v.compute(ctx, opt).await,
		}
	}
}

impl Display for RemoveStatement {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		match self {
			Self::Namespace(v) => Display::fmt(v, f),
			Self::Database(v) => Display::fmt(v, f),
			Self::Function(v) => Display::fmt(v, f),
			Self::Access(v) => Display::fmt(v, f),
			Self::Param(v) => Display::fmt(v, f),
			Self::Table(v) => Display::fmt(v, f),
			Self::Event(v) => Display::fmt(v, f),
			Self::Field(v) => Display::fmt(v, f),
			Self::Index(v) => Display::fmt(v, f),
			Self::Analyzer(v) => Display::fmt(v, f),
			Self::User(v) => Display::fmt(v, f),
			Self::Model(v) => Display::fmt(v, f),
			Self::Bucket(v) => Display::fmt(v, f),
			Self::Sequence(v) => Display::fmt(v, f),
		}
	}
}
