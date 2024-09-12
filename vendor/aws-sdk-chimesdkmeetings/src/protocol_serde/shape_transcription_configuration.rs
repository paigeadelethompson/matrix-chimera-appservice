// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_transcription_configuration(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::TranscriptionConfiguration,
) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.engine_transcribe_settings {
        #[allow(unused_mut)]
        let mut object_2 = object.key("EngineTranscribeSettings").start_object();
        crate::protocol_serde::shape_engine_transcribe_settings::ser_engine_transcribe_settings(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.engine_transcribe_medical_settings {
        #[allow(unused_mut)]
        let mut object_4 = object.key("EngineTranscribeMedicalSettings").start_object();
        crate::protocol_serde::shape_engine_transcribe_medical_settings::ser_engine_transcribe_medical_settings(&mut object_4, var_3)?;
        object_4.finish();
    }
    Ok(())
}