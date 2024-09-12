// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeleteVoiceProfileDomainInput {
    /// <p>The voice profile domain ID.</p>
    pub voice_profile_domain_id: ::std::option::Option<::std::string::String>,
}
impl DeleteVoiceProfileDomainInput {
    /// <p>The voice profile domain ID.</p>
    pub fn voice_profile_domain_id(&self) -> ::std::option::Option<&str> {
        self.voice_profile_domain_id.as_deref()
    }
}
impl DeleteVoiceProfileDomainInput {
    /// Creates a new builder-style object to manufacture [`DeleteVoiceProfileDomainInput`](crate::operation::delete_voice_profile_domain::DeleteVoiceProfileDomainInput).
    pub fn builder() -> crate::operation::delete_voice_profile_domain::builders::DeleteVoiceProfileDomainInputBuilder {
        crate::operation::delete_voice_profile_domain::builders::DeleteVoiceProfileDomainInputBuilder::default()
    }
}

/// A builder for [`DeleteVoiceProfileDomainInput`](crate::operation::delete_voice_profile_domain::DeleteVoiceProfileDomainInput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct DeleteVoiceProfileDomainInputBuilder {
    pub(crate) voice_profile_domain_id: ::std::option::Option<::std::string::String>,
}
impl DeleteVoiceProfileDomainInputBuilder {
    /// <p>The voice profile domain ID.</p>
    /// This field is required.
    pub fn voice_profile_domain_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.voice_profile_domain_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The voice profile domain ID.</p>
    pub fn set_voice_profile_domain_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.voice_profile_domain_id = input;
        self
    }
    /// <p>The voice profile domain ID.</p>
    pub fn get_voice_profile_domain_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.voice_profile_domain_id
    }
    /// Consumes the builder and constructs a [`DeleteVoiceProfileDomainInput`](crate::operation::delete_voice_profile_domain::DeleteVoiceProfileDomainInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::delete_voice_profile_domain::DeleteVoiceProfileDomainInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::delete_voice_profile_domain::DeleteVoiceProfileDomainInput {
            voice_profile_domain_id: self.voice_profile_domain_id,
        })
    }
}