// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetSipRuleInput {
    /// <p>The SIP rule ID.</p>
    pub sip_rule_id: ::std::option::Option<::std::string::String>,
}
impl GetSipRuleInput {
    /// <p>The SIP rule ID.</p>
    pub fn sip_rule_id(&self) -> ::std::option::Option<&str> {
        self.sip_rule_id.as_deref()
    }
}
impl GetSipRuleInput {
    /// Creates a new builder-style object to manufacture [`GetSipRuleInput`](crate::operation::get_sip_rule::GetSipRuleInput).
    pub fn builder() -> crate::operation::get_sip_rule::builders::GetSipRuleInputBuilder {
        crate::operation::get_sip_rule::builders::GetSipRuleInputBuilder::default()
    }
}

/// A builder for [`GetSipRuleInput`](crate::operation::get_sip_rule::GetSipRuleInput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct GetSipRuleInputBuilder {
    pub(crate) sip_rule_id: ::std::option::Option<::std::string::String>,
}
impl GetSipRuleInputBuilder {
    /// <p>The SIP rule ID.</p>
    /// This field is required.
    pub fn sip_rule_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.sip_rule_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The SIP rule ID.</p>
    pub fn set_sip_rule_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.sip_rule_id = input;
        self
    }
    /// <p>The SIP rule ID.</p>
    pub fn get_sip_rule_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.sip_rule_id
    }
    /// Consumes the builder and constructs a [`GetSipRuleInput`](crate::operation::get_sip_rule::GetSipRuleInput).
    pub fn build(self) -> ::std::result::Result<crate::operation::get_sip_rule::GetSipRuleInput, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::operation::get_sip_rule::GetSipRuleInput {
            sip_rule_id: self.sip_rule_id,
        })
    }
}
