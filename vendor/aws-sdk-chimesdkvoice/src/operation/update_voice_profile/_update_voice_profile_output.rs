// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct UpdateVoiceProfileOutput {
    /// <p>The updated voice profile settings.</p>
    pub voice_profile: ::std::option::Option<crate::types::VoiceProfile>,
    _request_id: Option<String>,
}
impl UpdateVoiceProfileOutput {
    /// <p>The updated voice profile settings.</p>
    pub fn voice_profile(&self) -> ::std::option::Option<&crate::types::VoiceProfile> {
        self.voice_profile.as_ref()
    }
}
impl ::aws_types::request_id::RequestId for UpdateVoiceProfileOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl UpdateVoiceProfileOutput {
    /// Creates a new builder-style object to manufacture [`UpdateVoiceProfileOutput`](crate::operation::update_voice_profile::UpdateVoiceProfileOutput).
    pub fn builder() -> crate::operation::update_voice_profile::builders::UpdateVoiceProfileOutputBuilder {
        crate::operation::update_voice_profile::builders::UpdateVoiceProfileOutputBuilder::default()
    }
}

/// A builder for [`UpdateVoiceProfileOutput`](crate::operation::update_voice_profile::UpdateVoiceProfileOutput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct UpdateVoiceProfileOutputBuilder {
    pub(crate) voice_profile: ::std::option::Option<crate::types::VoiceProfile>,
    _request_id: Option<String>,
}
impl UpdateVoiceProfileOutputBuilder {
    /// <p>The updated voice profile settings.</p>
    pub fn voice_profile(mut self, input: crate::types::VoiceProfile) -> Self {
        self.voice_profile = ::std::option::Option::Some(input);
        self
    }
    /// <p>The updated voice profile settings.</p>
    pub fn set_voice_profile(mut self, input: ::std::option::Option<crate::types::VoiceProfile>) -> Self {
        self.voice_profile = input;
        self
    }
    /// <p>The updated voice profile settings.</p>
    pub fn get_voice_profile(&self) -> &::std::option::Option<crate::types::VoiceProfile> {
        &self.voice_profile
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`UpdateVoiceProfileOutput`](crate::operation::update_voice_profile::UpdateVoiceProfileOutput).
    pub fn build(self) -> crate::operation::update_voice_profile::UpdateVoiceProfileOutput {
        crate::operation::update_voice_profile::UpdateVoiceProfileOutput {
            voice_profile: self.voice_profile,
            _request_id: self._request_id,
        }
    }
}