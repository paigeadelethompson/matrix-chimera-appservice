// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_phone_number_settings_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::update_phone_number_settings::UpdatePhoneNumberSettingsInput,
) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.calling_name {
        object.key("CallingName").string(var_1.as_str());
    }
    Ok(())
}