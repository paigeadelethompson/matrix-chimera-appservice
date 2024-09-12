// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DisassociatePhoneNumbersFromVoiceConnectorOutput {
    /// <p>If the action fails for one or more of the phone numbers in the request, a list of the phone numbers is returned, along with error codes and error messages.</p>
    pub phone_number_errors: ::std::option::Option<::std::vec::Vec<crate::types::PhoneNumberError>>,
    _request_id: Option<String>,
}
impl DisassociatePhoneNumbersFromVoiceConnectorOutput {
    /// <p>If the action fails for one or more of the phone numbers in the request, a list of the phone numbers is returned, along with error codes and error messages.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.phone_number_errors.is_none()`.
    pub fn phone_number_errors(&self) -> &[crate::types::PhoneNumberError] {
        self.phone_number_errors.as_deref().unwrap_or_default()
    }
}
impl ::aws_types::request_id::RequestId for DisassociatePhoneNumbersFromVoiceConnectorOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DisassociatePhoneNumbersFromVoiceConnectorOutput {
    /// Creates a new builder-style object to manufacture [`DisassociatePhoneNumbersFromVoiceConnectorOutput`](crate::operation::disassociate_phone_numbers_from_voice_connector::DisassociatePhoneNumbersFromVoiceConnectorOutput).
    pub fn builder(
    ) -> crate::operation::disassociate_phone_numbers_from_voice_connector::builders::DisassociatePhoneNumbersFromVoiceConnectorOutputBuilder {
        crate::operation::disassociate_phone_numbers_from_voice_connector::builders::DisassociatePhoneNumbersFromVoiceConnectorOutputBuilder::default(
        )
    }
}

/// A builder for [`DisassociatePhoneNumbersFromVoiceConnectorOutput`](crate::operation::disassociate_phone_numbers_from_voice_connector::DisassociatePhoneNumbersFromVoiceConnectorOutput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct DisassociatePhoneNumbersFromVoiceConnectorOutputBuilder {
    pub(crate) phone_number_errors: ::std::option::Option<::std::vec::Vec<crate::types::PhoneNumberError>>,
    _request_id: Option<String>,
}
impl DisassociatePhoneNumbersFromVoiceConnectorOutputBuilder {
    /// Appends an item to `phone_number_errors`.
    ///
    /// To override the contents of this collection use [`set_phone_number_errors`](Self::set_phone_number_errors).
    ///
    /// <p>If the action fails for one or more of the phone numbers in the request, a list of the phone numbers is returned, along with error codes and error messages.</p>
    pub fn phone_number_errors(mut self, input: crate::types::PhoneNumberError) -> Self {
        let mut v = self.phone_number_errors.unwrap_or_default();
        v.push(input);
        self.phone_number_errors = ::std::option::Option::Some(v);
        self
    }
    /// <p>If the action fails for one or more of the phone numbers in the request, a list of the phone numbers is returned, along with error codes and error messages.</p>
    pub fn set_phone_number_errors(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::PhoneNumberError>>) -> Self {
        self.phone_number_errors = input;
        self
    }
    /// <p>If the action fails for one or more of the phone numbers in the request, a list of the phone numbers is returned, along with error codes and error messages.</p>
    pub fn get_phone_number_errors(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::PhoneNumberError>> {
        &self.phone_number_errors
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`DisassociatePhoneNumbersFromVoiceConnectorOutput`](crate::operation::disassociate_phone_numbers_from_voice_connector::DisassociatePhoneNumbersFromVoiceConnectorOutput).
    pub fn build(self) -> crate::operation::disassociate_phone_numbers_from_voice_connector::DisassociatePhoneNumbersFromVoiceConnectorOutput {
        crate::operation::disassociate_phone_numbers_from_voice_connector::DisassociatePhoneNumbersFromVoiceConnectorOutput {
            phone_number_errors: self.phone_number_errors,
            _request_id: self._request_id,
        }
    }
}
