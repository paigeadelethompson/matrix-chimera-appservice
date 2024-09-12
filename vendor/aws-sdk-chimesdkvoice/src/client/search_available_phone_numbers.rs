// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`SearchAvailablePhoneNumbers`](crate::operation::search_available_phone_numbers::builders::SearchAvailablePhoneNumbersFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::search_available_phone_numbers::builders::SearchAvailablePhoneNumbersFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`area_code(impl Into<String>)`](crate::operation::search_available_phone_numbers::builders::SearchAvailablePhoneNumbersFluentBuilder::area_code) / [`set_area_code(Option<String>)`](crate::operation::search_available_phone_numbers::builders::SearchAvailablePhoneNumbersFluentBuilder::set_area_code):<br>required: **false**<br><p>Confines a search to just the phone numbers associated with the specified area code.</p><br>
    ///   - [`city(impl Into<String>)`](crate::operation::search_available_phone_numbers::builders::SearchAvailablePhoneNumbersFluentBuilder::city) / [`set_city(Option<String>)`](crate::operation::search_available_phone_numbers::builders::SearchAvailablePhoneNumbersFluentBuilder::set_city):<br>required: **false**<br><p>Confines a search to just the phone numbers associated with the specified city.</p><br>
    ///   - [`country(impl Into<String>)`](crate::operation::search_available_phone_numbers::builders::SearchAvailablePhoneNumbersFluentBuilder::country) / [`set_country(Option<String>)`](crate::operation::search_available_phone_numbers::builders::SearchAvailablePhoneNumbersFluentBuilder::set_country):<br>required: **false**<br><p>Confines a search to just the phone numbers associated with the specified country.</p><br>
    ///   - [`state(impl Into<String>)`](crate::operation::search_available_phone_numbers::builders::SearchAvailablePhoneNumbersFluentBuilder::state) / [`set_state(Option<String>)`](crate::operation::search_available_phone_numbers::builders::SearchAvailablePhoneNumbersFluentBuilder::set_state):<br>required: **false**<br><p>Confines a search to just the phone numbers associated with the specified state.</p><br>
    ///   - [`toll_free_prefix(impl Into<String>)`](crate::operation::search_available_phone_numbers::builders::SearchAvailablePhoneNumbersFluentBuilder::toll_free_prefix) / [`set_toll_free_prefix(Option<String>)`](crate::operation::search_available_phone_numbers::builders::SearchAvailablePhoneNumbersFluentBuilder::set_toll_free_prefix):<br>required: **false**<br><p>Confines a search to just the phone numbers associated with the specified toll-free prefix.</p><br>
    ///   - [`phone_number_type(PhoneNumberType)`](crate::operation::search_available_phone_numbers::builders::SearchAvailablePhoneNumbersFluentBuilder::phone_number_type) / [`set_phone_number_type(Option<PhoneNumberType>)`](crate::operation::search_available_phone_numbers::builders::SearchAvailablePhoneNumbersFluentBuilder::set_phone_number_type):<br>required: **false**<br><p>Confines a search to just the phone numbers associated with the specified phone number type, either <b>local</b> or <b>toll-free</b>.</p><br>
    ///   - [`max_results(i32)`](crate::operation::search_available_phone_numbers::builders::SearchAvailablePhoneNumbersFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::search_available_phone_numbers::builders::SearchAvailablePhoneNumbersFluentBuilder::set_max_results):<br>required: **false**<br><p>The maximum number of results to return.</p><br>
    ///   - [`next_token(impl Into<String>)`](crate::operation::search_available_phone_numbers::builders::SearchAvailablePhoneNumbersFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::search_available_phone_numbers::builders::SearchAvailablePhoneNumbersFluentBuilder::set_next_token):<br>required: **false**<br><p>The token used to return the next page of results.</p><br>
    /// - On success, responds with [`SearchAvailablePhoneNumbersOutput`](crate::operation::search_available_phone_numbers::SearchAvailablePhoneNumbersOutput) with field(s):
    ///   - [`e164_phone_numbers(Option<Vec::<String>>)`](crate::operation::search_available_phone_numbers::SearchAvailablePhoneNumbersOutput::e164_phone_numbers): <p>Confines a search to just the phone numbers in the E.164 format.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::search_available_phone_numbers::SearchAvailablePhoneNumbersOutput::next_token): <p>The token used to return the next page of results.</p>
    /// - On failure, responds with [`SdkError<SearchAvailablePhoneNumbersError>`](crate::operation::search_available_phone_numbers::SearchAvailablePhoneNumbersError)
    pub fn search_available_phone_numbers(
        &self,
    ) -> crate::operation::search_available_phone_numbers::builders::SearchAvailablePhoneNumbersFluentBuilder {
        crate::operation::search_available_phone_numbers::builders::SearchAvailablePhoneNumbersFluentBuilder::new(self.handle.clone())
    }
}
