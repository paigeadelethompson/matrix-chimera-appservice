// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct PutSipMediaApplicationLoggingConfigurationInput {
    /// <p>The SIP media application ID.</p>
    pub sip_media_application_id: ::std::option::Option<::std::string::String>,
    /// <p>The logging configuration for the specified SIP media application.</p>
    pub sip_media_application_logging_configuration: ::std::option::Option<crate::types::SipMediaApplicationLoggingConfiguration>,
}
impl PutSipMediaApplicationLoggingConfigurationInput {
    /// <p>The SIP media application ID.</p>
    pub fn sip_media_application_id(&self) -> ::std::option::Option<&str> {
        self.sip_media_application_id.as_deref()
    }
    /// <p>The logging configuration for the specified SIP media application.</p>
    pub fn sip_media_application_logging_configuration(&self) -> ::std::option::Option<&crate::types::SipMediaApplicationLoggingConfiguration> {
        self.sip_media_application_logging_configuration.as_ref()
    }
}
impl PutSipMediaApplicationLoggingConfigurationInput {
    /// Creates a new builder-style object to manufacture [`PutSipMediaApplicationLoggingConfigurationInput`](crate::operation::put_sip_media_application_logging_configuration::PutSipMediaApplicationLoggingConfigurationInput).
    pub fn builder(
    ) -> crate::operation::put_sip_media_application_logging_configuration::builders::PutSipMediaApplicationLoggingConfigurationInputBuilder {
        crate::operation::put_sip_media_application_logging_configuration::builders::PutSipMediaApplicationLoggingConfigurationInputBuilder::default()
    }
}

/// A builder for [`PutSipMediaApplicationLoggingConfigurationInput`](crate::operation::put_sip_media_application_logging_configuration::PutSipMediaApplicationLoggingConfigurationInput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct PutSipMediaApplicationLoggingConfigurationInputBuilder {
    pub(crate) sip_media_application_id: ::std::option::Option<::std::string::String>,
    pub(crate) sip_media_application_logging_configuration: ::std::option::Option<crate::types::SipMediaApplicationLoggingConfiguration>,
}
impl PutSipMediaApplicationLoggingConfigurationInputBuilder {
    /// <p>The SIP media application ID.</p>
    /// This field is required.
    pub fn sip_media_application_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.sip_media_application_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The SIP media application ID.</p>
    pub fn set_sip_media_application_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.sip_media_application_id = input;
        self
    }
    /// <p>The SIP media application ID.</p>
    pub fn get_sip_media_application_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.sip_media_application_id
    }
    /// <p>The logging configuration for the specified SIP media application.</p>
    pub fn sip_media_application_logging_configuration(mut self, input: crate::types::SipMediaApplicationLoggingConfiguration) -> Self {
        self.sip_media_application_logging_configuration = ::std::option::Option::Some(input);
        self
    }
    /// <p>The logging configuration for the specified SIP media application.</p>
    pub fn set_sip_media_application_logging_configuration(
        mut self,
        input: ::std::option::Option<crate::types::SipMediaApplicationLoggingConfiguration>,
    ) -> Self {
        self.sip_media_application_logging_configuration = input;
        self
    }
    /// <p>The logging configuration for the specified SIP media application.</p>
    pub fn get_sip_media_application_logging_configuration(&self) -> &::std::option::Option<crate::types::SipMediaApplicationLoggingConfiguration> {
        &self.sip_media_application_logging_configuration
    }
    /// Consumes the builder and constructs a [`PutSipMediaApplicationLoggingConfigurationInput`](crate::operation::put_sip_media_application_logging_configuration::PutSipMediaApplicationLoggingConfigurationInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::put_sip_media_application_logging_configuration::PutSipMediaApplicationLoggingConfigurationInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::put_sip_media_application_logging_configuration::PutSipMediaApplicationLoggingConfigurationInput {
                sip_media_application_id: self.sip_media_application_id,
                sip_media_application_logging_configuration: self.sip_media_application_logging_configuration,
            },
        )
    }
}