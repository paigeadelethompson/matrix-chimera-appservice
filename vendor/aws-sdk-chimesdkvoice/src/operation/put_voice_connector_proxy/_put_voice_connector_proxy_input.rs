// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
pub struct PutVoiceConnectorProxyInput {
    /// <p>The Voice Connector ID.</p>
    pub voice_connector_id: ::std::option::Option<::std::string::String>,
    /// <p>The default number of minutes allowed for proxy session.</p>
    pub default_session_expiry_minutes: ::std::option::Option<i32>,
    /// <p>The countries for proxy phone numbers to be selected from.</p>
    pub phone_number_pool_countries: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>The phone number to route calls to after a proxy session expires.</p>
    pub fall_back_phone_number: ::std::option::Option<::std::string::String>,
    /// <p>When true, stops proxy sessions from being created on the specified Amazon Chime SDK Voice Connector.</p>
    pub disabled: ::std::option::Option<bool>,
}
impl PutVoiceConnectorProxyInput {
    /// <p>The Voice Connector ID.</p>
    pub fn voice_connector_id(&self) -> ::std::option::Option<&str> {
        self.voice_connector_id.as_deref()
    }
    /// <p>The default number of minutes allowed for proxy session.</p>
    pub fn default_session_expiry_minutes(&self) -> ::std::option::Option<i32> {
        self.default_session_expiry_minutes
    }
    /// <p>The countries for proxy phone numbers to be selected from.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.phone_number_pool_countries.is_none()`.
    pub fn phone_number_pool_countries(&self) -> &[::std::string::String] {
        self.phone_number_pool_countries.as_deref().unwrap_or_default()
    }
    /// <p>The phone number to route calls to after a proxy session expires.</p>
    pub fn fall_back_phone_number(&self) -> ::std::option::Option<&str> {
        self.fall_back_phone_number.as_deref()
    }
    /// <p>When true, stops proxy sessions from being created on the specified Amazon Chime SDK Voice Connector.</p>
    pub fn disabled(&self) -> ::std::option::Option<bool> {
        self.disabled
    }
}
impl ::std::fmt::Debug for PutVoiceConnectorProxyInput {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("PutVoiceConnectorProxyInput");
        formatter.field("voice_connector_id", &self.voice_connector_id);
        formatter.field("default_session_expiry_minutes", &self.default_session_expiry_minutes);
        formatter.field("phone_number_pool_countries", &self.phone_number_pool_countries);
        formatter.field("fall_back_phone_number", &"*** Sensitive Data Redacted ***");
        formatter.field("disabled", &self.disabled);
        formatter.finish()
    }
}
impl PutVoiceConnectorProxyInput {
    /// Creates a new builder-style object to manufacture [`PutVoiceConnectorProxyInput`](crate::operation::put_voice_connector_proxy::PutVoiceConnectorProxyInput).
    pub fn builder() -> crate::operation::put_voice_connector_proxy::builders::PutVoiceConnectorProxyInputBuilder {
        crate::operation::put_voice_connector_proxy::builders::PutVoiceConnectorProxyInputBuilder::default()
    }
}

/// A builder for [`PutVoiceConnectorProxyInput`](crate::operation::put_voice_connector_proxy::PutVoiceConnectorProxyInput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
#[non_exhaustive]
pub struct PutVoiceConnectorProxyInputBuilder {
    pub(crate) voice_connector_id: ::std::option::Option<::std::string::String>,
    pub(crate) default_session_expiry_minutes: ::std::option::Option<i32>,
    pub(crate) phone_number_pool_countries: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) fall_back_phone_number: ::std::option::Option<::std::string::String>,
    pub(crate) disabled: ::std::option::Option<bool>,
}
impl PutVoiceConnectorProxyInputBuilder {
    /// <p>The Voice Connector ID.</p>
    /// This field is required.
    pub fn voice_connector_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.voice_connector_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Voice Connector ID.</p>
    pub fn set_voice_connector_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.voice_connector_id = input;
        self
    }
    /// <p>The Voice Connector ID.</p>
    pub fn get_voice_connector_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.voice_connector_id
    }
    /// <p>The default number of minutes allowed for proxy session.</p>
    /// This field is required.
    pub fn default_session_expiry_minutes(mut self, input: i32) -> Self {
        self.default_session_expiry_minutes = ::std::option::Option::Some(input);
        self
    }
    /// <p>The default number of minutes allowed for proxy session.</p>
    pub fn set_default_session_expiry_minutes(mut self, input: ::std::option::Option<i32>) -> Self {
        self.default_session_expiry_minutes = input;
        self
    }
    /// <p>The default number of minutes allowed for proxy session.</p>
    pub fn get_default_session_expiry_minutes(&self) -> &::std::option::Option<i32> {
        &self.default_session_expiry_minutes
    }
    /// Appends an item to `phone_number_pool_countries`.
    ///
    /// To override the contents of this collection use [`set_phone_number_pool_countries`](Self::set_phone_number_pool_countries).
    ///
    /// <p>The countries for proxy phone numbers to be selected from.</p>
    pub fn phone_number_pool_countries(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.phone_number_pool_countries.unwrap_or_default();
        v.push(input.into());
        self.phone_number_pool_countries = ::std::option::Option::Some(v);
        self
    }
    /// <p>The countries for proxy phone numbers to be selected from.</p>
    pub fn set_phone_number_pool_countries(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.phone_number_pool_countries = input;
        self
    }
    /// <p>The countries for proxy phone numbers to be selected from.</p>
    pub fn get_phone_number_pool_countries(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        &self.phone_number_pool_countries
    }
    /// <p>The phone number to route calls to after a proxy session expires.</p>
    pub fn fall_back_phone_number(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.fall_back_phone_number = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The phone number to route calls to after a proxy session expires.</p>
    pub fn set_fall_back_phone_number(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.fall_back_phone_number = input;
        self
    }
    /// <p>The phone number to route calls to after a proxy session expires.</p>
    pub fn get_fall_back_phone_number(&self) -> &::std::option::Option<::std::string::String> {
        &self.fall_back_phone_number
    }
    /// <p>When true, stops proxy sessions from being created on the specified Amazon Chime SDK Voice Connector.</p>
    pub fn disabled(mut self, input: bool) -> Self {
        self.disabled = ::std::option::Option::Some(input);
        self
    }
    /// <p>When true, stops proxy sessions from being created on the specified Amazon Chime SDK Voice Connector.</p>
    pub fn set_disabled(mut self, input: ::std::option::Option<bool>) -> Self {
        self.disabled = input;
        self
    }
    /// <p>When true, stops proxy sessions from being created on the specified Amazon Chime SDK Voice Connector.</p>
    pub fn get_disabled(&self) -> &::std::option::Option<bool> {
        &self.disabled
    }
    /// Consumes the builder and constructs a [`PutVoiceConnectorProxyInput`](crate::operation::put_voice_connector_proxy::PutVoiceConnectorProxyInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::put_voice_connector_proxy::PutVoiceConnectorProxyInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::put_voice_connector_proxy::PutVoiceConnectorProxyInput {
            voice_connector_id: self.voice_connector_id,
            default_session_expiry_minutes: self.default_session_expiry_minutes,
            phone_number_pool_countries: self.phone_number_pool_countries,
            fall_back_phone_number: self.fall_back_phone_number,
            disabled: self.disabled,
        })
    }
}
impl ::std::fmt::Debug for PutVoiceConnectorProxyInputBuilder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("PutVoiceConnectorProxyInputBuilder");
        formatter.field("voice_connector_id", &self.voice_connector_id);
        formatter.field("default_session_expiry_minutes", &self.default_session_expiry_minutes);
        formatter.field("phone_number_pool_countries", &self.phone_number_pool_countries);
        formatter.field("fall_back_phone_number", &"*** Sensitive Data Redacted ***");
        formatter.field("disabled", &self.disabled);
        formatter.finish()
    }
}