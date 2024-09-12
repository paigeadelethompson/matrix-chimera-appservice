// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListPhoneNumberOrdersInput {
    /// <p>The token used to retrieve the next page of results.</p>
    pub next_token: ::std::option::Option<::std::string::String>,
    /// <p>The maximum number of results to return in a single call.</p>
    pub max_results: ::std::option::Option<i32>,
}
impl ListPhoneNumberOrdersInput {
    /// <p>The token used to retrieve the next page of results.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
    /// <p>The maximum number of results to return in a single call.</p>
    pub fn max_results(&self) -> ::std::option::Option<i32> {
        self.max_results
    }
}
impl ListPhoneNumberOrdersInput {
    /// Creates a new builder-style object to manufacture [`ListPhoneNumberOrdersInput`](crate::operation::list_phone_number_orders::ListPhoneNumberOrdersInput).
    pub fn builder() -> crate::operation::list_phone_number_orders::builders::ListPhoneNumberOrdersInputBuilder {
        crate::operation::list_phone_number_orders::builders::ListPhoneNumberOrdersInputBuilder::default()
    }
}

/// A builder for [`ListPhoneNumberOrdersInput`](crate::operation::list_phone_number_orders::ListPhoneNumberOrdersInput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct ListPhoneNumberOrdersInputBuilder {
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    pub(crate) max_results: ::std::option::Option<i32>,
}
impl ListPhoneNumberOrdersInputBuilder {
    /// <p>The token used to retrieve the next page of results.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The token used to retrieve the next page of results.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// <p>The token used to retrieve the next page of results.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        &self.next_token
    }
    /// <p>The maximum number of results to return in a single call.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.max_results = ::std::option::Option::Some(input);
        self
    }
    /// <p>The maximum number of results to return in a single call.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.max_results = input;
        self
    }
    /// <p>The maximum number of results to return in a single call.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        &self.max_results
    }
    /// Consumes the builder and constructs a [`ListPhoneNumberOrdersInput`](crate::operation::list_phone_number_orders::ListPhoneNumberOrdersInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::operation::list_phone_number_orders::ListPhoneNumberOrdersInput, ::aws_smithy_types::error::operation::BuildError>
    {
        ::std::result::Result::Ok(crate::operation::list_phone_number_orders::ListPhoneNumberOrdersInput {
            next_token: self.next_token,
            max_results: self.max_results,
        })
    }
}
