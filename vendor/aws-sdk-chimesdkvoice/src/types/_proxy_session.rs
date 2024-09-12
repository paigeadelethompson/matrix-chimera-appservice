// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The proxy session for an Amazon Chime SDK Voice Connector.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ProxySession {
    /// <p>The Voice Connector ID.</p>
    pub voice_connector_id: ::std::option::Option<::std::string::String>,
    /// <p>The proxy session ID.</p>
    pub proxy_session_id: ::std::option::Option<::std::string::String>,
    /// <p>The proxy session name.</p>
    pub name: ::std::option::Option<::std::string::String>,
    /// <p>The proxy session status.</p>
    pub status: ::std::option::Option<crate::types::ProxySessionStatus>,
    /// <p>The number of minutes allowed for the proxy session.</p>
    pub expiry_minutes: ::std::option::Option<i32>,
    /// <p>The proxy session capabilities.</p>
    pub capabilities: ::std::option::Option<::std::vec::Vec<crate::types::Capability>>,
    /// <p>The created time stamp, in ISO 8601 format.</p>
    pub created_timestamp: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>The updated time stamp, in ISO 8601 format.</p>
    pub updated_timestamp: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>The ended time stamp, in ISO 8601 format.</p>
    pub ended_timestamp: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>The proxy session participants.</p>
    pub participants: ::std::option::Option<::std::vec::Vec<crate::types::Participant>>,
    /// <p>The preference for proxy phone number reuse, or stickiness, between the same participants across sessions.</p>
    pub number_selection_behavior: ::std::option::Option<crate::types::NumberSelectionBehavior>,
    /// <p>The preference for matching the country or area code of the proxy phone number with that of the first participant.</p>
    pub geo_match_level: ::std::option::Option<crate::types::GeoMatchLevel>,
    /// <p>The country and area code for the proxy phone number.</p>
    pub geo_match_params: ::std::option::Option<crate::types::GeoMatchParams>,
}
impl ProxySession {
    /// <p>The Voice Connector ID.</p>
    pub fn voice_connector_id(&self) -> ::std::option::Option<&str> {
        self.voice_connector_id.as_deref()
    }
    /// <p>The proxy session ID.</p>
    pub fn proxy_session_id(&self) -> ::std::option::Option<&str> {
        self.proxy_session_id.as_deref()
    }
    /// <p>The proxy session name.</p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p>The proxy session status.</p>
    pub fn status(&self) -> ::std::option::Option<&crate::types::ProxySessionStatus> {
        self.status.as_ref()
    }
    /// <p>The number of minutes allowed for the proxy session.</p>
    pub fn expiry_minutes(&self) -> ::std::option::Option<i32> {
        self.expiry_minutes
    }
    /// <p>The proxy session capabilities.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.capabilities.is_none()`.
    pub fn capabilities(&self) -> &[crate::types::Capability] {
        self.capabilities.as_deref().unwrap_or_default()
    }
    /// <p>The created time stamp, in ISO 8601 format.</p>
    pub fn created_timestamp(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.created_timestamp.as_ref()
    }
    /// <p>The updated time stamp, in ISO 8601 format.</p>
    pub fn updated_timestamp(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.updated_timestamp.as_ref()
    }
    /// <p>The ended time stamp, in ISO 8601 format.</p>
    pub fn ended_timestamp(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.ended_timestamp.as_ref()
    }
    /// <p>The proxy session participants.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.participants.is_none()`.
    pub fn participants(&self) -> &[crate::types::Participant] {
        self.participants.as_deref().unwrap_or_default()
    }
    /// <p>The preference for proxy phone number reuse, or stickiness, between the same participants across sessions.</p>
    pub fn number_selection_behavior(&self) -> ::std::option::Option<&crate::types::NumberSelectionBehavior> {
        self.number_selection_behavior.as_ref()
    }
    /// <p>The preference for matching the country or area code of the proxy phone number with that of the first participant.</p>
    pub fn geo_match_level(&self) -> ::std::option::Option<&crate::types::GeoMatchLevel> {
        self.geo_match_level.as_ref()
    }
    /// <p>The country and area code for the proxy phone number.</p>
    pub fn geo_match_params(&self) -> ::std::option::Option<&crate::types::GeoMatchParams> {
        self.geo_match_params.as_ref()
    }
}
impl ProxySession {
    /// Creates a new builder-style object to manufacture [`ProxySession`](crate::types::ProxySession).
    pub fn builder() -> crate::types::builders::ProxySessionBuilder {
        crate::types::builders::ProxySessionBuilder::default()
    }
}

/// A builder for [`ProxySession`](crate::types::ProxySession).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct ProxySessionBuilder {
    pub(crate) voice_connector_id: ::std::option::Option<::std::string::String>,
    pub(crate) proxy_session_id: ::std::option::Option<::std::string::String>,
    pub(crate) name: ::std::option::Option<::std::string::String>,
    pub(crate) status: ::std::option::Option<crate::types::ProxySessionStatus>,
    pub(crate) expiry_minutes: ::std::option::Option<i32>,
    pub(crate) capabilities: ::std::option::Option<::std::vec::Vec<crate::types::Capability>>,
    pub(crate) created_timestamp: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) updated_timestamp: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) ended_timestamp: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) participants: ::std::option::Option<::std::vec::Vec<crate::types::Participant>>,
    pub(crate) number_selection_behavior: ::std::option::Option<crate::types::NumberSelectionBehavior>,
    pub(crate) geo_match_level: ::std::option::Option<crate::types::GeoMatchLevel>,
    pub(crate) geo_match_params: ::std::option::Option<crate::types::GeoMatchParams>,
}
impl ProxySessionBuilder {
    /// <p>The Voice Connector ID.</p>
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
    /// <p>The proxy session ID.</p>
    pub fn proxy_session_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.proxy_session_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The proxy session ID.</p>
    pub fn set_proxy_session_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.proxy_session_id = input;
        self
    }
    /// <p>The proxy session ID.</p>
    pub fn get_proxy_session_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.proxy_session_id
    }
    /// <p>The proxy session name.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The proxy session name.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// <p>The proxy session name.</p>
    pub fn get_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.name
    }
    /// <p>The proxy session status.</p>
    pub fn status(mut self, input: crate::types::ProxySessionStatus) -> Self {
        self.status = ::std::option::Option::Some(input);
        self
    }
    /// <p>The proxy session status.</p>
    pub fn set_status(mut self, input: ::std::option::Option<crate::types::ProxySessionStatus>) -> Self {
        self.status = input;
        self
    }
    /// <p>The proxy session status.</p>
    pub fn get_status(&self) -> &::std::option::Option<crate::types::ProxySessionStatus> {
        &self.status
    }
    /// <p>The number of minutes allowed for the proxy session.</p>
    pub fn expiry_minutes(mut self, input: i32) -> Self {
        self.expiry_minutes = ::std::option::Option::Some(input);
        self
    }
    /// <p>The number of minutes allowed for the proxy session.</p>
    pub fn set_expiry_minutes(mut self, input: ::std::option::Option<i32>) -> Self {
        self.expiry_minutes = input;
        self
    }
    /// <p>The number of minutes allowed for the proxy session.</p>
    pub fn get_expiry_minutes(&self) -> &::std::option::Option<i32> {
        &self.expiry_minutes
    }
    /// Appends an item to `capabilities`.
    ///
    /// To override the contents of this collection use [`set_capabilities`](Self::set_capabilities).
    ///
    /// <p>The proxy session capabilities.</p>
    pub fn capabilities(mut self, input: crate::types::Capability) -> Self {
        let mut v = self.capabilities.unwrap_or_default();
        v.push(input);
        self.capabilities = ::std::option::Option::Some(v);
        self
    }
    /// <p>The proxy session capabilities.</p>
    pub fn set_capabilities(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Capability>>) -> Self {
        self.capabilities = input;
        self
    }
    /// <p>The proxy session capabilities.</p>
    pub fn get_capabilities(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Capability>> {
        &self.capabilities
    }
    /// <p>The created time stamp, in ISO 8601 format.</p>
    pub fn created_timestamp(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.created_timestamp = ::std::option::Option::Some(input);
        self
    }
    /// <p>The created time stamp, in ISO 8601 format.</p>
    pub fn set_created_timestamp(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.created_timestamp = input;
        self
    }
    /// <p>The created time stamp, in ISO 8601 format.</p>
    pub fn get_created_timestamp(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.created_timestamp
    }
    /// <p>The updated time stamp, in ISO 8601 format.</p>
    pub fn updated_timestamp(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.updated_timestamp = ::std::option::Option::Some(input);
        self
    }
    /// <p>The updated time stamp, in ISO 8601 format.</p>
    pub fn set_updated_timestamp(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.updated_timestamp = input;
        self
    }
    /// <p>The updated time stamp, in ISO 8601 format.</p>
    pub fn get_updated_timestamp(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.updated_timestamp
    }
    /// <p>The ended time stamp, in ISO 8601 format.</p>
    pub fn ended_timestamp(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.ended_timestamp = ::std::option::Option::Some(input);
        self
    }
    /// <p>The ended time stamp, in ISO 8601 format.</p>
    pub fn set_ended_timestamp(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.ended_timestamp = input;
        self
    }
    /// <p>The ended time stamp, in ISO 8601 format.</p>
    pub fn get_ended_timestamp(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.ended_timestamp
    }
    /// Appends an item to `participants`.
    ///
    /// To override the contents of this collection use [`set_participants`](Self::set_participants).
    ///
    /// <p>The proxy session participants.</p>
    pub fn participants(mut self, input: crate::types::Participant) -> Self {
        let mut v = self.participants.unwrap_or_default();
        v.push(input);
        self.participants = ::std::option::Option::Some(v);
        self
    }
    /// <p>The proxy session participants.</p>
    pub fn set_participants(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Participant>>) -> Self {
        self.participants = input;
        self
    }
    /// <p>The proxy session participants.</p>
    pub fn get_participants(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Participant>> {
        &self.participants
    }
    /// <p>The preference for proxy phone number reuse, or stickiness, between the same participants across sessions.</p>
    pub fn number_selection_behavior(mut self, input: crate::types::NumberSelectionBehavior) -> Self {
        self.number_selection_behavior = ::std::option::Option::Some(input);
        self
    }
    /// <p>The preference for proxy phone number reuse, or stickiness, between the same participants across sessions.</p>
    pub fn set_number_selection_behavior(mut self, input: ::std::option::Option<crate::types::NumberSelectionBehavior>) -> Self {
        self.number_selection_behavior = input;
        self
    }
    /// <p>The preference for proxy phone number reuse, or stickiness, between the same participants across sessions.</p>
    pub fn get_number_selection_behavior(&self) -> &::std::option::Option<crate::types::NumberSelectionBehavior> {
        &self.number_selection_behavior
    }
    /// <p>The preference for matching the country or area code of the proxy phone number with that of the first participant.</p>
    pub fn geo_match_level(mut self, input: crate::types::GeoMatchLevel) -> Self {
        self.geo_match_level = ::std::option::Option::Some(input);
        self
    }
    /// <p>The preference for matching the country or area code of the proxy phone number with that of the first participant.</p>
    pub fn set_geo_match_level(mut self, input: ::std::option::Option<crate::types::GeoMatchLevel>) -> Self {
        self.geo_match_level = input;
        self
    }
    /// <p>The preference for matching the country or area code of the proxy phone number with that of the first participant.</p>
    pub fn get_geo_match_level(&self) -> &::std::option::Option<crate::types::GeoMatchLevel> {
        &self.geo_match_level
    }
    /// <p>The country and area code for the proxy phone number.</p>
    pub fn geo_match_params(mut self, input: crate::types::GeoMatchParams) -> Self {
        self.geo_match_params = ::std::option::Option::Some(input);
        self
    }
    /// <p>The country and area code for the proxy phone number.</p>
    pub fn set_geo_match_params(mut self, input: ::std::option::Option<crate::types::GeoMatchParams>) -> Self {
        self.geo_match_params = input;
        self
    }
    /// <p>The country and area code for the proxy phone number.</p>
    pub fn get_geo_match_params(&self) -> &::std::option::Option<crate::types::GeoMatchParams> {
        &self.geo_match_params
    }
    /// Consumes the builder and constructs a [`ProxySession`](crate::types::ProxySession).
    pub fn build(self) -> crate::types::ProxySession {
        crate::types::ProxySession {
            voice_connector_id: self.voice_connector_id,
            proxy_session_id: self.proxy_session_id,
            name: self.name,
            status: self.status,
            expiry_minutes: self.expiry_minutes,
            capabilities: self.capabilities,
            created_timestamp: self.created_timestamp,
            updated_timestamp: self.updated_timestamp,
            ended_timestamp: self.ended_timestamp,
            participants: self.participants,
            number_selection_behavior: self.number_selection_behavior,
            geo_match_level: self.geo_match_level,
            geo_match_params: self.geo_match_params,
        }
    }
}
