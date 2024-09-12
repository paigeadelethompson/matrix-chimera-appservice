// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteVoiceConnectorOrigination`](crate::operation::delete_voice_connector_origination::builders::DeleteVoiceConnectorOriginationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`voice_connector_id(impl Into<String>)`](crate::operation::delete_voice_connector_origination::builders::DeleteVoiceConnectorOriginationFluentBuilder::voice_connector_id) / [`set_voice_connector_id(Option<String>)`](crate::operation::delete_voice_connector_origination::builders::DeleteVoiceConnectorOriginationFluentBuilder::set_voice_connector_id):<br>required: **true**<br><p>The Voice Connector ID.</p><br>
    /// - On success, responds with [`DeleteVoiceConnectorOriginationOutput`](crate::operation::delete_voice_connector_origination::DeleteVoiceConnectorOriginationOutput)
    /// - On failure, responds with [`SdkError<DeleteVoiceConnectorOriginationError>`](crate::operation::delete_voice_connector_origination::DeleteVoiceConnectorOriginationError)
    pub fn delete_voice_connector_origination(
        &self,
    ) -> crate::operation::delete_voice_connector_origination::builders::DeleteVoiceConnectorOriginationFluentBuilder {
        crate::operation::delete_voice_connector_origination::builders::DeleteVoiceConnectorOriginationFluentBuilder::new(self.handle.clone())
    }
}
