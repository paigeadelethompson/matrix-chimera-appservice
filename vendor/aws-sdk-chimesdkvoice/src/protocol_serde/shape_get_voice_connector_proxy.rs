// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_get_voice_connector_proxy_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::get_voice_connector_proxy::GetVoiceConnectorProxyOutput,
    crate::operation::get_voice_connector_proxy::GetVoiceConnectorProxyError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::get_voice_connector_proxy::GetVoiceConnectorProxyError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => {
            return Err(crate::operation::get_voice_connector_proxy::GetVoiceConnectorProxyError::unhandled(
                generic,
            ))
        }
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "BadRequestException" => crate::operation::get_voice_connector_proxy::GetVoiceConnectorProxyError::BadRequestException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::BadRequestExceptionBuilder::default();
                output = crate::protocol_serde::shape_bad_request_exception::de_bad_request_exception_json_err(_response_body, output)
                    .map_err(crate::operation::get_voice_connector_proxy::GetVoiceConnectorProxyError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "ForbiddenException" => crate::operation::get_voice_connector_proxy::GetVoiceConnectorProxyError::ForbiddenException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ForbiddenExceptionBuilder::default();
                output = crate::protocol_serde::shape_forbidden_exception::de_forbidden_exception_json_err(_response_body, output)
                    .map_err(crate::operation::get_voice_connector_proxy::GetVoiceConnectorProxyError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "NotFoundException" => crate::operation::get_voice_connector_proxy::GetVoiceConnectorProxyError::NotFoundException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::NotFoundExceptionBuilder::default();
                output = crate::protocol_serde::shape_not_found_exception::de_not_found_exception_json_err(_response_body, output)
                    .map_err(crate::operation::get_voice_connector_proxy::GetVoiceConnectorProxyError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "ServiceFailureException" => crate::operation::get_voice_connector_proxy::GetVoiceConnectorProxyError::ServiceFailureException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ServiceFailureExceptionBuilder::default();
                output = crate::protocol_serde::shape_service_failure_exception::de_service_failure_exception_json_err(_response_body, output)
                    .map_err(crate::operation::get_voice_connector_proxy::GetVoiceConnectorProxyError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "ServiceUnavailableException" => crate::operation::get_voice_connector_proxy::GetVoiceConnectorProxyError::ServiceUnavailableException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ServiceUnavailableExceptionBuilder::default();
                output =
                    crate::protocol_serde::shape_service_unavailable_exception::de_service_unavailable_exception_json_err(_response_body, output)
                        .map_err(crate::operation::get_voice_connector_proxy::GetVoiceConnectorProxyError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "ThrottledClientException" => crate::operation::get_voice_connector_proxy::GetVoiceConnectorProxyError::ThrottledClientException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ThrottledClientExceptionBuilder::default();
                output = crate::protocol_serde::shape_throttled_client_exception::de_throttled_client_exception_json_err(_response_body, output)
                    .map_err(crate::operation::get_voice_connector_proxy::GetVoiceConnectorProxyError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "UnauthorizedClientException" => crate::operation::get_voice_connector_proxy::GetVoiceConnectorProxyError::UnauthorizedClientException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::UnauthorizedClientExceptionBuilder::default();
                output =
                    crate::protocol_serde::shape_unauthorized_client_exception::de_unauthorized_client_exception_json_err(_response_body, output)
                        .map_err(crate::operation::get_voice_connector_proxy::GetVoiceConnectorProxyError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        _ => crate::operation::get_voice_connector_proxy::GetVoiceConnectorProxyError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_get_voice_connector_proxy_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::get_voice_connector_proxy::GetVoiceConnectorProxyOutput,
    crate::operation::get_voice_connector_proxy::GetVoiceConnectorProxyError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::get_voice_connector_proxy::builders::GetVoiceConnectorProxyOutputBuilder::default();
        output = crate::protocol_serde::shape_get_voice_connector_proxy::de_get_voice_connector_proxy(_response_body, output)
            .map_err(crate::operation::get_voice_connector_proxy::GetVoiceConnectorProxyError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

pub(crate) fn de_get_voice_connector_proxy(
    value: &[u8],
    mut builder: crate::operation::get_voice_connector_proxy::builders::GetVoiceConnectorProxyOutputBuilder,
) -> Result<
    crate::operation::get_voice_connector_proxy::builders::GetVoiceConnectorProxyOutputBuilder,
    ::aws_smithy_json::deserialize::error::DeserializeError,
> {
    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
    let tokens = &mut tokens_owned;
    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                "Proxy" => {
                    builder = builder.set_proxy(crate::protocol_serde::shape_proxy::de_proxy(tokens)?);
                }
                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
            },
            other => {
                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
                    "expected object key or end object, found: {:?}",
                    other
                )))
            }
        }
    }
    if tokens.next().is_some() {
        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "found more JSON tokens after completing parsing",
        ));
    }
    Ok(builder)
}
