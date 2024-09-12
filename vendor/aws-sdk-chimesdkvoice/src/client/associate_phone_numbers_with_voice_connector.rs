// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`AssociatePhoneNumbersWithVoiceConnector`](crate::operation::associate_phone_numbers_with_voice_connector::builders::AssociatePhoneNumbersWithVoiceConnectorFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`voice_connector_id(impl Into<String>)`](crate::operation::associate_phone_numbers_with_voice_connector::builders::AssociatePhoneNumbersWithVoiceConnectorFluentBuilder::voice_connector_id) / [`set_voice_connector_id(Option<String>)`](crate::operation::associate_phone_numbers_with_voice_connector::builders::AssociatePhoneNumbersWithVoiceConnectorFluentBuilder::set_voice_connector_id):<br>required: **true**<br><p>The Voice Connector ID.</p><br>
    ///   - [`e164_phone_numbers(impl Into<String>)`](crate::operation::associate_phone_numbers_with_voice_connector::builders::AssociatePhoneNumbersWithVoiceConnectorFluentBuilder::e164_phone_numbers) / [`set_e164_phone_numbers(Option<Vec::<String>>)`](crate::operation::associate_phone_numbers_with_voice_connector::builders::AssociatePhoneNumbersWithVoiceConnectorFluentBuilder::set_e164_phone_numbers):<br>required: **true**<br><p>List of phone numbers, in E.164 format.</p><br>
    ///   - [`force_associate(bool)`](crate::operation::associate_phone_numbers_with_voice_connector::builders::AssociatePhoneNumbersWithVoiceConnectorFluentBuilder::force_associate) / [`set_force_associate(Option<bool>)`](crate::operation::associate_phone_numbers_with_voice_connector::builders::AssociatePhoneNumbersWithVoiceConnectorFluentBuilder::set_force_associate):<br>required: **false**<br><p>If true, associates the provided phone numbers with the provided Amazon Chime SDK Voice Connector and removes any previously existing associations. If false, does not associate any phone numbers that have previously existing associations.</p><br>
    /// - On success, responds with [`AssociatePhoneNumbersWithVoiceConnectorOutput`](crate::operation::associate_phone_numbers_with_voice_connector::AssociatePhoneNumbersWithVoiceConnectorOutput) with field(s):
    ///   - [`phone_number_errors(Option<Vec::<PhoneNumberError>>)`](crate::operation::associate_phone_numbers_with_voice_connector::AssociatePhoneNumbersWithVoiceConnectorOutput::phone_number_errors): <p>If the action fails for one or more of the phone numbers in the request, a list of the phone numbers is returned, along with error codes and error messages.</p>
    /// - On failure, responds with [`SdkError<AssociatePhoneNumbersWithVoiceConnectorError>`](crate::operation::associate_phone_numbers_with_voice_connector::AssociatePhoneNumbersWithVoiceConnectorError)
    pub fn associate_phone_numbers_with_voice_connector(
        &self,
    ) -> crate::operation::associate_phone_numbers_with_voice_connector::builders::AssociatePhoneNumbersWithVoiceConnectorFluentBuilder {
        crate::operation::associate_phone_numbers_with_voice_connector::builders::AssociatePhoneNumbersWithVoiceConnectorFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
