// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_put_voice_connector_termination_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::put_voice_connector_termination::PutVoiceConnectorTerminationInput,
) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.termination {
        #[allow(unused_mut)]
        let mut object_2 = object.key("Termination").start_object();
        crate::protocol_serde::shape_termination::ser_termination(&mut object_2, var_1)?;
        object_2.finish();
    }
    Ok(())
}
