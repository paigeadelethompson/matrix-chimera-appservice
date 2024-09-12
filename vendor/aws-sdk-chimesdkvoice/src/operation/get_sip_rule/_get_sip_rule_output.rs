// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetSipRuleOutput {
    /// <p>The SIP rule details.</p>
    pub sip_rule: ::std::option::Option<crate::types::SipRule>,
    _request_id: Option<String>,
}
impl GetSipRuleOutput {
    /// <p>The SIP rule details.</p>
    pub fn sip_rule(&self) -> ::std::option::Option<&crate::types::SipRule> {
        self.sip_rule.as_ref()
    }
}
impl ::aws_types::request_id::RequestId for GetSipRuleOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl GetSipRuleOutput {
    /// Creates a new builder-style object to manufacture [`GetSipRuleOutput`](crate::operation::get_sip_rule::GetSipRuleOutput).
    pub fn builder() -> crate::operation::get_sip_rule::builders::GetSipRuleOutputBuilder {
        crate::operation::get_sip_rule::builders::GetSipRuleOutputBuilder::default()
    }
}

/// A builder for [`GetSipRuleOutput`](crate::operation::get_sip_rule::GetSipRuleOutput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct GetSipRuleOutputBuilder {
    pub(crate) sip_rule: ::std::option::Option<crate::types::SipRule>,
    _request_id: Option<String>,
}
impl GetSipRuleOutputBuilder {
    /// <p>The SIP rule details.</p>
    pub fn sip_rule(mut self, input: crate::types::SipRule) -> Self {
        self.sip_rule = ::std::option::Option::Some(input);
        self
    }
    /// <p>The SIP rule details.</p>
    pub fn set_sip_rule(mut self, input: ::std::option::Option<crate::types::SipRule>) -> Self {
        self.sip_rule = input;
        self
    }
    /// <p>The SIP rule details.</p>
    pub fn get_sip_rule(&self) -> &::std::option::Option<crate::types::SipRule> {
        &self.sip_rule
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`GetSipRuleOutput`](crate::operation::get_sip_rule::GetSipRuleOutput).
    pub fn build(self) -> crate::operation::get_sip_rule::GetSipRuleOutput {
        crate::operation::get_sip_rule::GetSipRuleOutput {
            sip_rule: self.sip_rule,
            _request_id: self._request_id,
        }
    }
}
