// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListVoiceConnectorTerminationCredentials`](crate::operation::list_voice_connector_termination_credentials::builders::ListVoiceConnectorTerminationCredentialsFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`voice_connector_id(impl Into<String>)`](crate::operation::list_voice_connector_termination_credentials::builders::ListVoiceConnectorTerminationCredentialsFluentBuilder::voice_connector_id) / [`set_voice_connector_id(Option<String>)`](crate::operation::list_voice_connector_termination_credentials::builders::ListVoiceConnectorTerminationCredentialsFluentBuilder::set_voice_connector_id):<br>required: **true**<br><p>The Voice Connector ID.</p><br>
    /// - On success, responds with [`ListVoiceConnectorTerminationCredentialsOutput`](crate::operation::list_voice_connector_termination_credentials::ListVoiceConnectorTerminationCredentialsOutput) with field(s):
    ///   - [`usernames(Option<Vec::<String>>)`](crate::operation::list_voice_connector_termination_credentials::ListVoiceConnectorTerminationCredentialsOutput::usernames): <p>A list of user names.</p>
    /// - On failure, responds with [`SdkError<ListVoiceConnectorTerminationCredentialsError>`](crate::operation::list_voice_connector_termination_credentials::ListVoiceConnectorTerminationCredentialsError)
    pub fn list_voice_connector_termination_credentials(
        &self,
    ) -> crate::operation::list_voice_connector_termination_credentials::builders::ListVoiceConnectorTerminationCredentialsFluentBuilder {
        crate::operation::list_voice_connector_termination_credentials::builders::ListVoiceConnectorTerminationCredentialsFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
