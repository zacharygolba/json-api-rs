use json_api::value::StatusCode;

use rocket::{Catcher, Error as RocketError, Request, Response, Rocket};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Status;

use respond;

pub struct ErrorHandler;

impl Fairing for ErrorHandler {
    fn info(&self) -> Info {
        Info {
            kind: Kind::Attach,
            name: "ErrorHandler",
        }
    }

    fn on_attach(&self, rocket: Rocket) -> Result<Rocket, Rocket> {
        Ok(rocket.catch(handle()))
    }
}

macro_rules! catchers {
    ({ $($status:expr => $name:ident),* }) => {
        $(fn $name(_: RocketError, _req: &Request) -> Result<Response<'static>, Status> {
            let code = $status.as_u16();
            let reason = $status
                .canonical_reason()
                .map(String::from)
                .unwrap_or(format!("{}", $status));

            let mut e = ::json_api::doc::Error::build();

            e.status($status);

            if !reason.is_empty() {
                e.title(&reason);
            }

            ::json_api::ErrorDocument::build()
                .error(e.finalize().map_err(|_| Status::InternalServerError)?)
                .finalize()
                .map_err(|_| Status::InternalServerError)
                .and_then(respond::with_body)
                .map(move |mut resp| {
                    resp.set_raw_status(code, "");
                    resp
                })
        })*

        fn handle() -> Vec<Catcher> {
            vec![$(Catcher::new($status.as_u16(), $name)),*,]
        }
    }
}

catchers!({
    StatusCode::BAD_REQUEST => handle_bad_request,
    StatusCode::UNAUTHORIZED => handle_unauthorized,
    StatusCode::PAYMENT_REQUIRED => handle_payment_required,
    StatusCode::FORBIDDEN => handle_forbidden,
    StatusCode::NOT_FOUND => handle_not_found,
    StatusCode::METHOD_NOT_ALLOWED => handle_method_not_allowed,
    StatusCode::NOT_ACCEPTABLE => handle_not_acceptable,
    StatusCode::PROXY_AUTHENTICATION_REQUIRED => handle_proxy_authentication_required,
    StatusCode::REQUEST_TIMEOUT => handle_request_timeout,
    StatusCode::CONFLICT => handle_conflict,
    StatusCode::GONE => handle_gone,
    StatusCode::LENGTH_REQUIRED => handle_length_required,
    StatusCode::PRECONDITION_FAILED => handle_precondition_failed,
    StatusCode::PAYLOAD_TOO_LARGE => handle_payload_too_large,
    StatusCode::URI_TOO_LONG => handle_uri_too_long,
    StatusCode::UNSUPPORTED_MEDIA_TYPE => handle_unsupported_media_type,
    StatusCode::RANGE_NOT_SATISFIABLE => handle_range_not_satisfiable,
    StatusCode::EXPECTATION_FAILED => handle_expectation_failed,
    StatusCode::IM_A_TEAPOT => handle_im_a_teapot,
    StatusCode::MISDIRECTED_REQUEST => handle_misdirected_request,
    StatusCode::UNPROCESSABLE_ENTITY => handle_unprocessable_entity,
    StatusCode::LOCKED => handle_locked,
    StatusCode::FAILED_DEPENDENCY => handle_failed_dependency,
    StatusCode::UPGRADE_REQUIRED => handle_upgrade_required,
    StatusCode::PRECONDITION_REQUIRED => handle_precondition_required,
    StatusCode::TOO_MANY_REQUESTS => handle_too_many_requests,
    StatusCode::REQUEST_HEADER_FIELDS_TOO_LARGE => handle_request_header_fields_too_large,
    StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS => handle_unavailable_for_legal_reasons,
    StatusCode::INTERNAL_SERVER_ERROR => handle_internal_server_error,
    StatusCode::NOT_IMPLEMENTED => handle_not_implemented,
    StatusCode::BAD_GATEWAY => handle_bad_gateway,
    StatusCode::SERVICE_UNAVAILABLE => handle_service_unavailable,
    StatusCode::GATEWAY_TIMEOUT => handle_gateway_timeout,
    StatusCode::HTTP_VERSION_NOT_SUPPORTED => handle_http_version_not_supported,
    StatusCode::VARIANT_ALSO_NEGOTIATES => handle_variant_also_negotiates,
    StatusCode::INSUFFICIENT_STORAGE => handle_insufficient_storage,
    StatusCode::LOOP_DETECTED => handle_loop_detected,
    StatusCode::NOT_EXTENDED => handle_not_extended,
    StatusCode::NETWORK_AUTHENTICATION_REQUIRED => handle_network_authentication_required
});
