// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetGlobalSettings`](crate::operation::get_global_settings::builders::GetGlobalSettingsFluentBuilder) operation.
    ///
    /// - The fluent builder takes no input, just [`send`](crate::operation::get_global_settings::builders::GetGlobalSettingsFluentBuilder::send) it.
    /// - On success, responds with [`GetGlobalSettingsOutput`](crate::operation::get_global_settings::GetGlobalSettingsOutput) with field(s):
    ///   - [`voice_connector(Option<VoiceConnectorSettings>)`](crate::operation::get_global_settings::GetGlobalSettingsOutput::voice_connector): <p>The Voice Connector settings.</p>
    /// - On failure, responds with [`SdkError<GetGlobalSettingsError>`](crate::operation::get_global_settings::GetGlobalSettingsError)
    pub fn get_global_settings(&self) -> crate::operation::get_global_settings::builders::GetGlobalSettingsFluentBuilder {
        crate::operation::get_global_settings::builders::GetGlobalSettingsFluentBuilder::new(self.handle.clone())
    }
}