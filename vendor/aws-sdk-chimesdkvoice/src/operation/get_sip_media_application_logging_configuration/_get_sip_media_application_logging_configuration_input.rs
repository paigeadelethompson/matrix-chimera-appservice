// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetSipMediaApplicationLoggingConfigurationInput {
    /// <p>The SIP media application ID.</p>
    pub sip_media_application_id: ::std::option::Option<::std::string::String>,
}
impl GetSipMediaApplicationLoggingConfigurationInput {
    /// <p>The SIP media application ID.</p>
    pub fn sip_media_application_id(&self) -> ::std::option::Option<&str> {
        self.sip_media_application_id.as_deref()
    }
}
impl GetSipMediaApplicationLoggingConfigurationInput {
    /// Creates a new builder-style object to manufacture [`GetSipMediaApplicationLoggingConfigurationInput`](crate::operation::get_sip_media_application_logging_configuration::GetSipMediaApplicationLoggingConfigurationInput).
    pub fn builder(
    ) -> crate::operation::get_sip_media_application_logging_configuration::builders::GetSipMediaApplicationLoggingConfigurationInputBuilder {
        crate::operation::get_sip_media_application_logging_configuration::builders::GetSipMediaApplicationLoggingConfigurationInputBuilder::default()
    }
}

/// A builder for [`GetSipMediaApplicationLoggingConfigurationInput`](crate::operation::get_sip_media_application_logging_configuration::GetSipMediaApplicationLoggingConfigurationInput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct GetSipMediaApplicationLoggingConfigurationInputBuilder {
    pub(crate) sip_media_application_id: ::std::option::Option<::std::string::String>,
}
impl GetSipMediaApplicationLoggingConfigurationInputBuilder {
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
    /// Consumes the builder and constructs a [`GetSipMediaApplicationLoggingConfigurationInput`](crate::operation::get_sip_media_application_logging_configuration::GetSipMediaApplicationLoggingConfigurationInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::get_sip_media_application_logging_configuration::GetSipMediaApplicationLoggingConfigurationInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::get_sip_media_application_logging_configuration::GetSipMediaApplicationLoggingConfigurationInput {
                sip_media_application_id: self.sip_media_application_id,
            },
        )
    }
}