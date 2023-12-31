use axum::extract::{multipart::MultipartRejection, rejection::JsonRejection};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
pub type Result<T> = core::result::Result<T, Error>;

#[derive(Clone, Debug, Serialize, strum_macros::AsRefStr)]
#[serde(tag = "type", content = "data")]
pub enum Error {
    LoginFail,
    AuthFailNoAuthToken,
    AuthFailTokenWrongFormat,
    AuthFailTokenInvalid,
    AuthFailTokenExpired,
    AuthErrorCtxNotInRequestExt,
    InvalidAccessDirectoryTraversal,
    InvalidAccessEscapeBaseDir,
    InvalidFilePath,
    InvalidFile,
    InvalidMimeType,
    FileNotFound,
    ParseFailInvalidWhiteList,
    ParseFailInvalidBlackList,
    MultipartInvalidBoundary,
    MultipartUnknownError,
    InvalidJson,
}

impl Error {
    /// Translate internal error into client error with less internal inrofmation
    /// `returns` an http status code + the clienterror message
    pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
        // fallback maybe redundant
        #[allow(unreachable_patterns)]
        match self {
            // - Auth
            Self::AuthFailNoAuthToken
            | Self::AuthFailTokenExpired
            | Self::AuthFailTokenWrongFormat
            | Self::AuthErrorCtxNotInRequestExt => (StatusCode::FORBIDDEN, ClientError::NO_AUTH),
            // - Model niy

            // File Access
            Self::InvalidAccessEscapeBaseDir
            | Self::InvalidFilePath
            | Self::InvalidFile
            | Self::MultipartInvalidBoundary
            | Self::MultipartUnknownError
            | Self::InvalidJson
            | Self::FileNotFound
            | Self::InvalidMimeType
            | Self::InvalidAccessDirectoryTraversal => {
                (StatusCode::BAD_REQUEST, ClientError::INVALID_ACCESS)
            }

            // - Fallback
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ClientError::SERVICE_ERROR,
            ),
        }
    }
}

impl From<MultipartRejection> for Error {
    /// Creates internal error from from Multipart extractor rejection
    /// * `rej` - the rejection of the Multipart extractor
    fn from(rej: MultipartRejection) -> Self {
        match rej {
            MultipartRejection::InvalidBoundary(_) => Self::MultipartInvalidBoundary,
            _ => Self::MultipartUnknownError,
        }
    }
}

impl From<JsonRejection> for Error {
    /// Creates internal error from from Json extractor rejection
    /// * `rej` - the rejection of the json extractor
    fn from(_rej: JsonRejection) -> Self {
        Self::InvalidJson
    }
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

        // Create a a placeholder Axum response
        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();
        // Insert the Error intoresponse
        response.extensions_mut().insert(self);
        response
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, strum_macros::AsRefStr)]
pub enum ClientError {
    LOGIN_FAIL,
    NO_AUTH,
    INVALID_PARAMS,
    SERVICE_ERROR,
    INVALID_ACCESS,
}
