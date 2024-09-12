// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct RestorePhoneNumberOutput {
    /// <p>The restored phone number.</p>
    pub phone_number: ::std::option::Option<crate::types::PhoneNumber>,
    _request_id: Option<String>,
}
impl RestorePhoneNumberOutput {
    /// <p>The restored phone number.</p>
    pub fn phone_number(&self) -> ::std::option::Option<&crate::types::PhoneNumber> {
        self.phone_number.as_ref()
    }
}
impl ::aws_types::request_id::RequestId for RestorePhoneNumberOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl RestorePhoneNumberOutput {
    /// Creates a new builder-style object to manufacture [`RestorePhoneNumberOutput`](crate::operation::restore_phone_number::RestorePhoneNumberOutput).
    pub fn builder() -> crate::operation::restore_phone_number::builders::RestorePhoneNumberOutputBuilder {
        crate::operation::restore_phone_number::builders::RestorePhoneNumberOutputBuilder::default()
    }
}

/// A builder for [`RestorePhoneNumberOutput`](crate::operation::restore_phone_number::RestorePhoneNumberOutput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct RestorePhoneNumberOutputBuilder {
    pub(crate) phone_number: ::std::option::Option<crate::types::PhoneNumber>,
    _request_id: Option<String>,
}
impl RestorePhoneNumberOutputBuilder {
    /// <p>The restored phone number.</p>
    pub fn phone_number(mut self, input: crate::types::PhoneNumber) -> Self {
        self.phone_number = ::std::option::Option::Some(input);
        self
    }
    /// <p>The restored phone number.</p>
    pub fn set_phone_number(mut self, input: ::std::option::Option<crate::types::PhoneNumber>) -> Self {
        self.phone_number = input;
        self
    }
    /// <p>The restored phone number.</p>
    pub fn get_phone_number(&self) -> &::std::option::Option<crate::types::PhoneNumber> {
        &self.phone_number
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`RestorePhoneNumberOutput`](crate::operation::restore_phone_number::RestorePhoneNumberOutput).
    pub fn build(self) -> crate::operation::restore_phone_number::RestorePhoneNumberOutput {
        crate::operation::restore_phone_number::RestorePhoneNumberOutput {
            phone_number: self.phone_number,
            _request_id: self._request_id,
        }
    }
}