// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListPhoneNumberOrders`](crate::operation::list_phone_number_orders::builders::ListPhoneNumberOrdersFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_phone_number_orders::builders::ListPhoneNumberOrdersFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`next_token(impl Into<String>)`](crate::operation::list_phone_number_orders::builders::ListPhoneNumberOrdersFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_phone_number_orders::builders::ListPhoneNumberOrdersFluentBuilder::set_next_token):<br>required: **false**<br><p>The token used to retrieve the next page of results.</p><br>
    ///   - [`max_results(i32)`](crate::operation::list_phone_number_orders::builders::ListPhoneNumberOrdersFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_phone_number_orders::builders::ListPhoneNumberOrdersFluentBuilder::set_max_results):<br>required: **false**<br><p>The maximum number of results to return in a single call.</p><br>
    /// - On success, responds with [`ListPhoneNumberOrdersOutput`](crate::operation::list_phone_number_orders::ListPhoneNumberOrdersOutput) with field(s):
    ///   - [`phone_number_orders(Option<Vec::<PhoneNumberOrder>>)`](crate::operation::list_phone_number_orders::ListPhoneNumberOrdersOutput::phone_number_orders): <p>The phone number order details.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_phone_number_orders::ListPhoneNumberOrdersOutput::next_token): <p>The token used to retrieve the next page of results.</p>
    /// - On failure, responds with [`SdkError<ListPhoneNumberOrdersError>`](crate::operation::list_phone_number_orders::ListPhoneNumberOrdersError)
    pub fn list_phone_number_orders(&self) -> crate::operation::list_phone_number_orders::builders::ListPhoneNumberOrdersFluentBuilder {
        crate::operation::list_phone_number_orders::builders::ListPhoneNumberOrdersFluentBuilder::new(self.handle.clone())
    }
}
