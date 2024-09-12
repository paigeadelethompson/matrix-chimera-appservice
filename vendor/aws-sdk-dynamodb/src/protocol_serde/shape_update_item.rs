// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_update_item_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<crate::operation::update_item::UpdateItemOutput, crate::operation::update_item::UpdateItemError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::update_item::UpdateItemError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::operation::update_item::UpdateItemError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "ConditionalCheckFailedException" => crate::operation::update_item::UpdateItemError::ConditionalCheckFailedException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ConditionalCheckFailedExceptionBuilder::default();
                output = crate::protocol_serde::shape_conditional_check_failed_exception::de_conditional_check_failed_exception_json_err(
                    _response_body,
                    output,
                )
                .map_err(crate::operation::update_item::UpdateItemError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InternalServerError" => crate::operation::update_item::UpdateItemError::InternalServerError({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InternalServerErrorBuilder::default();
                output = crate::protocol_serde::shape_internal_server_error::de_internal_server_error_json_err(_response_body, output)
                    .map_err(crate::operation::update_item::UpdateItemError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InvalidEndpointException" => crate::operation::update_item::UpdateItemError::InvalidEndpointException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidEndpointExceptionBuilder::default();
                output = crate::protocol_serde::shape_invalid_endpoint_exception::de_invalid_endpoint_exception_json_err(_response_body, output)
                    .map_err(crate::operation::update_item::UpdateItemError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "ItemCollectionSizeLimitExceededException" => crate::operation::update_item::UpdateItemError::ItemCollectionSizeLimitExceededException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ItemCollectionSizeLimitExceededExceptionBuilder::default();
                output = crate::protocol_serde::shape_item_collection_size_limit_exceeded_exception::de_item_collection_size_limit_exceeded_exception_json_err(_response_body, output).map_err(crate::operation::update_item::UpdateItemError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "ProvisionedThroughputExceededException" => {
            crate::operation::update_item::UpdateItemError::ProvisionedThroughputExceededException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ProvisionedThroughputExceededExceptionBuilder::default();
                    output = crate::protocol_serde::shape_provisioned_throughput_exceeded_exception::de_provisioned_throughput_exceeded_exception_json_err(_response_body, output).map_err(crate::operation::update_item::UpdateItemError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "RequestLimitExceeded" => crate::operation::update_item::UpdateItemError::RequestLimitExceeded({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::RequestLimitExceededBuilder::default();
                output = crate::protocol_serde::shape_request_limit_exceeded::de_request_limit_exceeded_json_err(_response_body, output)
                    .map_err(crate::operation::update_item::UpdateItemError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "ResourceNotFoundException" => crate::operation::update_item::UpdateItemError::ResourceNotFoundException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ResourceNotFoundExceptionBuilder::default();
                output = crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
                    .map_err(crate::operation::update_item::UpdateItemError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "TransactionConflictException" => crate::operation::update_item::UpdateItemError::TransactionConflictException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::TransactionConflictExceptionBuilder::default();
                output =
                    crate::protocol_serde::shape_transaction_conflict_exception::de_transaction_conflict_exception_json_err(_response_body, output)
                        .map_err(crate::operation::update_item::UpdateItemError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        _ => crate::operation::update_item::UpdateItemError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_update_item_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<crate::operation::update_item::UpdateItemOutput, crate::operation::update_item::UpdateItemError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::update_item::builders::UpdateItemOutputBuilder::default();
        output = crate::protocol_serde::shape_update_item::de_update_item(_response_body, output)
            .map_err(crate::operation::update_item::UpdateItemError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

pub fn ser_update_item_input(
    input: &crate::operation::update_item::UpdateItemInput,
) -> Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_update_item_input::ser_update_item_input_input(&mut object, input)?;
    object.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}

pub(crate) fn de_update_item(
    value: &[u8],
    mut builder: crate::operation::update_item::builders::UpdateItemOutputBuilder,
) -> Result<crate::operation::update_item::builders::UpdateItemOutputBuilder, ::aws_smithy_json::deserialize::error::DeserializeError> {
    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
    let tokens = &mut tokens_owned;
    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                "Attributes" => {
                    builder = builder.set_attributes(crate::protocol_serde::shape_attribute_map::de_attribute_map(tokens)?);
                }
                "ConsumedCapacity" => {
                    builder = builder.set_consumed_capacity(crate::protocol_serde::shape_consumed_capacity::de_consumed_capacity(tokens)?);
                }
                "ItemCollectionMetrics" => {
                    builder = builder
                        .set_item_collection_metrics(crate::protocol_serde::shape_item_collection_metrics::de_item_collection_metrics(tokens)?);
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
