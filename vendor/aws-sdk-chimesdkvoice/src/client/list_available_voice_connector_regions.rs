// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListAvailableVoiceConnectorRegions`](crate::operation::list_available_voice_connector_regions::builders::ListAvailableVoiceConnectorRegionsFluentBuilder) operation.
    ///
    /// - The fluent builder takes no input, just [`send`](crate::operation::list_available_voice_connector_regions::builders::ListAvailableVoiceConnectorRegionsFluentBuilder::send) it.
    /// - On success, responds with [`ListAvailableVoiceConnectorRegionsOutput`](crate::operation::list_available_voice_connector_regions::ListAvailableVoiceConnectorRegionsOutput) with field(s):
    ///   - [`voice_connector_regions(Option<Vec::<VoiceConnectorAwsRegion>>)`](crate::operation::list_available_voice_connector_regions::ListAvailableVoiceConnectorRegionsOutput::voice_connector_regions): <p>The list of AWS Regions.</p>
    /// - On failure, responds with [`SdkError<ListAvailableVoiceConnectorRegionsError>`](crate::operation::list_available_voice_connector_regions::ListAvailableVoiceConnectorRegionsError)
    pub fn list_available_voice_connector_regions(
        &self,
    ) -> crate::operation::list_available_voice_connector_regions::builders::ListAvailableVoiceConnectorRegionsFluentBuilder {
        crate::operation::list_available_voice_connector_regions::builders::ListAvailableVoiceConnectorRegionsFluentBuilder::new(self.handle.clone())
    }
}
