// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_disassociate_channel_flow_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::disassociate_channel_flow::DisassociateChannelFlowOutput,
    crate::operation::disassociate_channel_flow::DisassociateChannelFlowError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::disassociate_channel_flow::DisassociateChannelFlowError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => {
            return Err(crate::operation::disassociate_channel_flow::DisassociateChannelFlowError::unhandled(
                generic,
            ))
        }
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "BadRequestException" => crate::operation::disassociate_channel_flow::DisassociateChannelFlowError::BadRequestException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::BadRequestExceptionBuilder::default();
                output = crate::protocol_serde::shape_bad_request_exception::de_bad_request_exception_json_err(_response_body, output)
                    .map_err(crate::operation::disassociate_channel_flow::DisassociateChannelFlowError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "ConflictException" => crate::operation::disassociate_channel_flow::DisassociateChannelFlowError::ConflictException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ConflictExceptionBuilder::default();
                output = crate::protocol_serde::shape_conflict_exception::de_conflict_exception_json_err(_response_body, output)
                    .map_err(crate::operation::disassociate_channel_flow::DisassociateChannelFlowError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "ForbiddenException" => crate::operation::disassociate_channel_flow::DisassociateChannelFlowError::ForbiddenException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ForbiddenExceptionBuilder::default();
                output = crate::protocol_serde::shape_forbidden_exception::de_forbidden_exception_json_err(_response_body, output)
                    .map_err(crate::operation::disassociate_channel_flow::DisassociateChannelFlowError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "NotFoundException" => crate::operation::disassociate_channel_flow::DisassociateChannelFlowError::NotFoundException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::NotFoundExceptionBuilder::default();
                output = crate::protocol_serde::shape_not_found_exception::de_not_found_exception_json_err(_response_body, output)
                    .map_err(crate::operation::disassociate_channel_flow::DisassociateChannelFlowError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "ServiceFailureException" => crate::operation::disassociate_channel_flow::DisassociateChannelFlowError::ServiceFailureException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ServiceFailureExceptionBuilder::default();
                output = crate::protocol_serde::shape_service_failure_exception::de_service_failure_exception_json_err(_response_body, output)
                    .map_err(crate::operation::disassociate_channel_flow::DisassociateChannelFlowError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "ServiceUnavailableException" => crate::operation::disassociate_channel_flow::DisassociateChannelFlowError::ServiceUnavailableException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ServiceUnavailableExceptionBuilder::default();
                output =
                    crate::protocol_serde::shape_service_unavailable_exception::de_service_unavailable_exception_json_err(_response_body, output)
                        .map_err(crate::operation::disassociate_channel_flow::DisassociateChannelFlowError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "ThrottledClientException" => crate::operation::disassociate_channel_flow::DisassociateChannelFlowError::ThrottledClientException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ThrottledClientExceptionBuilder::default();
                output = crate::protocol_serde::shape_throttled_client_exception::de_throttled_client_exception_json_err(_response_body, output)
                    .map_err(crate::operation::disassociate_channel_flow::DisassociateChannelFlowError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "UnauthorizedClientException" => crate::operation::disassociate_channel_flow::DisassociateChannelFlowError::UnauthorizedClientException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::UnauthorizedClientExceptionBuilder::default();
                output =
                    crate::protocol_serde::shape_unauthorized_client_exception::de_unauthorized_client_exception_json_err(_response_body, output)
                        .map_err(crate::operation::disassociate_channel_flow::DisassociateChannelFlowError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        _ => crate::operation::disassociate_channel_flow::DisassociateChannelFlowError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_disassociate_channel_flow_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::disassociate_channel_flow::DisassociateChannelFlowOutput,
    crate::operation::disassociate_channel_flow::DisassociateChannelFlowError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::disassociate_channel_flow::builders::DisassociateChannelFlowOutputBuilder::default();
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

pub fn ser_disassociate_channel_flow_headers(
    input: &crate::operation::disassociate_channel_flow::DisassociateChannelFlowInput,
    mut builder: ::http::request::Builder,
) -> std::result::Result<::http::request::Builder, ::aws_smithy_types::error::operation::BuildError> {
    if let ::std::option::Option::Some(inner_1) = &input.chime_bearer {
        let formatted_2 = inner_1.as_str();
        if !formatted_2.is_empty() {
            let header_value = formatted_2;
            let header_value: ::http::HeaderValue = header_value.parse().map_err(|err| {
                ::aws_smithy_types::error::operation::BuildError::invalid_field(
                    "chime_bearer",
                    format!("`{}` cannot be used as a header value: {}", &header_value, err),
                )
            })?;
            builder = builder.header("x-amz-chime-bearer", header_value);
        }
    }
    Ok(builder)
}