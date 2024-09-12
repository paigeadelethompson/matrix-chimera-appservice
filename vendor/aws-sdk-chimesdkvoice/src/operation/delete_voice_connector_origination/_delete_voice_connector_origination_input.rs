// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeleteVoiceConnectorOriginationInput {
    /// <p>The Voice Connector ID.</p>
    pub voice_connector_id: ::std::option::Option<::std::string::String>,
}
impl DeleteVoiceConnectorOriginationInput {
    /// <p>The Voice Connector ID.</p>
    pub fn voice_connector_id(&self) -> ::std::option::Option<&str> {
        self.voice_connector_id.as_deref()
    }
}
impl DeleteVoiceConnectorOriginationInput {
    /// Creates a new builder-style object to manufacture [`DeleteVoiceConnectorOriginationInput`](crate::operation::delete_voice_connector_origination::DeleteVoiceConnectorOriginationInput).
    pub fn builder() -> crate::operation::delete_voice_connector_origination::builders::DeleteVoiceConnectorOriginationInputBuilder {
        crate::operation::delete_voice_connector_origination::builders::DeleteVoiceConnectorOriginationInputBuilder::default()
    }
}

/// A builder for [`DeleteVoiceConnectorOriginationInput`](crate::operation::delete_voice_connector_origination::DeleteVoiceConnectorOriginationInput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct DeleteVoiceConnectorOriginationInputBuilder {
    pub(crate) voice_connector_id: ::std::option::Option<::std::string::String>,
}
impl DeleteVoiceConnectorOriginationInputBuilder {
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
    /// Consumes the builder and constructs a [`DeleteVoiceConnectorOriginationInput`](crate::operation::delete_voice_connector_origination::DeleteVoiceConnectorOriginationInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::delete_voice_connector_origination::DeleteVoiceConnectorOriginationInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::delete_voice_connector_origination::DeleteVoiceConnectorOriginationInput {
                voice_connector_id: self.voice_connector_id,
            },
        )
    }
}