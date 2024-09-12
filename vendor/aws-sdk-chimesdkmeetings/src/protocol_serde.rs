// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn type_erase_result<O, E>(
    result: ::std::result::Result<O, E>,
) -> ::std::result::Result<
    ::aws_smithy_runtime_api::client::interceptors::context::Output,
    ::aws_smithy_runtime_api::client::orchestrator::OrchestratorError<::aws_smithy_runtime_api::client::interceptors::context::Error>,
>
where
    O: ::std::fmt::Debug + ::std::marker::Send + ::std::marker::Sync + 'static,
    E: ::std::error::Error + std::fmt::Debug + ::std::marker::Send + ::std::marker::Sync + 'static,
{
    result
        .map(|output| ::aws_smithy_runtime_api::client::interceptors::context::Output::erase(output))
        .map_err(|error| ::aws_smithy_runtime_api::client::interceptors::context::Error::erase(error))
        .map_err(::std::convert::Into::into)
}

pub fn parse_http_error_metadata(
    _response_status: u16,
    response_headers: &::aws_smithy_runtime_api::http::Headers,
    response_body: &[u8],
) -> Result<::aws_smithy_types::error::metadata::Builder, ::aws_smithy_json::deserialize::error::DeserializeError> {
    crate::json_errors::parse_error_metadata(response_body, response_headers)
}

pub(crate) mod shape_batch_create_attendee;

pub(crate) mod shape_batch_update_attendee_capabilities_except;

pub(crate) mod shape_create_attendee;

pub(crate) mod shape_create_meeting;

pub(crate) mod shape_create_meeting_with_attendees;

pub(crate) mod shape_delete_attendee;

pub(crate) mod shape_delete_meeting;

pub(crate) mod shape_get_attendee;

pub(crate) mod shape_get_meeting;

pub(crate) mod shape_list_attendees;

pub(crate) mod shape_list_tags_for_resource;

pub(crate) mod shape_start_meeting_transcription;

pub(crate) mod shape_stop_meeting_transcription;

pub(crate) mod shape_tag_resource;

pub(crate) mod shape_untag_resource;

pub(crate) mod shape_update_attendee_capabilities;

pub(crate) fn or_empty_doc(data: &[u8]) -> &[u8] {
    if data.is_empty() {
        b"{}"
    } else {
        data
    }
}

pub(crate) mod shape_bad_request_exception;

pub(crate) mod shape_batch_create_attendee_input;

pub(crate) mod shape_batch_update_attendee_capabilities_except_input;

pub(crate) mod shape_conflict_exception;

pub(crate) mod shape_create_attendee_input;

pub(crate) mod shape_create_meeting_input;

pub(crate) mod shape_create_meeting_with_attendees_input;

pub(crate) mod shape_forbidden_exception;

pub(crate) mod shape_limit_exceeded_exception;

pub(crate) mod shape_not_found_exception;

pub(crate) mod shape_resource_not_found_exception;

pub(crate) mod shape_service_failure_exception;

pub(crate) mod shape_service_unavailable_exception;

pub(crate) mod shape_start_meeting_transcription_input;

pub(crate) mod shape_tag_resource_input;

pub(crate) mod shape_throttling_exception;

pub(crate) mod shape_too_many_tags_exception;

pub(crate) mod shape_unauthorized_exception;

pub(crate) mod shape_unprocessable_entity_exception;

pub(crate) mod shape_untag_resource_input;

pub(crate) mod shape_update_attendee_capabilities_input;

pub(crate) mod shape_attendee;

pub(crate) mod shape_attendee_capabilities;

pub(crate) mod shape_attendee_id_item;

pub(crate) mod shape_attendee_list;

pub(crate) mod shape_batch_create_attendee_error_list;

pub(crate) mod shape_create_attendee_request_item;

pub(crate) mod shape_meeting;

pub(crate) mod shape_meeting_features_configuration;

pub(crate) mod shape_notifications_configuration;

pub(crate) mod shape_tag;

pub(crate) mod shape_tag_list;

pub(crate) mod shape_transcription_configuration;

pub(crate) mod shape_attendee_features;

pub(crate) mod shape_audio_features;

pub(crate) mod shape_content_features;

pub(crate) mod shape_create_attendee_error;

pub(crate) mod shape_engine_transcribe_medical_settings;

pub(crate) mod shape_engine_transcribe_settings;

pub(crate) mod shape_media_placement;

pub(crate) mod shape_tenant_id_list;

pub(crate) mod shape_video_features;
