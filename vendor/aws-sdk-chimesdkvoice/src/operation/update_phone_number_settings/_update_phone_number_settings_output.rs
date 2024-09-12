// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct UpdatePhoneNumberSettingsOutput {
    _request_id: Option<String>,
}
impl ::aws_types::request_id::RequestId for UpdatePhoneNumberSettingsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl UpdatePhoneNumberSettingsOutput {
    /// Creates a new builder-style object to manufacture [`UpdatePhoneNumberSettingsOutput`](crate::operation::update_phone_number_settings::UpdatePhoneNumberSettingsOutput).
    pub fn builder() -> crate::operation::update_phone_number_settings::builders::UpdatePhoneNumberSettingsOutputBuilder {
        crate::operation::update_phone_number_settings::builders::UpdatePhoneNumberSettingsOutputBuilder::default()
    }
}

/// A builder for [`UpdatePhoneNumberSettingsOutput`](crate::operation::update_phone_number_settings::UpdatePhoneNumberSettingsOutput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct UpdatePhoneNumberSettingsOutputBuilder {
    _request_id: Option<String>,
}
impl UpdatePhoneNumberSettingsOutputBuilder {
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`UpdatePhoneNumberSettingsOutput`](crate::operation::update_phone_number_settings::UpdatePhoneNumberSettingsOutput).
    pub fn build(self) -> crate::operation::update_phone_number_settings::UpdatePhoneNumberSettingsOutput {
        crate::operation::update_phone_number_settings::UpdatePhoneNumberSettingsOutput {
            _request_id: self._request_id,
        }
    }
}