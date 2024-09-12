// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`PutSipMediaApplicationAlexaSkillConfiguration`](crate::operation::put_sip_media_application_alexa_skill_configuration::builders::PutSipMediaApplicationAlexaSkillConfigurationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`sip_media_application_id(impl Into<String>)`](crate::operation::put_sip_media_application_alexa_skill_configuration::builders::PutSipMediaApplicationAlexaSkillConfigurationFluentBuilder::sip_media_application_id) / [`set_sip_media_application_id(Option<String>)`](crate::operation::put_sip_media_application_alexa_skill_configuration::builders::PutSipMediaApplicationAlexaSkillConfigurationFluentBuilder::set_sip_media_application_id):<br>required: **true**<br><p>The SIP media application ID.</p><br>
    ///   - [`sip_media_application_alexa_skill_configuration(SipMediaApplicationAlexaSkillConfiguration)`](crate::operation::put_sip_media_application_alexa_skill_configuration::builders::PutSipMediaApplicationAlexaSkillConfigurationFluentBuilder::sip_media_application_alexa_skill_configuration) / [`set_sip_media_application_alexa_skill_configuration(Option<SipMediaApplicationAlexaSkillConfiguration>)`](crate::operation::put_sip_media_application_alexa_skill_configuration::builders::PutSipMediaApplicationAlexaSkillConfigurationFluentBuilder::set_sip_media_application_alexa_skill_configuration):<br>required: **false**<br><p>The Alexa Skill configuration.</p><br>
    /// - On success, responds with [`PutSipMediaApplicationAlexaSkillConfigurationOutput`](crate::operation::put_sip_media_application_alexa_skill_configuration::PutSipMediaApplicationAlexaSkillConfigurationOutput) with field(s):
    ///   - [`sip_media_application_alexa_skill_configuration(Option<SipMediaApplicationAlexaSkillConfiguration>)`](crate::operation::put_sip_media_application_alexa_skill_configuration::PutSipMediaApplicationAlexaSkillConfigurationOutput::sip_media_application_alexa_skill_configuration): <p>Returns the Alexa Skill configuration.</p>
    /// - On failure, responds with [`SdkError<PutSipMediaApplicationAlexaSkillConfigurationError>`](crate::operation::put_sip_media_application_alexa_skill_configuration::PutSipMediaApplicationAlexaSkillConfigurationError)
    #[deprecated(
        note = "Due to changes made by the Amazon Alexa service, this API is no longer available for use. For more information, refer to the Alexa Smart Properties page(https://developer.amazon.com/en-US/alexa/alexasmartproperties)."
    )]
    pub fn put_sip_media_application_alexa_skill_configuration(
        &self,
    ) -> crate::operation::put_sip_media_application_alexa_skill_configuration::builders::PutSipMediaApplicationAlexaSkillConfigurationFluentBuilder
    {
        crate::operation::put_sip_media_application_alexa_skill_configuration::builders::PutSipMediaApplicationAlexaSkillConfigurationFluentBuilder::new(self.handle.clone())
    }
}
