// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListVoiceConnectors`](crate::operation::list_voice_connectors::builders::ListVoiceConnectorsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_voice_connectors::builders::ListVoiceConnectorsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`next_token(impl Into<String>)`](crate::operation::list_voice_connectors::builders::ListVoiceConnectorsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_voice_connectors::builders::ListVoiceConnectorsFluentBuilder::set_next_token):<br>required: **false**<br><p>The token used to return the next page of results.</p><br>
    ///   - [`max_results(i32)`](crate::operation::list_voice_connectors::builders::ListVoiceConnectorsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_voice_connectors::builders::ListVoiceConnectorsFluentBuilder::set_max_results):<br>required: **false**<br><p>The maximum number of results to return in a single call.</p><br>
    /// - On success, responds with [`ListVoiceConnectorsOutput`](crate::operation::list_voice_connectors::ListVoiceConnectorsOutput) with field(s):
    ///   - [`voice_connectors(Option<Vec::<VoiceConnector>>)`](crate::operation::list_voice_connectors::ListVoiceConnectorsOutput::voice_connectors): <p>The details of the Voice Connectors.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_voice_connectors::ListVoiceConnectorsOutput::next_token): <p>The token used to return the next page of results.</p>
    /// - On failure, responds with [`SdkError<ListVoiceConnectorsError>`](crate::operation::list_voice_connectors::ListVoiceConnectorsError)
    pub fn list_voice_connectors(&self) -> crate::operation::list_voice_connectors::builders::ListVoiceConnectorsFluentBuilder {
        crate::operation::list_voice_connectors::builders::ListVoiceConnectorsFluentBuilder::new(self.handle.clone())
    }
}
