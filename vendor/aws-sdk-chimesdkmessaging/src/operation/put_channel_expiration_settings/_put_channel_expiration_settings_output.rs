// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct PutChannelExpirationSettingsOutput {
    /// <p>The channel ARN.</p>
    pub channel_arn: ::std::option::Option<::std::string::String>,
    /// <p>Settings that control the interval after which a channel is deleted.</p>
    pub expiration_settings: ::std::option::Option<crate::types::ExpirationSettings>,
    _request_id: Option<String>,
}
impl PutChannelExpirationSettingsOutput {
    /// <p>The channel ARN.</p>
    pub fn channel_arn(&self) -> ::std::option::Option<&str> {
        self.channel_arn.as_deref()
    }
    /// <p>Settings that control the interval after which a channel is deleted.</p>
    pub fn expiration_settings(&self) -> ::std::option::Option<&crate::types::ExpirationSettings> {
        self.expiration_settings.as_ref()
    }
}
impl ::aws_types::request_id::RequestId for PutChannelExpirationSettingsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl PutChannelExpirationSettingsOutput {
    /// Creates a new builder-style object to manufacture [`PutChannelExpirationSettingsOutput`](crate::operation::put_channel_expiration_settings::PutChannelExpirationSettingsOutput).
    pub fn builder() -> crate::operation::put_channel_expiration_settings::builders::PutChannelExpirationSettingsOutputBuilder {
        crate::operation::put_channel_expiration_settings::builders::PutChannelExpirationSettingsOutputBuilder::default()
    }
}

/// A builder for [`PutChannelExpirationSettingsOutput`](crate::operation::put_channel_expiration_settings::PutChannelExpirationSettingsOutput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct PutChannelExpirationSettingsOutputBuilder {
    pub(crate) channel_arn: ::std::option::Option<::std::string::String>,
    pub(crate) expiration_settings: ::std::option::Option<crate::types::ExpirationSettings>,
    _request_id: Option<String>,
}
impl PutChannelExpirationSettingsOutputBuilder {
    /// <p>The channel ARN.</p>
    pub fn channel_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.channel_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The channel ARN.</p>
    pub fn set_channel_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.channel_arn = input;
        self
    }
    /// <p>The channel ARN.</p>
    pub fn get_channel_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.channel_arn
    }
    /// <p>Settings that control the interval after which a channel is deleted.</p>
    pub fn expiration_settings(mut self, input: crate::types::ExpirationSettings) -> Self {
        self.expiration_settings = ::std::option::Option::Some(input);
        self
    }
    /// <p>Settings that control the interval after which a channel is deleted.</p>
    pub fn set_expiration_settings(mut self, input: ::std::option::Option<crate::types::ExpirationSettings>) -> Self {
        self.expiration_settings = input;
        self
    }
    /// <p>Settings that control the interval after which a channel is deleted.</p>
    pub fn get_expiration_settings(&self) -> &::std::option::Option<crate::types::ExpirationSettings> {
        &self.expiration_settings
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`PutChannelExpirationSettingsOutput`](crate::operation::put_channel_expiration_settings::PutChannelExpirationSettingsOutput).
    pub fn build(self) -> crate::operation::put_channel_expiration_settings::PutChannelExpirationSettingsOutput {
        crate::operation::put_channel_expiration_settings::PutChannelExpirationSettingsOutput {
            channel_arn: self.channel_arn,
            expiration_settings: self.expiration_settings,
            _request_id: self._request_id,
        }
    }
}
