// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_voice_profile_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::update_voice_profile::UpdateVoiceProfileInput,
) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.speaker_search_task_id {
        object.key("SpeakerSearchTaskId").string(var_1.as_str());
    }
    Ok(())
}