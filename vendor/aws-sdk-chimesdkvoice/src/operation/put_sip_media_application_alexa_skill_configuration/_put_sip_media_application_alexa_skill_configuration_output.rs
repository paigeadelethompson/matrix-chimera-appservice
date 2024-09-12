// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct PutSipMediaApplicationAlexaSkillConfigurationOutput {
    /// <p>Returns the Alexa Skill configuration.</p>
    pub sip_media_application_alexa_skill_configuration: ::std::option::Option<crate::types::SipMediaApplicationAlexaSkillConfiguration>,
    _request_id: Option<String>,
}
impl PutSipMediaApplicationAlexaSkillConfigurationOutput {
    /// <p>Returns the Alexa Skill configuration.</p>
    pub fn sip_media_application_alexa_skill_configuration(
        &self,
    ) -> ::std::option::Option<&crate::types::SipMediaApplicationAlexaSkillConfiguration> {
        self.sip_media_application_alexa_skill_configuration.as_ref()
    }
}
impl ::aws_types::request_id::RequestId for PutSipMediaApplicationAlexaSkillConfigurationOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl PutSipMediaApplicationAlexaSkillConfigurationOutput {
    /// Creates a new builder-style object to manufacture [`PutSipMediaApplicationAlexaSkillConfigurationOutput`](crate::operation::put_sip_media_application_alexa_skill_configuration::PutSipMediaApplicationAlexaSkillConfigurationOutput).
    pub fn builder(
    ) -> crate::operation::put_sip_media_application_alexa_skill_configuration::builders::PutSipMediaApplicationAlexaSkillConfigurationOutputBuilder
    {
        crate::operation::put_sip_media_application_alexa_skill_configuration::builders::PutSipMediaApplicationAlexaSkillConfigurationOutputBuilder::default()
    }
}

/// A builder for [`PutSipMediaApplicationAlexaSkillConfigurationOutput`](crate::operation::put_sip_media_application_alexa_skill_configuration::PutSipMediaApplicationAlexaSkillConfigurationOutput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct PutSipMediaApplicationAlexaSkillConfigurationOutputBuilder {
    pub(crate) sip_media_application_alexa_skill_configuration: ::std::option::Option<crate::types::SipMediaApplicationAlexaSkillConfiguration>,
    _request_id: Option<String>,
}
impl PutSipMediaApplicationAlexaSkillConfigurationOutputBuilder {
    /// <p>Returns the Alexa Skill configuration.</p>
    pub fn sip_media_application_alexa_skill_configuration(mut self, input: crate::types::SipMediaApplicationAlexaSkillConfiguration) -> Self {
        self.sip_media_application_alexa_skill_configuration = ::std::option::Option::Some(input);
        self
    }
    /// <p>Returns the Alexa Skill configuration.</p>
    pub fn set_sip_media_application_alexa_skill_configuration(
        mut self,
        input: ::std::option::Option<crate::types::SipMediaApplicationAlexaSkillConfiguration>,
    ) -> Self {
        self.sip_media_application_alexa_skill_configuration = input;
        self
    }
    /// <p>Returns the Alexa Skill configuration.</p>
    pub fn get_sip_media_application_alexa_skill_configuration(
        &self,
    ) -> &::std::option::Option<crate::types::SipMediaApplicationAlexaSkillConfiguration> {
        &self.sip_media_application_alexa_skill_configuration
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`PutSipMediaApplicationAlexaSkillConfigurationOutput`](crate::operation::put_sip_media_application_alexa_skill_configuration::PutSipMediaApplicationAlexaSkillConfigurationOutput).
    pub fn build(self) -> crate::operation::put_sip_media_application_alexa_skill_configuration::PutSipMediaApplicationAlexaSkillConfigurationOutput {
        crate::operation::put_sip_media_application_alexa_skill_configuration::PutSipMediaApplicationAlexaSkillConfigurationOutput {
            sip_media_application_alexa_skill_configuration: self.sip_media_application_alexa_skill_configuration,
            _request_id: self._request_id,
        }
    }
}
