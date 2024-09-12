// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetVoiceConnector`](crate::operation::get_voice_connector::builders::GetVoiceConnectorFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`voice_connector_id(impl Into<String>)`](crate::operation::get_voice_connector::builders::GetVoiceConnectorFluentBuilder::voice_connector_id) / [`set_voice_connector_id(Option<String>)`](crate::operation::get_voice_connector::builders::GetVoiceConnectorFluentBuilder::set_voice_connector_id):<br>required: **true**<br><p>The Voice Connector ID.</p><br>
    /// - On success, responds with [`GetVoiceConnectorOutput`](crate::operation::get_voice_connector::GetVoiceConnectorOutput) with field(s):
    ///   - [`voice_connector(Option<VoiceConnector>)`](crate::operation::get_voice_connector::GetVoiceConnectorOutput::voice_connector): <p>The Voice Connector details.</p>
    /// - On failure, responds with [`SdkError<GetVoiceConnectorError>`](crate::operation::get_voice_connector::GetVoiceConnectorError)
    pub fn get_voice_connector(&self) -> crate::operation::get_voice_connector::builders::GetVoiceConnectorFluentBuilder {
        crate::operation::get_voice_connector::builders::GetVoiceConnectorFluentBuilder::new(self.handle.clone())
    }
}