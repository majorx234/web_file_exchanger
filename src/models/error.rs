use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Clone, Debug)]
pub enum Error {
    LoginFail,
    AuthFailNoAuthToken,
    AuthFailTokenWrongFormat,
    AuthFailTokenInvalid,
    AuthErrorCtxNotInRequestExt,
}

impl std::fmt::Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        println!("->> {:12} - {self:?}", "INTO_RES");
        match self {
            Error::LoginFail => {
                (StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLED_CLIENT_ERROR").into_response()
            }
            Error::AuthFailNoAuthToken => {
                (StatusCode::UNAUTHORIZED, "UNAUTHORIZED_CLIENT_ERROR").into_response()
            }
            Error::AuthFailTokenWrongFormat => {
                (StatusCode::UNAUTHORIZED, "UNAUTHORIZED_CLIENT_ERROR").into_response()
            }
            Error::AuthFailTokenInvalid => {
                (StatusCode::UNAUTHORIZED, "UNAUTHORIZED_CLIENT_ERROR").into_response()
            }
            Error::AuthErrorCtxNotInRequestExt => {
                (StatusCode::UNAUTHORIZED, "UNAUTHORIZED_CLIENT_ERROR").into_response()
            }
        }
    }
}
