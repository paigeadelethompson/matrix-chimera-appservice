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

pub(crate) mod shape_associate_phone_numbers_with_voice_connector;

pub(crate) mod shape_associate_phone_numbers_with_voice_connector_group;

pub(crate) mod shape_batch_delete_phone_number;

pub(crate) mod shape_batch_update_phone_number;

pub(crate) mod shape_create_phone_number_order;

pub(crate) mod shape_create_proxy_session;

pub(crate) mod shape_create_sip_media_application;

pub(crate) mod shape_create_sip_media_application_call;

pub(crate) mod shape_create_sip_rule;

pub(crate) mod shape_create_voice_connector;

pub(crate) mod shape_create_voice_connector_group;

pub(crate) mod shape_create_voice_profile;

pub(crate) mod shape_create_voice_profile_domain;

pub(crate) mod shape_delete_phone_number;

pub(crate) mod shape_delete_proxy_session;

pub(crate) mod shape_delete_sip_media_application;

pub(crate) mod shape_delete_sip_rule;

pub(crate) mod shape_delete_voice_connector;

pub(crate) mod shape_delete_voice_connector_emergency_calling_configuration;

pub(crate) mod shape_delete_voice_connector_group;

pub(crate) mod shape_delete_voice_connector_origination;

pub(crate) mod shape_delete_voice_connector_proxy;

pub(crate) mod shape_delete_voice_connector_streaming_configuration;

pub(crate) mod shape_delete_voice_connector_termination;

pub(crate) mod shape_delete_voice_connector_termination_credentials;

pub(crate) mod shape_delete_voice_profile;

pub(crate) mod shape_delete_voice_profile_domain;

pub(crate) mod shape_disassociate_phone_numbers_from_voice_connector;

pub(crate) mod shape_disassociate_phone_numbers_from_voice_connector_group;

pub(crate) mod shape_get_global_settings;

pub(crate) mod shape_get_phone_number;

pub(crate) mod shape_get_phone_number_order;

pub(crate) mod shape_get_phone_number_settings;

pub(crate) mod shape_get_proxy_session;

pub(crate) mod shape_get_sip_media_application;

pub(crate) mod shape_get_sip_media_application_alexa_skill_configuration;

pub(crate) mod shape_get_sip_media_application_logging_configuration;

pub(crate) mod shape_get_sip_rule;

pub(crate) mod shape_get_speaker_search_task;

pub(crate) mod shape_get_voice_connector;

pub(crate) mod shape_get_voice_connector_emergency_calling_configuration;

pub(crate) mod shape_get_voice_connector_group;

pub(crate) mod shape_get_voice_connector_logging_configuration;

pub(crate) mod shape_get_voice_connector_origination;

pub(crate) mod shape_get_voice_connector_proxy;

pub(crate) mod shape_get_voice_connector_streaming_configuration;

pub(crate) mod shape_get_voice_connector_termination;

pub(crate) mod shape_get_voice_connector_termination_health;

pub(crate) mod shape_get_voice_profile;

pub(crate) mod shape_get_voice_profile_domain;

pub(crate) mod shape_get_voice_tone_analysis_task;

pub(crate) mod shape_list_available_voice_connector_regions;

pub(crate) mod shape_list_phone_number_orders;

pub(crate) mod shape_list_phone_numbers;

pub(crate) mod shape_list_proxy_sessions;

pub(crate) mod shape_list_sip_media_applications;

pub(crate) mod shape_list_sip_rules;

pub(crate) mod shape_list_supported_phone_number_countries;

pub(crate) mod shape_list_tags_for_resource;

pub(crate) mod shape_list_voice_connector_groups;

pub(crate) mod shape_list_voice_connector_termination_credentials;

pub(crate) mod shape_list_voice_connectors;

pub(crate) mod shape_list_voice_profile_domains;

pub(crate) mod shape_list_voice_profiles;

pub(crate) mod shape_put_sip_media_application_alexa_skill_configuration;

pub(crate) mod shape_put_sip_media_application_logging_configuration;

pub(crate) mod shape_put_voice_connector_emergency_calling_configuration;

pub(crate) mod shape_put_voice_connector_logging_configuration;

pub(crate) mod shape_put_voice_connector_origination;

pub(crate) mod shape_put_voice_connector_proxy;

pub(crate) mod shape_put_voice_connector_streaming_configuration;

pub(crate) mod shape_put_voice_connector_termination;

pub(crate) mod shape_put_voice_connector_termination_credentials;

pub(crate) mod shape_restore_phone_number;

pub(crate) mod shape_search_available_phone_numbers;

pub(crate) mod shape_start_speaker_search_task;

pub(crate) mod shape_start_voice_tone_analysis_task;

pub(crate) mod shape_stop_speaker_search_task;

pub(crate) mod shape_stop_voice_tone_analysis_task;

pub(crate) mod shape_tag_resource;

pub(crate) mod shape_untag_resource;

pub(crate) mod shape_update_global_settings;

pub(crate) mod shape_update_phone_number;

pub(crate) mod shape_update_phone_number_settings;

pub(crate) mod shape_update_proxy_session;

pub(crate) mod shape_update_sip_media_application;

pub(crate) mod shape_update_sip_media_application_call;

pub(crate) mod shape_update_sip_rule;

pub(crate) mod shape_update_voice_connector;

pub(crate) mod shape_update_voice_connector_group;

pub(crate) mod shape_update_voice_profile;

pub(crate) mod shape_update_voice_profile_domain;

pub(crate) mod shape_validate_e911_address;

pub(crate) fn or_empty_doc(data: &[u8]) -> &[u8] {
    if data.is_empty() {
        b"{}"
    } else {
        data
    }
}

pub(crate) mod shape_access_denied_exception;

pub(crate) mod shape_associate_phone_numbers_with_voice_connector_group_input;

pub(crate) mod shape_associate_phone_numbers_with_voice_connector_input;

pub(crate) mod shape_bad_request_exception;

pub(crate) mod shape_batch_delete_phone_number_input;

pub(crate) mod shape_batch_update_phone_number_input;

pub(crate) mod shape_conflict_exception;

pub(crate) mod shape_create_phone_number_order_input;

pub(crate) mod shape_create_proxy_session_input;

pub(crate) mod shape_create_sip_media_application_call_input;

pub(crate) mod shape_create_sip_media_application_input;

pub(crate) mod shape_create_sip_rule_input;

pub(crate) mod shape_create_voice_connector_group_input;

pub(crate) mod shape_create_voice_connector_input;

pub(crate) mod shape_create_voice_profile_domain_input;

pub(crate) mod shape_create_voice_profile_input;

pub(crate) mod shape_delete_voice_connector_termination_credentials_input;

pub(crate) mod shape_disassociate_phone_numbers_from_voice_connector_group_input;

pub(crate) mod shape_disassociate_phone_numbers_from_voice_connector_input;

pub(crate) mod shape_forbidden_exception;

pub(crate) mod shape_gone_exception;

pub(crate) mod shape_not_found_exception;

pub(crate) mod shape_put_sip_media_application_alexa_skill_configuration_input;

pub(crate) mod shape_put_sip_media_application_logging_configuration_input;

pub(crate) mod shape_put_voice_connector_emergency_calling_configuration_input;

pub(crate) mod shape_put_voice_connector_logging_configuration_input;

pub(crate) mod shape_put_voice_connector_origination_input;

pub(crate) mod shape_put_voice_connector_proxy_input;

pub(crate) mod shape_put_voice_connector_streaming_configuration_input;

pub(crate) mod shape_put_voice_connector_termination_credentials_input;

pub(crate) mod shape_put_voice_connector_termination_input;

pub(crate) mod shape_resource_limit_exceeded_exception;

pub(crate) mod shape_service_failure_exception;

pub(crate) mod shape_service_unavailable_exception;

pub(crate) mod shape_start_speaker_search_task_input;

pub(crate) mod shape_start_voice_tone_analysis_task_input;

pub(crate) mod shape_tag_resource_input;

pub(crate) mod shape_throttled_client_exception;

pub(crate) mod shape_unauthorized_client_exception;

pub(crate) mod shape_unprocessable_entity_exception;

pub(crate) mod shape_untag_resource_input;

pub(crate) mod shape_update_global_settings_input;

pub(crate) mod shape_update_phone_number_input;

pub(crate) mod shape_update_phone_number_settings_input;

pub(crate) mod shape_update_proxy_session_input;

pub(crate) mod shape_update_sip_media_application_call_input;

pub(crate) mod shape_update_sip_media_application_input;

pub(crate) mod shape_update_sip_rule_input;

pub(crate) mod shape_update_voice_connector_group_input;

pub(crate) mod shape_update_voice_connector_input;

pub(crate) mod shape_update_voice_profile_domain_input;

pub(crate) mod shape_update_voice_profile_input;

pub(crate) mod shape_validate_e911_address_input;

pub(crate) mod shape_address;

pub(crate) mod shape_candidate_address_list;

pub(crate) mod shape_credential;

pub(crate) mod shape_e164_phone_number_list;

pub(crate) mod shape_emergency_calling_configuration;

pub(crate) mod shape_geo_match_params;

pub(crate) mod shape_logging_configuration;

pub(crate) mod shape_origination;

pub(crate) mod shape_phone_number;

pub(crate) mod shape_phone_number_countries_list;

pub(crate) mod shape_phone_number_error_list;

pub(crate) mod shape_phone_number_list;

pub(crate) mod shape_phone_number_order;

pub(crate) mod shape_phone_number_order_list;

pub(crate) mod shape_proxy;

pub(crate) mod shape_proxy_session;

pub(crate) mod shape_proxy_sessions;

pub(crate) mod shape_sensitive_string_list;

pub(crate) mod shape_server_side_encryption_configuration;

pub(crate) mod shape_sip_media_application;

pub(crate) mod shape_sip_media_application_alexa_skill_configuration;

pub(crate) mod shape_sip_media_application_call;

pub(crate) mod shape_sip_media_application_endpoint;

pub(crate) mod shape_sip_media_application_list;

pub(crate) mod shape_sip_media_application_logging_configuration;

pub(crate) mod shape_sip_rule;

pub(crate) mod shape_sip_rule_list;

pub(crate) mod shape_sip_rule_target_application;

pub(crate) mod shape_speaker_search_task;

pub(crate) mod shape_streaming_configuration;

pub(crate) mod shape_tag;

pub(crate) mod shape_tag_list;

pub(crate) mod shape_termination;

pub(crate) mod shape_termination_health;

pub(crate) mod shape_update_phone_number_request_item;

pub(crate) mod shape_voice_connector;

pub(crate) mod shape_voice_connector_aws_region_list;

pub(crate) mod shape_voice_connector_group;

pub(crate) mod shape_voice_connector_group_list;

pub(crate) mod shape_voice_connector_item;

pub(crate) mod shape_voice_connector_list;

pub(crate) mod shape_voice_connector_settings;

pub(crate) mod shape_voice_profile;

pub(crate) mod shape_voice_profile_domain;

pub(crate) mod shape_voice_profile_domain_summary_list;

pub(crate) mod shape_voice_profile_summary_list;

pub(crate) mod shape_voice_tone_analysis_task;

pub(crate) mod shape_alexa_skill_id_list;

pub(crate) mod shape_call_details;

pub(crate) mod shape_calling_region_list;

pub(crate) mod shape_candidate_address;

pub(crate) mod shape_capability_list;

pub(crate) mod shape_dnis_emergency_calling_configuration;

pub(crate) mod shape_dnis_emergency_calling_configuration_list;

pub(crate) mod shape_media_insights_configuration;

pub(crate) mod shape_ordered_phone_number_list;

pub(crate) mod shape_origination_route;

pub(crate) mod shape_origination_route_list;

pub(crate) mod shape_participants;

pub(crate) mod shape_phone_number_association_list;

pub(crate) mod shape_phone_number_capabilities;

pub(crate) mod shape_phone_number_country;

pub(crate) mod shape_phone_number_error;

pub(crate) mod shape_sip_media_application_endpoint_list;

pub(crate) mod shape_sip_rule_target_application_list;

pub(crate) mod shape_speaker_search_details;

pub(crate) mod shape_streaming_notification_target;

pub(crate) mod shape_streaming_notification_target_list;

pub(crate) mod shape_string_list;

pub(crate) mod shape_voice_connector_item_list;

pub(crate) mod shape_voice_profile_domain_summary;

pub(crate) mod shape_voice_profile_summary;

pub(crate) mod shape_ordered_phone_number;

pub(crate) mod shape_participant;

pub(crate) mod shape_phone_number_association;

pub(crate) mod shape_phone_number_type_list;

pub(crate) mod shape_speaker_search_result_list;

pub(crate) mod shape_speaker_search_result;