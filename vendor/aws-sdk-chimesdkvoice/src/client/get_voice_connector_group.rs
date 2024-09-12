// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetVoiceConnectorGroup`](crate::operation::get_voice_connector_group::builders::GetVoiceConnectorGroupFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`voice_connector_group_id(impl Into<String>)`](crate::operation::get_voice_connector_group::builders::GetVoiceConnectorGroupFluentBuilder::voice_connector_group_id) / [`set_voice_connector_group_id(Option<String>)`](crate::operation::get_voice_connector_group::builders::GetVoiceConnectorGroupFluentBuilder::set_voice_connector_group_id):<br>required: **true**<br><p>The Voice Connector group ID.</p><br>
    /// - On success, responds with [`GetVoiceConnectorGroupOutput`](crate::operation::get_voice_connector_group::GetVoiceConnectorGroupOutput) with field(s):
    ///   - [`voice_connector_group(Option<VoiceConnectorGroup>)`](crate::operation::get_voice_connector_group::GetVoiceConnectorGroupOutput::voice_connector_group): <p>The details of the Voice Connector group.</p>
    /// - On failure, responds with [`SdkError<GetVoiceConnectorGroupError>`](crate::operation::get_voice_connector_group::GetVoiceConnectorGroupError)
    pub fn get_voice_connector_group(&self) -> crate::operation::get_voice_connector_group::builders::GetVoiceConnectorGroupFluentBuilder {
        crate::operation::get_voice_connector_group::builders::GetVoiceConnectorGroupFluentBuilder::new(self.handle.clone())
    }
}