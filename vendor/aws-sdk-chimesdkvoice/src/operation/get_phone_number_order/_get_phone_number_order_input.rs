// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetPhoneNumberOrderInput {
    /// <p>The ID of the phone number order .</p>
    pub phone_number_order_id: ::std::option::Option<::std::string::String>,
}
impl GetPhoneNumberOrderInput {
    /// <p>The ID of the phone number order .</p>
    pub fn phone_number_order_id(&self) -> ::std::option::Option<&str> {
        self.phone_number_order_id.as_deref()
    }
}
impl GetPhoneNumberOrderInput {
    /// Creates a new builder-style object to manufacture [`GetPhoneNumberOrderInput`](crate::operation::get_phone_number_order::GetPhoneNumberOrderInput).
    pub fn builder() -> crate::operation::get_phone_number_order::builders::GetPhoneNumberOrderInputBuilder {
        crate::operation::get_phone_number_order::builders::GetPhoneNumberOrderInputBuilder::default()
    }
}

/// A builder for [`GetPhoneNumberOrderInput`](crate::operation::get_phone_number_order::GetPhoneNumberOrderInput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct GetPhoneNumberOrderInputBuilder {
    pub(crate) phone_number_order_id: ::std::option::Option<::std::string::String>,
}
impl GetPhoneNumberOrderInputBuilder {
    /// <p>The ID of the phone number order .</p>
    /// This field is required.
    pub fn phone_number_order_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.phone_number_order_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the phone number order .</p>
    pub fn set_phone_number_order_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.phone_number_order_id = input;
        self
    }
    /// <p>The ID of the phone number order .</p>
    pub fn get_phone_number_order_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.phone_number_order_id
    }
    /// Consumes the builder and constructs a [`GetPhoneNumberOrderInput`](crate::operation::get_phone_number_order::GetPhoneNumberOrderInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::operation::get_phone_number_order::GetPhoneNumberOrderInput, ::aws_smithy_types::error::operation::BuildError>
    {
        ::std::result::Result::Ok(crate::operation::get_phone_number_order::GetPhoneNumberOrderInput {
            phone_number_order_id: self.phone_number_order_id,
        })
    }
}