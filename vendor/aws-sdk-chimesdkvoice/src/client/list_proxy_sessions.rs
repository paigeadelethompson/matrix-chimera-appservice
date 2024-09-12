// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListProxySessions`](crate::operation::list_proxy_sessions::builders::ListProxySessionsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_proxy_sessions::builders::ListProxySessionsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`voice_connector_id(impl Into<String>)`](crate::operation::list_proxy_sessions::builders::ListProxySessionsFluentBuilder::voice_connector_id) / [`set_voice_connector_id(Option<String>)`](crate::operation::list_proxy_sessions::builders::ListProxySessionsFluentBuilder::set_voice_connector_id):<br>required: **true**<br><p>The Voice Connector ID.</p><br>
    ///   - [`status(ProxySessionStatus)`](crate::operation::list_proxy_sessions::builders::ListProxySessionsFluentBuilder::status) / [`set_status(Option<ProxySessionStatus>)`](crate::operation::list_proxy_sessions::builders::ListProxySessionsFluentBuilder::set_status):<br>required: **false**<br><p>The proxy session status.</p><br>
    ///   - [`next_token(impl Into<String>)`](crate::operation::list_proxy_sessions::builders::ListProxySessionsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_proxy_sessions::builders::ListProxySessionsFluentBuilder::set_next_token):<br>required: **false**<br><p>The token used to retrieve the next page of results.</p><br>
    ///   - [`max_results(i32)`](crate::operation::list_proxy_sessions::builders::ListProxySessionsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_proxy_sessions::builders::ListProxySessionsFluentBuilder::set_max_results):<br>required: **false**<br><p>The maximum number of results to return in a single call.</p><br>
    /// - On success, responds with [`ListProxySessionsOutput`](crate::operation::list_proxy_sessions::ListProxySessionsOutput) with field(s):
    ///   - [`proxy_sessions(Option<Vec::<ProxySession>>)`](crate::operation::list_proxy_sessions::ListProxySessionsOutput::proxy_sessions): <p>The proxy sessions' details.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_proxy_sessions::ListProxySessionsOutput::next_token): <p>The token used to retrieve the next page of results.</p>
    /// - On failure, responds with [`SdkError<ListProxySessionsError>`](crate::operation::list_proxy_sessions::ListProxySessionsError)
    pub fn list_proxy_sessions(&self) -> crate::operation::list_proxy_sessions::builders::ListProxySessionsFluentBuilder {
        crate::operation::list_proxy_sessions::builders::ListProxySessionsFluentBuilder::new(self.handle.clone())
    }
}
