// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_put_channel_membership_preferences_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::put_channel_membership_preferences::PutChannelMembershipPreferencesInput,
) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.preferences {
        #[allow(unused_mut)]
        let mut object_2 = object.key("Preferences").start_object();
        crate::protocol_serde::shape_channel_membership_preferences::ser_channel_membership_preferences(&mut object_2, var_1)?;
        object_2.finish();
    }
    Ok(())
}
