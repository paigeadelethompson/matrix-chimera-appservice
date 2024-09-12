// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_update_continuous_backups_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::update_continuous_backups::UpdateContinuousBackupsOutput,
    crate::operation::update_continuous_backups::UpdateContinuousBackupsError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::update_continuous_backups::UpdateContinuousBackupsError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => {
            return Err(crate::operation::update_continuous_backups::UpdateContinuousBackupsError::unhandled(
                generic,
            ))
        }
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "ContinuousBackupsUnavailableException" => {
            crate::operation::update_continuous_backups::UpdateContinuousBackupsError::ContinuousBackupsUnavailableException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ContinuousBackupsUnavailableExceptionBuilder::default();
                    output =
                        crate::protocol_serde::shape_continuous_backups_unavailable_exception::de_continuous_backups_unavailable_exception_json_err(
                            _response_body,
                            output,
                        )
                        .map_err(crate::operation::update_continuous_backups::UpdateContinuousBackupsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "InternalServerError" => crate::operation::update_continuous_backups::UpdateContinuousBackupsError::InternalServerError({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InternalServerErrorBuilder::default();
                output = crate::protocol_serde::shape_internal_server_error::de_internal_server_error_json_err(_response_body, output)
                    .map_err(crate::operation::update_continuous_backups::UpdateContinuousBackupsError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InvalidEndpointException" => crate::operation::update_continuous_backups::UpdateContinuousBackupsError::InvalidEndpointException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidEndpointExceptionBuilder::default();
                output = crate::protocol_serde::shape_invalid_endpoint_exception::de_invalid_endpoint_exception_json_err(_response_body, output)
                    .map_err(crate::operation::update_continuous_backups::UpdateContinuousBackupsError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "TableNotFoundException" => crate::operation::update_continuous_backups::UpdateContinuousBackupsError::TableNotFoundException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::TableNotFoundExceptionBuilder::default();
                output = crate::protocol_serde::shape_table_not_found_exception::de_table_not_found_exception_json_err(_response_body, output)
                    .map_err(crate::operation::update_continuous_backups::UpdateContinuousBackupsError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        _ => crate::operation::update_continuous_backups::UpdateContinuousBackupsError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_update_continuous_backups_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::update_continuous_backups::UpdateContinuousBackupsOutput,
    crate::operation::update_continuous_backups::UpdateContinuousBackupsError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::update_continuous_backups::builders::UpdateContinuousBackupsOutputBuilder::default();
        output = crate::protocol_serde::shape_update_continuous_backups::de_update_continuous_backups(_response_body, output)
            .map_err(crate::operation::update_continuous_backups::UpdateContinuousBackupsError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

pub fn ser_update_continuous_backups_input(
    input: &crate::operation::update_continuous_backups::UpdateContinuousBackupsInput,
) -> Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_update_continuous_backups_input::ser_update_continuous_backups_input_input(&mut object, input)?;
    object.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}

pub(crate) fn de_update_continuous_backups(
    value: &[u8],
    mut builder: crate::operation::update_continuous_backups::builders::UpdateContinuousBackupsOutputBuilder,
) -> Result<
    crate::operation::update_continuous_backups::builders::UpdateContinuousBackupsOutputBuilder,
    ::aws_smithy_json::deserialize::error::DeserializeError,
> {
    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
    let tokens = &mut tokens_owned;
    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                "ContinuousBackupsDescription" => {
                    builder = builder.set_continuous_backups_description(
                        crate::protocol_serde::shape_continuous_backups_description::de_continuous_backups_description(tokens)?,
                    );
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
