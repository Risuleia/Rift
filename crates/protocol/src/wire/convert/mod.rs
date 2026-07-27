mod capabilities;
mod control;
mod ids;
mod manifest;
mod session;
mod transfer;
mod heartbeat;

pub(super) use control::*;
pub(super) use ids::*;
pub(super) use manifest::*;
pub(super) use session::*;
pub(super) use transfer::*;
pub(super) use heartbeat::*;

macro_rules! proto_enum_converter {
    (
        encode = $encode:ident,
        decode = $decode:ident,
        domain = $domain:path,
        proto = $proto:path,
        error = $error:ident,

        {
            $(
                $domain_variant:ident => $proto_variant:ident
            ),+ $(,)?
        }
    ) => {

        fn $encode(value: $domain) -> $proto {
            match value {
                $(
                    <$domain>::$domain_variant => <$proto>::$proto_variant,
                )+
            }
        }

        fn $decode(value: i32) -> Result<$domain, crate::ProtocolError> {
            match <$proto>::try_from(value) {
                $(
                    Ok(<$proto>::$proto_variant) => Ok(<$domain>::$domain_variant),
                )+
                _ => Err(crate::ProtocolError::$error(value)),
            }
        }
    };
}

pub(super) use proto_enum_converter;
