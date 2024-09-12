// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_start_meeting_transcription_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::start_meeting_transcription::StartMeetingTranscriptionInput,
) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.transcription_configuration {
        #[allow(unused_mut)]
        let mut object_2 = object.key("TranscriptionConfiguration").start_object();
        crate::protocol_serde::shape_transcription_configuration::ser_transcription_configuration(&mut object_2, var_1)?;
        object_2.finish();
    }
    Ok(())
}