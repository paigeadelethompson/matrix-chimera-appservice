// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListPhoneNumberOrdersOutput {
    /// <p>The phone number order details.</p>
    pub phone_number_orders: ::std::option::Option<::std::vec::Vec<crate::types::PhoneNumberOrder>>,
    /// <p>The token used to retrieve the next page of results.</p>
    pub next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl ListPhoneNumberOrdersOutput {
    /// <p>The phone number order details.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.phone_number_orders.is_none()`.
    pub fn phone_number_orders(&self) -> &[crate::types::PhoneNumberOrder] {
        self.phone_number_orders.as_deref().unwrap_or_default()
    }
    /// <p>The token used to retrieve the next page of results.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl ::aws_types::request_id::RequestId for ListPhoneNumberOrdersOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ListPhoneNumberOrdersOutput {
    /// Creates a new builder-style object to manufacture [`ListPhoneNumberOrdersOutput`](crate::operation::list_phone_number_orders::ListPhoneNumberOrdersOutput).
    pub fn builder() -> crate::operation::list_phone_number_orders::builders::ListPhoneNumberOrdersOutputBuilder {
        crate::operation::list_phone_number_orders::builders::ListPhoneNumberOrdersOutputBuilder::default()
    }
}

/// A builder for [`ListPhoneNumberOrdersOutput`](crate::operation::list_phone_number_orders::ListPhoneNumberOrdersOutput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct ListPhoneNumberOrdersOutputBuilder {
    pub(crate) phone_number_orders: ::std::option::Option<::std::vec::Vec<crate::types::PhoneNumberOrder>>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl ListPhoneNumberOrdersOutputBuilder {
    /// Appends an item to `phone_number_orders`.
    ///
    /// To override the contents of this collection use [`set_phone_number_orders`](Self::set_phone_number_orders).
    ///
    /// <p>The phone number order details.</p>
    pub fn phone_number_orders(mut self, input: crate::types::PhoneNumberOrder) -> Self {
        let mut v = self.phone_number_orders.unwrap_or_default();
        v.push(input);
        self.phone_number_orders = ::std::option::Option::Some(v);
        self
    }
    /// <p>The phone number order details.</p>
    pub fn set_phone_number_orders(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::PhoneNumberOrder>>) -> Self {
        self.phone_number_orders = input;
        self
    }
    /// <p>The phone number order details.</p>
    pub fn get_phone_number_orders(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::PhoneNumberOrder>> {
        &self.phone_number_orders
    }
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
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`ListPhoneNumberOrdersOutput`](crate::operation::list_phone_number_orders::ListPhoneNumberOrdersOutput).
    pub fn build(self) -> crate::operation::list_phone_number_orders::ListPhoneNumberOrdersOutput {
        crate::operation::list_phone_number_orders::ListPhoneNumberOrdersOutput {
            phone_number_orders: self.phone_number_orders,
            next_token: self.next_token,
            _request_id: self._request_id,
        }
    }
}
