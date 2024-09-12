// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct StartSpeakerSearchTaskInput {
    /// <p>The Voice Connector ID.</p>
    pub voice_connector_id: ::std::option::Option<::std::string::String>,
    /// <p>The transaction ID of the call being analyzed.</p>
    pub transaction_id: ::std::option::Option<::std::string::String>,
    /// <p>The ID of the voice profile domain that will store the voice profile.</p>
    pub voice_profile_domain_id: ::std::option::Option<::std::string::String>,
    /// <p>The unique identifier for the client request. Use a different token for different speaker search tasks.</p>
    pub client_request_token: ::std::option::Option<::std::string::String>,
    /// <p>Specifies which call leg to stream for speaker search.</p>
    pub call_leg: ::std::option::Option<crate::types::CallLegType>,
}
impl StartSpeakerSearchTaskInput {
    /// <p>The Voice Connector ID.</p>
    pub fn voice_connector_id(&self) -> ::std::option::Option<&str> {
        self.voice_connector_id.as_deref()
    }
    /// <p>The transaction ID of the call being analyzed.</p>
    pub fn transaction_id(&self) -> ::std::option::Option<&str> {
        self.transaction_id.as_deref()
    }
    /// <p>The ID of the voice profile domain that will store the voice profile.</p>
    pub fn voice_profile_domain_id(&self) -> ::std::option::Option<&str> {
        self.voice_profile_domain_id.as_deref()
    }
    /// <p>The unique identifier for the client request. Use a different token for different speaker search tasks.</p>
    pub fn client_request_token(&self) -> ::std::option::Option<&str> {
        self.client_request_token.as_deref()
    }
    /// <p>Specifies which call leg to stream for speaker search.</p>
    pub fn call_leg(&self) -> ::std::option::Option<&crate::types::CallLegType> {
        self.call_leg.as_ref()
    }
}
impl StartSpeakerSearchTaskInput {
    /// Creates a new builder-style object to manufacture [`StartSpeakerSearchTaskInput`](crate::operation::start_speaker_search_task::StartSpeakerSearchTaskInput).
    pub fn builder() -> crate::operation::start_speaker_search_task::builders::StartSpeakerSearchTaskInputBuilder {
        crate::operation::start_speaker_search_task::builders::StartSpeakerSearchTaskInputBuilder::default()
    }
}

/// A builder for [`StartSpeakerSearchTaskInput`](crate::operation::start_speaker_search_task::StartSpeakerSearchTaskInput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct StartSpeakerSearchTaskInputBuilder {
    pub(crate) voice_connector_id: ::std::option::Option<::std::string::String>,
    pub(crate) transaction_id: ::std::option::Option<::std::string::String>,
    pub(crate) voice_profile_domain_id: ::std::option::Option<::std::string::String>,
    pub(crate) client_request_token: ::std::option::Option<::std::string::String>,
    pub(crate) call_leg: ::std::option::Option<crate::types::CallLegType>,
}
impl StartSpeakerSearchTaskInputBuilder {
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
    /// <p>The transaction ID of the call being analyzed.</p>
    /// This field is required.
    pub fn transaction_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.transaction_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The transaction ID of the call being analyzed.</p>
    pub fn set_transaction_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.transaction_id = input;
        self
    }
    /// <p>The transaction ID of the call being analyzed.</p>
    pub fn get_transaction_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.transaction_id
    }
    /// <p>The ID of the voice profile domain that will store the voice profile.</p>
    /// This field is required.
    pub fn voice_profile_domain_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.voice_profile_domain_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the voice profile domain that will store the voice profile.</p>
    pub fn set_voice_profile_domain_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.voice_profile_domain_id = input;
        self
    }
    /// <p>The ID of the voice profile domain that will store the voice profile.</p>
    pub fn get_voice_profile_domain_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.voice_profile_domain_id
    }
    /// <p>The unique identifier for the client request. Use a different token for different speaker search tasks.</p>
    pub fn client_request_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.client_request_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The unique identifier for the client request. Use a different token for different speaker search tasks.</p>
    pub fn set_client_request_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.client_request_token = input;
        self
    }
    /// <p>The unique identifier for the client request. Use a different token for different speaker search tasks.</p>
    pub fn get_client_request_token(&self) -> &::std::option::Option<::std::string::String> {
        &self.client_request_token
    }
    /// <p>Specifies which call leg to stream for speaker search.</p>
    pub fn call_leg(mut self, input: crate::types::CallLegType) -> Self {
        self.call_leg = ::std::option::Option::Some(input);
        self
    }
    /// <p>Specifies which call leg to stream for speaker search.</p>
    pub fn set_call_leg(mut self, input: ::std::option::Option<crate::types::CallLegType>) -> Self {
        self.call_leg = input;
        self
    }
    /// <p>Specifies which call leg to stream for speaker search.</p>
    pub fn get_call_leg(&self) -> &::std::option::Option<crate::types::CallLegType> {
        &self.call_leg
    }
    /// Consumes the builder and constructs a [`StartSpeakerSearchTaskInput`](crate::operation::start_speaker_search_task::StartSpeakerSearchTaskInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::start_speaker_search_task::StartSpeakerSearchTaskInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::start_speaker_search_task::StartSpeakerSearchTaskInput {
            voice_connector_id: self.voice_connector_id,
            transaction_id: self.transaction_id,
            voice_profile_domain_id: self.voice_profile_domain_id,
            client_request_token: self.client_request_token,
            call_leg: self.call_leg,
        })
    }
}
