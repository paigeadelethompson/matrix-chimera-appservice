// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateProxySession`](crate::operation::create_proxy_session::builders::CreateProxySessionFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`voice_connector_id(impl Into<String>)`](crate::operation::create_proxy_session::builders::CreateProxySessionFluentBuilder::voice_connector_id) / [`set_voice_connector_id(Option<String>)`](crate::operation::create_proxy_session::builders::CreateProxySessionFluentBuilder::set_voice_connector_id):<br>required: **true**<br><p>The Voice Connector ID.</p><br>
    ///   - [`participant_phone_numbers(impl Into<String>)`](crate::operation::create_proxy_session::builders::CreateProxySessionFluentBuilder::participant_phone_numbers) / [`set_participant_phone_numbers(Option<Vec::<String>>)`](crate::operation::create_proxy_session::builders::CreateProxySessionFluentBuilder::set_participant_phone_numbers):<br>required: **true**<br><p>The participant phone numbers.</p><br>
    ///   - [`name(impl Into<String>)`](crate::operation::create_proxy_session::builders::CreateProxySessionFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::create_proxy_session::builders::CreateProxySessionFluentBuilder::set_name):<br>required: **false**<br><p>The name of the proxy session.</p><br>
    ///   - [`expiry_minutes(i32)`](crate::operation::create_proxy_session::builders::CreateProxySessionFluentBuilder::expiry_minutes) / [`set_expiry_minutes(Option<i32>)`](crate::operation::create_proxy_session::builders::CreateProxySessionFluentBuilder::set_expiry_minutes):<br>required: **false**<br><p>The number of minutes allowed for the proxy session.</p><br>
    ///   - [`capabilities(Capability)`](crate::operation::create_proxy_session::builders::CreateProxySessionFluentBuilder::capabilities) / [`set_capabilities(Option<Vec::<Capability>>)`](crate::operation::create_proxy_session::builders::CreateProxySessionFluentBuilder::set_capabilities):<br>required: **true**<br><p>The proxy session's capabilities.</p><br>
    ///   - [`number_selection_behavior(NumberSelectionBehavior)`](crate::operation::create_proxy_session::builders::CreateProxySessionFluentBuilder::number_selection_behavior) / [`set_number_selection_behavior(Option<NumberSelectionBehavior>)`](crate::operation::create_proxy_session::builders::CreateProxySessionFluentBuilder::set_number_selection_behavior):<br>required: **false**<br><p>The preference for proxy phone number reuse, or stickiness, between the same participants across sessions.</p><br>
    ///   - [`geo_match_level(GeoMatchLevel)`](crate::operation::create_proxy_session::builders::CreateProxySessionFluentBuilder::geo_match_level) / [`set_geo_match_level(Option<GeoMatchLevel>)`](crate::operation::create_proxy_session::builders::CreateProxySessionFluentBuilder::set_geo_match_level):<br>required: **false**<br><p>The preference for matching the country or area code of the proxy phone number with that of the first participant.</p><br>
    ///   - [`geo_match_params(GeoMatchParams)`](crate::operation::create_proxy_session::builders::CreateProxySessionFluentBuilder::geo_match_params) / [`set_geo_match_params(Option<GeoMatchParams>)`](crate::operation::create_proxy_session::builders::CreateProxySessionFluentBuilder::set_geo_match_params):<br>required: **false**<br><p>The country and area code for the proxy phone number.</p><br>
    /// - On success, responds with [`CreateProxySessionOutput`](crate::operation::create_proxy_session::CreateProxySessionOutput) with field(s):
    ///   - [`proxy_session(Option<ProxySession>)`](crate::operation::create_proxy_session::CreateProxySessionOutput::proxy_session): <p>The proxy session details.</p>
    /// - On failure, responds with [`SdkError<CreateProxySessionError>`](crate::operation::create_proxy_session::CreateProxySessionError)
    pub fn create_proxy_session(&self) -> crate::operation::create_proxy_session::builders::CreateProxySessionFluentBuilder {
        crate::operation::create_proxy_session::builders::CreateProxySessionFluentBuilder::new(self.handle.clone())
    }
}