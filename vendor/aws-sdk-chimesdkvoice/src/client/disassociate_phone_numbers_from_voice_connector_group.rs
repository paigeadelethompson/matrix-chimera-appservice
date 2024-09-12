// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DisassociatePhoneNumbersFromVoiceConnectorGroup`](crate::operation::disassociate_phone_numbers_from_voice_connector_group::builders::DisassociatePhoneNumbersFromVoiceConnectorGroupFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`voice_connector_group_id(impl Into<String>)`](crate::operation::disassociate_phone_numbers_from_voice_connector_group::builders::DisassociatePhoneNumbersFromVoiceConnectorGroupFluentBuilder::voice_connector_group_id) / [`set_voice_connector_group_id(Option<String>)`](crate::operation::disassociate_phone_numbers_from_voice_connector_group::builders::DisassociatePhoneNumbersFromVoiceConnectorGroupFluentBuilder::set_voice_connector_group_id):<br>required: **true**<br><p>The Voice Connector group ID.</p><br>
    ///   - [`e164_phone_numbers(impl Into<String>)`](crate::operation::disassociate_phone_numbers_from_voice_connector_group::builders::DisassociatePhoneNumbersFromVoiceConnectorGroupFluentBuilder::e164_phone_numbers) / [`set_e164_phone_numbers(Option<Vec::<String>>)`](crate::operation::disassociate_phone_numbers_from_voice_connector_group::builders::DisassociatePhoneNumbersFromVoiceConnectorGroupFluentBuilder::set_e164_phone_numbers):<br>required: **true**<br><p>The list of phone numbers, in E.164 format.</p><br>
    /// - On success, responds with [`DisassociatePhoneNumbersFromVoiceConnectorGroupOutput`](crate::operation::disassociate_phone_numbers_from_voice_connector_group::DisassociatePhoneNumbersFromVoiceConnectorGroupOutput) with field(s):
    ///   - [`phone_number_errors(Option<Vec::<PhoneNumberError>>)`](crate::operation::disassociate_phone_numbers_from_voice_connector_group::DisassociatePhoneNumbersFromVoiceConnectorGroupOutput::phone_number_errors): <p>If the action fails for one or more of the phone numbers in the request, a list of the phone numbers is returned, along with error codes and error messages.</p>
    /// - On failure, responds with [`SdkError<DisassociatePhoneNumbersFromVoiceConnectorGroupError>`](crate::operation::disassociate_phone_numbers_from_voice_connector_group::DisassociatePhoneNumbersFromVoiceConnectorGroupError)
    pub fn disassociate_phone_numbers_from_voice_connector_group(
        &self,
    ) -> crate::operation::disassociate_phone_numbers_from_voice_connector_group::builders::DisassociatePhoneNumbersFromVoiceConnectorGroupFluentBuilder
    {
        crate::operation::disassociate_phone_numbers_from_voice_connector_group::builders::DisassociatePhoneNumbersFromVoiceConnectorGroupFluentBuilder::new(self.handle.clone())
    }
}
