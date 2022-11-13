use crate::http1::components::{
    date::Date as DateComponent, coding::TransferCoding
};

use super::FieldName;

pub(crate) struct CacheControl(
    Vec<CacheDirective>
);
    enum CacheDirective {
        Request(CacheRequestDirective),
        Response(CacheResponseDirective),
    }
        enum CacheRequestDirective {
            NoCache,
            NoStore,
            MaxAge(/*delta-seconds*/usize),
            MaxStale(Option</*delta-seconds*/usize>),
            MinFresh(/*delta-seconds*/usize),
            NoTransform,
            OnlyIfCached,
        }
        enum CacheResponseDirective {
            Public,
            Private(Vec<FieldName>),
            NoCache(Vec<FieldName>),
            NoStore,
            NoTransform,
            MustRevalidate,
            ProxyRevalidate,
            MaxAge(/*delta-seconds*/usize),
            SMaxAge(/*delta-seconds*/usize),
        }
pub(crate) struct Connection(
    Vec<ConnectionToken>
);
    enum ConnectionToken {
        Close
    }
pub(crate) struct Date(
    DateComponent
);
// pub(crate) struct Pragma
// this seems to be optional
pub(crate) struct Trailer(
    Vec<FieldName>
);
pub(crate) struct TransferEncoding(
    Vec<TransferCoding>
);
// pub(crate) struct Upgrade
// I don't support this
pub(crate) struct Via(
    Vec<Receiver>
);
    struct Receiver {
        protocol_name:    Option<String>,
        protocol_version: String,
        host:             String, /* or pseudonym */
        port:             Option<u16>,
    }
pub(crate) struct Warning(
    Vec<WaringValue>
);
    struct WaringValue {
        code:  WarningCode,
        agent: String, // host name or pseudonym
        text:  String,
    }
        enum WarningCode {
            ResponseIsStale       = 110,
            RevalidationFailed    = 111,
            DisconnectedOperation = 112,
            HeuristicExpiration   = 113,
            MiscellaneousWarning  = 199,
            TransformationApplied = 214,
            // MiscellaneousPersostentWarning
            // what's the difference from 'MiscellaneousWarning' ?
        }