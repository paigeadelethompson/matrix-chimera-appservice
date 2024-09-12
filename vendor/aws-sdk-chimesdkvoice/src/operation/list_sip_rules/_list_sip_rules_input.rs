// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListSipRulesInput {
    /// <p>The SIP media application ID.</p>
    pub sip_media_application_id: ::std::option::Option<::std::string::String>,
    /// <p>The maximum number of results to return in a single call. Defaults to 100.</p>
    pub max_results: ::std::option::Option<i32>,
    /// <p>The token used to return the next page of results.</p>
    pub next_token: ::std::option::Option<::std::string::String>,
}
impl ListSipRulesInput {
    /// <p>The SIP media application ID.</p>
    pub fn sip_media_application_id(&self) -> ::std::option::Option<&str> {
        self.sip_media_application_id.as_deref()
    }
    /// <p>The maximum number of results to return in a single call. Defaults to 100.</p>
    pub fn max_results(&self) -> ::std::option::Option<i32> {
        self.max_results
    }
    /// <p>The token used to return the next page of results.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl ListSipRulesInput {
    /// Creates a new builder-style object to manufacture [`ListSipRulesInput`](crate::operation::list_sip_rules::ListSipRulesInput).
    pub fn builder() -> crate::operation::list_sip_rules::builders::ListSipRulesInputBuilder {
        crate::operation::list_sip_rules::builders::ListSipRulesInputBuilder::default()
    }
}

/// A builder for [`ListSipRulesInput`](crate::operation::list_sip_rules::ListSipRulesInput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct ListSipRulesInputBuilder {
    pub(crate) sip_media_application_id: ::std::option::Option<::std::string::String>,
    pub(crate) max_results: ::std::option::Option<i32>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
}
impl ListSipRulesInputBuilder {
    /// <p>The SIP media application ID.</p>
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
    /// <p>The maximum number of results to return in a single call. Defaults to 100.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.max_results = ::std::option::Option::Some(input);
        self
    }
    /// <p>The maximum number of results to return in a single call. Defaults to 100.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.max_results = input;
        self
    }
    /// <p>The maximum number of results to return in a single call. Defaults to 100.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        &self.max_results
    }
    /// <p>The token used to return the next page of results.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The token used to return the next page of results.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// <p>The token used to return the next page of results.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        &self.next_token
    }
    /// Consumes the builder and constructs a [`ListSipRulesInput`](crate::operation::list_sip_rules::ListSipRulesInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::operation::list_sip_rules::ListSipRulesInput, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::operation::list_sip_rules::ListSipRulesInput {
            sip_media_application_id: self.sip_media_application_id,
            max_results: self.max_results,
            next_token: self.next_token,
        })
    }
}
