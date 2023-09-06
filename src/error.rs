use axum::{response::{IntoResponse, Response}, Error};
use serde::Serialize;

impl core::fmt::Display for AuthError {
	fn fmt(
		&self,
		fmt: &mut core::fmt::Formatter,
	) -> core::result::Result<(), core::fmt::Error> {
		write!(fmt, "{self:?}")
	}
}

pub type Result<T> = core::result::Result<T, Error>;


#[derive(Clone, Debug, Serialize, strum_macros::AsRefStr)]
#[serde(tag = "type", content = "data")]
pub enum AuthError {
    AuthRegisterFailed,
	AuthFailNoAuthTokenCookie,
	AuthFailTokenWrongFormat,
	AuthFailCtxNotInRequestExt
}

#[derive(Debug, strum_macros::AsRefStr)]
pub enum ClientError {
	LoginFail,
	NoAuth,
	InvalidParams,
	ServiceError,
}

impl std::error::Error for AuthError {}

impl IntoResponse for AuthError {
	fn into_response(self) -> Response {
		let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();
		response.extensions_mut().insert(self); 
		response
	}
}

impl Error {
	pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
		match self {
			Self::LoginFail => (StatusCode::FORBIDDEN, ClientError::LOGIN_FAIL),

			Self::AuthFailNoAuthTokenCookie
			| Self::AuthFailTokenWrongFormat
			| Self::AuthFailCtxNotInRequestExt => {
				(StatusCode::FORBIDDEN, ClientError::NO_AUTH)
			}

			_ => (
				StatusCode::INTERNAL_SERVER_ERROR,
				ClientError::SERVICE_ERROR,
			),
		}
	}
}

