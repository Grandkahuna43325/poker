use serde::de::{self, MapAccess, Visitor};
use serde::{Deserialize, Deserializer};

#[derive(Debug)]
pub enum ServerResponse {
    Ok,
    ConnectingToDbError(String),
    DieselError(String),
    BadPassword,
    UserDoesNotExist,
    U32toI32,
    BadRequest,
    ArgonError,
    ChangePasswordError,
}

//impl display for error
impl std::fmt::Display for ServerResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ServerResponse::Ok => write!(f, "Ok"),
            ServerResponse::ConnectingToDbError(err) => write!(f, "ConnectingToDbError: {}", err),
            ServerResponse::DieselError(err) => write!(f, "DieselError: {}", err),
            ServerResponse::BadPassword => write!(f, "BadPassword"),
            ServerResponse::UserDoesNotExist => write!(f, "UserDoesNotExist"),
            ServerResponse::U32toI32 => write!(f, "U32toI32"),
            ServerResponse::BadRequest => write!(f, "BadRequest"),
            ServerResponse::ArgonError => write!(f, "ArgonError"),
            ServerResponse::ChangePasswordError => write!(f, "ChangePasswordError"),
        }
    }
}

impl<'de> Deserialize<'de> for ServerResponse {
    fn deserialize<D>(deserializer: D) -> Result<ServerResponse, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ErrorVisitor;

        impl<'de> Visitor<'de> for ErrorVisitor {
            type Value = ServerResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct Error")
            }

            fn visit_map<A>(self, mut map: A) -> Result<ServerResponse, A::Error>
            where
                A: MapAccess<'de>,
            {
                let variant = map.next_key::<&str>()?;
                match variant {
                    Some("Ok") => {
                        let value: String = map.next_value()?;
                       let _ = value.len();
                        Ok(ServerResponse::Ok) },
                    Some("ConnectingToDbError") => {
                        let value = map.next_value()?;
                        Ok(ServerResponse::ConnectingToDbError(value))
                    }
                    Some("DieselError") => {
                        let value = map.next_value()?;
                        Ok(ServerResponse::DieselError(value))
                    }
                    Some("U32toI32") => {
                        let _value: String = map.next_value()?;
                        Ok(ServerResponse::U32toI32)
                    }
                    Some("BadRequest") => {
                        let value: String = map.next_value()?;
                        let _ = value.len();
                        Ok(ServerResponse::BadRequest)
                    }
                    Some("ArgonError") => {
                        let value: String = map.next_value()?;
                        let _ = value.len();
                        Ok(ServerResponse::ArgonError)
                    }
                    Some("BadPassword") => {
                        let value: String = map.next_value()?;
                        let _ = value.len();
                        Ok(ServerResponse::BadPassword)
                    }
                    Some("ChangePasswordError") => {
                        let value: String = map.next_value()?;
                        let _ = value.len();
                        Ok(ServerResponse::ChangePasswordError)
                    }
                    Some("UserDoesNotExist") => {
                        let value: String = map.next_value()?;
                        let _ = value.len();
                        Ok(ServerResponse::UserDoesNotExist)
                    }
                    _ => {
                        println!("past all some got to _{:?}", variant);
                        Err(de::Error::unknown_variant(
                            &format!("{:?}", variant),
                            &[
                                "ConnectingToDbError",
                                "DieselError",
                                "WriteFileError",
                                "CreateFileError",
                                "DeleteFileError",
                                "PostError",
                                "U32toI32",
                                "BadRequest",
                                "BadPassword",
                                "ArgonError",
                                "ChangePasswordError",
                            ],
                        ))
                    }
                }
            }
        }

        deserializer.deserialize_struct("Error", &["variant", "value"], ErrorVisitor)
    }
}

