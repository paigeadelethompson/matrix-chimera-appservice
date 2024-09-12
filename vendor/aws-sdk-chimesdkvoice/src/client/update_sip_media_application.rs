// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateSipMediaApplication`](crate::operation::update_sip_media_application::builders::UpdateSipMediaApplicationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`sip_media_application_id(impl Into<String>)`](crate::operation::update_sip_media_application::builders::UpdateSipMediaApplicationFluentBuilder::sip_media_application_id) / [`set_sip_media_application_id(Option<String>)`](crate::operation::update_sip_media_application::builders::UpdateSipMediaApplicationFluentBuilder::set_sip_media_application_id):<br>required: **true**<br><p>The SIP media application ID.</p><br>
    ///   - [`name(impl Into<String>)`](crate::operation::update_sip_media_application::builders::UpdateSipMediaApplicationFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::update_sip_media_application::builders::UpdateSipMediaApplicationFluentBuilder::set_name):<br>required: **false**<br><p>The new name for the specified SIP media application.</p><br>
    ///   - [`endpoints(SipMediaApplicationEndpoint)`](crate::operation::update_sip_media_application::builders::UpdateSipMediaApplicationFluentBuilder::endpoints) / [`set_endpoints(Option<Vec::<SipMediaApplicationEndpoint>>)`](crate::operation::update_sip_media_application::builders::UpdateSipMediaApplicationFluentBuilder::set_endpoints):<br>required: **false**<br><p>The new set of endpoints for the specified SIP media application.</p><br>
    /// - On success, responds with [`UpdateSipMediaApplicationOutput`](crate::operation::update_sip_media_application::UpdateSipMediaApplicationOutput) with field(s):
    ///   - [`sip_media_application(Option<SipMediaApplication>)`](crate::operation::update_sip_media_application::UpdateSipMediaApplicationOutput::sip_media_application): <p>The updated SIP media application’s details.</p>
    /// - On failure, responds with [`SdkError<UpdateSipMediaApplicationError>`](crate::operation::update_sip_media_application::UpdateSipMediaApplicationError)
    pub fn update_sip_media_application(&self) -> crate::operation::update_sip_media_application::builders::UpdateSipMediaApplicationFluentBuilder {
        crate::operation::update_sip_media_application::builders::UpdateSipMediaApplicationFluentBuilder::new(self.handle.clone())
    }
}