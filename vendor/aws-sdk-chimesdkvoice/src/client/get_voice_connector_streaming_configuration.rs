// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetVoiceConnectorStreamingConfiguration`](crate::operation::get_voice_connector_streaming_configuration::builders::GetVoiceConnectorStreamingConfigurationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`voice_connector_id(impl Into<String>)`](crate::operation::get_voice_connector_streaming_configuration::builders::GetVoiceConnectorStreamingConfigurationFluentBuilder::voice_connector_id) / [`set_voice_connector_id(Option<String>)`](crate::operation::get_voice_connector_streaming_configuration::builders::GetVoiceConnectorStreamingConfigurationFluentBuilder::set_voice_connector_id):<br>required: **true**<br><p>The Voice Connector ID.</p><br>
    /// - On success, responds with [`GetVoiceConnectorStreamingConfigurationOutput`](crate::operation::get_voice_connector_streaming_configuration::GetVoiceConnectorStreamingConfigurationOutput) with field(s):
    ///   - [`streaming_configuration(Option<StreamingConfiguration>)`](crate::operation::get_voice_connector_streaming_configuration::GetVoiceConnectorStreamingConfigurationOutput::streaming_configuration): <p>The details of the streaming configuration.</p>
    /// - On failure, responds with [`SdkError<GetVoiceConnectorStreamingConfigurationError>`](crate::operation::get_voice_connector_streaming_configuration::GetVoiceConnectorStreamingConfigurationError)
    pub fn get_voice_connector_streaming_configuration(
        &self,
    ) -> crate::operation::get_voice_connector_streaming_configuration::builders::GetVoiceConnectorStreamingConfigurationFluentBuilder {
        crate::operation::get_voice_connector_streaming_configuration::builders::GetVoiceConnectorStreamingConfigurationFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
