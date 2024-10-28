// Copyright 2015-2019 Benjamin Fry <benjaminfry@me.com>
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// https://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// https://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! Module for `Catalog` of `Authority` zones which are responsible for storing `RRSet` records.

use crate::proto::op::ResponseCode;

/// Result of an Update operation
pub type UpdateResult<T> = Result<T, ResponseCode>;

pub mod auth_lookup;
#[allow(clippy::module_inception)]
pub mod authority;
pub mod authority_object;
pub mod catalog;
pub mod error;
pub mod message_request;
pub mod message_response;
pub mod zone_type;

pub use self::auth_lookup::{
    AnyRecords, AuthLookup, AuthLookupIter, LookupRecords, LookupRecordsIter,
};
pub use self::authority::{Authority, LookupControlFlow, LookupOptions};
pub use self::authority_object::{AuthorityObject, DnssecSummary, EmptyLookup, LookupObject};
pub use self::catalog::Catalog;
pub use self::error::LookupError;
pub use self::message_request::{MessageRequest, Queries, UpdateRequest};
pub use self::message_response::{MessageResponse, MessageResponseBuilder};
pub use self::zone_type::ZoneType;

#[cfg(feature = "dnssec")]
pub use self::authority::{DnssecAuthority, Nsec3QueryInfo};
