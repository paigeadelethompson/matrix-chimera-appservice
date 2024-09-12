// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`StartVoiceToneAnalysisTask`](crate::operation::start_voice_tone_analysis_task::builders::StartVoiceToneAnalysisTaskFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`voice_connector_id(impl Into<String>)`](crate::operation::start_voice_tone_analysis_task::builders::StartVoiceToneAnalysisTaskFluentBuilder::voice_connector_id) / [`set_voice_connector_id(Option<String>)`](crate::operation::start_voice_tone_analysis_task::builders::StartVoiceToneAnalysisTaskFluentBuilder::set_voice_connector_id):<br>required: **true**<br><p>The Voice Connector ID.</p><br>
    ///   - [`transaction_id(impl Into<String>)`](crate::operation::start_voice_tone_analysis_task::builders::StartVoiceToneAnalysisTaskFluentBuilder::transaction_id) / [`set_transaction_id(Option<String>)`](crate::operation::start_voice_tone_analysis_task::builders::StartVoiceToneAnalysisTaskFluentBuilder::set_transaction_id):<br>required: **true**<br><p>The transaction ID.</p><br>
    ///   - [`language_code(LanguageCode)`](crate::operation::start_voice_tone_analysis_task::builders::StartVoiceToneAnalysisTaskFluentBuilder::language_code) / [`set_language_code(Option<LanguageCode>)`](crate::operation::start_voice_tone_analysis_task::builders::StartVoiceToneAnalysisTaskFluentBuilder::set_language_code):<br>required: **true**<br><p>The language code.</p><br>
    ///   - [`client_request_token(impl Into<String>)`](crate::operation::start_voice_tone_analysis_task::builders::StartVoiceToneAnalysisTaskFluentBuilder::client_request_token) / [`set_client_request_token(Option<String>)`](crate::operation::start_voice_tone_analysis_task::builders::StartVoiceToneAnalysisTaskFluentBuilder::set_client_request_token):<br>required: **false**<br><p>The unique identifier for the client request. Use a different token for different voice tone analysis tasks.</p><br>
    /// - On success, responds with [`StartVoiceToneAnalysisTaskOutput`](crate::operation::start_voice_tone_analysis_task::StartVoiceToneAnalysisTaskOutput) with field(s):
    ///   - [`voice_tone_analysis_task(Option<VoiceToneAnalysisTask>)`](crate::operation::start_voice_tone_analysis_task::StartVoiceToneAnalysisTaskOutput::voice_tone_analysis_task): <p>The details of the voice tone analysis task.</p>
    /// - On failure, responds with [`SdkError<StartVoiceToneAnalysisTaskError>`](crate::operation::start_voice_tone_analysis_task::StartVoiceToneAnalysisTaskError)
    pub fn start_voice_tone_analysis_task(
        &self,
    ) -> crate::operation::start_voice_tone_analysis_task::builders::StartVoiceToneAnalysisTaskFluentBuilder {
        crate::operation::start_voice_tone_analysis_task::builders::StartVoiceToneAnalysisTaskFluentBuilder::new(self.handle.clone())
    }
}