// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetVoiceConnectorStreamingConfigurationInput {
    /// <p>The Voice Connector ID.</p>
    pub voice_connector_id: ::std::option::Option<::std::string::String>,
}
impl GetVoiceConnectorStreamingConfigurationInput {
    /// <p>The Voice Connector ID.</p>
    pub fn voice_connector_id(&self) -> ::std::option::Option<&str> {
        self.voice_connector_id.as_deref()
    }
}
impl GetVoiceConnectorStreamingConfigurationInput {
    /// Creates a new builder-style object to manufacture [`GetVoiceConnectorStreamingConfigurationInput`](crate::operation::get_voice_connector_streaming_configuration::GetVoiceConnectorStreamingConfigurationInput).
    pub fn builder() -> crate::operation::get_voice_connector_streaming_configuration::builders::GetVoiceConnectorStreamingConfigurationInputBuilder {
        crate::operation::get_voice_connector_streaming_configuration::builders::GetVoiceConnectorStreamingConfigurationInputBuilder::default()
    }
}

/// A builder for [`GetVoiceConnectorStreamingConfigurationInput`](crate::operation::get_voice_connector_streaming_configuration::GetVoiceConnectorStreamingConfigurationInput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct GetVoiceConnectorStreamingConfigurationInputBuilder {
    pub(crate) voice_connector_id: ::std::option::Option<::std::string::String>,
}
impl GetVoiceConnectorStreamingConfigurationInputBuilder {
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
    /// Consumes the builder and constructs a [`GetVoiceConnectorStreamingConfigurationInput`](crate::operation::get_voice_connector_streaming_configuration::GetVoiceConnectorStreamingConfigurationInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::get_voice_connector_streaming_configuration::GetVoiceConnectorStreamingConfigurationInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::get_voice_connector_streaming_configuration::GetVoiceConnectorStreamingConfigurationInput {
                voice_connector_id: self.voice_connector_id,
            },
        )
    }
}
