// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetVoiceConnectorTerminationHealth`](crate::operation::get_voice_connector_termination_health::builders::GetVoiceConnectorTerminationHealthFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`voice_connector_id(impl Into<String>)`](crate::operation::get_voice_connector_termination_health::builders::GetVoiceConnectorTerminationHealthFluentBuilder::voice_connector_id) / [`set_voice_connector_id(Option<String>)`](crate::operation::get_voice_connector_termination_health::builders::GetVoiceConnectorTerminationHealthFluentBuilder::set_voice_connector_id):<br>required: **true**<br><p>The Voice Connector ID.</p><br>
    /// - On success, responds with [`GetVoiceConnectorTerminationHealthOutput`](crate::operation::get_voice_connector_termination_health::GetVoiceConnectorTerminationHealthOutput) with field(s):
    ///   - [`termination_health(Option<TerminationHealth>)`](crate::operation::get_voice_connector_termination_health::GetVoiceConnectorTerminationHealthOutput::termination_health): <p>The termination health details.</p>
    /// - On failure, responds with [`SdkError<GetVoiceConnectorTerminationHealthError>`](crate::operation::get_voice_connector_termination_health::GetVoiceConnectorTerminationHealthError)
    pub fn get_voice_connector_termination_health(
        &self,
    ) -> crate::operation::get_voice_connector_termination_health::builders::GetVoiceConnectorTerminationHealthFluentBuilder {
        crate::operation::get_voice_connector_termination_health::builders::GetVoiceConnectorTerminationHealthFluentBuilder::new(self.handle.clone())
    }
}
