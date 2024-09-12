// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Provides information about the role that is assigned to the user.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct RoleInfo {
    /// <p>The friendly name of the role that is assigned to the user.</p>
    pub role_name: ::std::option::Option<::std::string::String>,
    /// <p>The identifier of the AWS account assigned to the user.</p>
    pub account_id: ::std::option::Option<::std::string::String>,
}
impl RoleInfo {
    /// <p>The friendly name of the role that is assigned to the user.</p>
    pub fn role_name(&self) -> ::std::option::Option<&str> {
        self.role_name.as_deref()
    }
    /// <p>The identifier of the AWS account assigned to the user.</p>
    pub fn account_id(&self) -> ::std::option::Option<&str> {
        self.account_id.as_deref()
    }
}
impl RoleInfo {
    /// Creates a new builder-style object to manufacture [`RoleInfo`](crate::types::RoleInfo).
    pub fn builder() -> crate::types::builders::RoleInfoBuilder {
        crate::types::builders::RoleInfoBuilder::default()
    }
}

/// A builder for [`RoleInfo`](crate::types::RoleInfo).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct RoleInfoBuilder {
    pub(crate) role_name: ::std::option::Option<::std::string::String>,
    pub(crate) account_id: ::std::option::Option<::std::string::String>,
}
impl RoleInfoBuilder {
    /// <p>The friendly name of the role that is assigned to the user.</p>
    pub fn role_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.role_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The friendly name of the role that is assigned to the user.</p>
    pub fn set_role_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.role_name = input;
        self
    }
    /// <p>The friendly name of the role that is assigned to the user.</p>
    pub fn get_role_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.role_name
    }
    /// <p>The identifier of the AWS account assigned to the user.</p>
    pub fn account_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.account_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The identifier of the AWS account assigned to the user.</p>
    pub fn set_account_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.account_id = input;
        self
    }
    /// <p>The identifier of the AWS account assigned to the user.</p>
    pub fn get_account_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.account_id
    }
    /// Consumes the builder and constructs a [`RoleInfo`](crate::types::RoleInfo).
    pub fn build(self) -> crate::types::RoleInfo {
        crate::types::RoleInfo {
            role_name: self.role_name,
            account_id: self.account_id,
        }
    }
}