// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateSipMediaApplication`](crate::operation::create_sip_media_application::builders::CreateSipMediaApplicationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`aws_region(impl Into<String>)`](crate::operation::create_sip_media_application::builders::CreateSipMediaApplicationFluentBuilder::aws_region) / [`set_aws_region(Option<String>)`](crate::operation::create_sip_media_application::builders::CreateSipMediaApplicationFluentBuilder::set_aws_region):<br>required: **true**<br><p>The AWS Region assigned to the SIP media application.</p><br>
    ///   - [`name(impl Into<String>)`](crate::operation::create_sip_media_application::builders::CreateSipMediaApplicationFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::create_sip_media_application::builders::CreateSipMediaApplicationFluentBuilder::set_name):<br>required: **true**<br><p>The SIP media application's name.</p><br>
    ///   - [`endpoints(SipMediaApplicationEndpoint)`](crate::operation::create_sip_media_application::builders::CreateSipMediaApplicationFluentBuilder::endpoints) / [`set_endpoints(Option<Vec::<SipMediaApplicationEndpoint>>)`](crate::operation::create_sip_media_application::builders::CreateSipMediaApplicationFluentBuilder::set_endpoints):<br>required: **true**<br><p>List of endpoints (Lambda ARNs) specified for the SIP media application.</p><br>
    ///   - [`tags(Tag)`](crate::operation::create_sip_media_application::builders::CreateSipMediaApplicationFluentBuilder::tags) / [`set_tags(Option<Vec::<Tag>>)`](crate::operation::create_sip_media_application::builders::CreateSipMediaApplicationFluentBuilder::set_tags):<br>required: **false**<br><p>The tags assigned to the SIP media application.</p><br>
    /// - On success, responds with [`CreateSipMediaApplicationOutput`](crate::operation::create_sip_media_application::CreateSipMediaApplicationOutput) with field(s):
    ///   - [`sip_media_application(Option<SipMediaApplication>)`](crate::operation::create_sip_media_application::CreateSipMediaApplicationOutput::sip_media_application): <p>The SIP media application details.</p>
    /// - On failure, responds with [`SdkError<CreateSipMediaApplicationError>`](crate::operation::create_sip_media_application::CreateSipMediaApplicationError)
    pub fn create_sip_media_application(&self) -> crate::operation::create_sip_media_application::builders::CreateSipMediaApplicationFluentBuilder {
        crate::operation::create_sip_media_application::builders::CreateSipMediaApplicationFluentBuilder::new(self.handle.clone())
    }
}
