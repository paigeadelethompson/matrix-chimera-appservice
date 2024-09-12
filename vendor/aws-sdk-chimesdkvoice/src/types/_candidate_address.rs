// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A suggested address.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
pub struct CandidateAddress {
    /// <p>The street information of the candidate address.</p>
    pub street_info: ::std::option::Option<::std::string::String>,
    /// <p>The numeric portion of the candidate address.</p>
    pub street_number: ::std::option::Option<::std::string::String>,
    /// <p>The city of the candidate address.</p>
    pub city: ::std::option::Option<::std::string::String>,
    /// <p>The state of the candidate address.</p>
    pub state: ::std::option::Option<::std::string::String>,
    /// <p>The postal code of the candidate address.</p>
    pub postal_code: ::std::option::Option<::std::string::String>,
    /// <p>The zip + 4 or postal code +4 of the candidate address.</p>
    pub postal_code_plus4: ::std::option::Option<::std::string::String>,
    /// <p>The country of the candidate address.</p>
    pub country: ::std::option::Option<::std::string::String>,
}
impl CandidateAddress {
    /// <p>The street information of the candidate address.</p>
    pub fn street_info(&self) -> ::std::option::Option<&str> {
        self.street_info.as_deref()
    }
    /// <p>The numeric portion of the candidate address.</p>
    pub fn street_number(&self) -> ::std::option::Option<&str> {
        self.street_number.as_deref()
    }
    /// <p>The city of the candidate address.</p>
    pub fn city(&self) -> ::std::option::Option<&str> {
        self.city.as_deref()
    }
    /// <p>The state of the candidate address.</p>
    pub fn state(&self) -> ::std::option::Option<&str> {
        self.state.as_deref()
    }
    /// <p>The postal code of the candidate address.</p>
    pub fn postal_code(&self) -> ::std::option::Option<&str> {
        self.postal_code.as_deref()
    }
    /// <p>The zip + 4 or postal code +4 of the candidate address.</p>
    pub fn postal_code_plus4(&self) -> ::std::option::Option<&str> {
        self.postal_code_plus4.as_deref()
    }
    /// <p>The country of the candidate address.</p>
    pub fn country(&self) -> ::std::option::Option<&str> {
        self.country.as_deref()
    }
}
impl ::std::fmt::Debug for CandidateAddress {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("CandidateAddress");
        formatter.field("street_info", &"*** Sensitive Data Redacted ***");
        formatter.field("street_number", &"*** Sensitive Data Redacted ***");
        formatter.field("city", &"*** Sensitive Data Redacted ***");
        formatter.field("state", &"*** Sensitive Data Redacted ***");
        formatter.field("postal_code", &"*** Sensitive Data Redacted ***");
        formatter.field("postal_code_plus4", &"*** Sensitive Data Redacted ***");
        formatter.field("country", &"*** Sensitive Data Redacted ***");
        formatter.finish()
    }
}
impl CandidateAddress {
    /// Creates a new builder-style object to manufacture [`CandidateAddress`](crate::types::CandidateAddress).
    pub fn builder() -> crate::types::builders::CandidateAddressBuilder {
        crate::types::builders::CandidateAddressBuilder::default()
    }
}

/// A builder for [`CandidateAddress`](crate::types::CandidateAddress).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
#[non_exhaustive]
pub struct CandidateAddressBuilder {
    pub(crate) street_info: ::std::option::Option<::std::string::String>,
    pub(crate) street_number: ::std::option::Option<::std::string::String>,
    pub(crate) city: ::std::option::Option<::std::string::String>,
    pub(crate) state: ::std::option::Option<::std::string::String>,
    pub(crate) postal_code: ::std::option::Option<::std::string::String>,
    pub(crate) postal_code_plus4: ::std::option::Option<::std::string::String>,
    pub(crate) country: ::std::option::Option<::std::string::String>,
}
impl CandidateAddressBuilder {
    /// <p>The street information of the candidate address.</p>
    pub fn street_info(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.street_info = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The street information of the candidate address.</p>
    pub fn set_street_info(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.street_info = input;
        self
    }
    /// <p>The street information of the candidate address.</p>
    pub fn get_street_info(&self) -> &::std::option::Option<::std::string::String> {
        &self.street_info
    }
    /// <p>The numeric portion of the candidate address.</p>
    pub fn street_number(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.street_number = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The numeric portion of the candidate address.</p>
    pub fn set_street_number(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.street_number = input;
        self
    }
    /// <p>The numeric portion of the candidate address.</p>
    pub fn get_street_number(&self) -> &::std::option::Option<::std::string::String> {
        &self.street_number
    }
    /// <p>The city of the candidate address.</p>
    pub fn city(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.city = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The city of the candidate address.</p>
    pub fn set_city(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.city = input;
        self
    }
    /// <p>The city of the candidate address.</p>
    pub fn get_city(&self) -> &::std::option::Option<::std::string::String> {
        &self.city
    }
    /// <p>The state of the candidate address.</p>
    pub fn state(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.state = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The state of the candidate address.</p>
    pub fn set_state(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.state = input;
        self
    }
    /// <p>The state of the candidate address.</p>
    pub fn get_state(&self) -> &::std::option::Option<::std::string::String> {
        &self.state
    }
    /// <p>The postal code of the candidate address.</p>
    pub fn postal_code(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.postal_code = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The postal code of the candidate address.</p>
    pub fn set_postal_code(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.postal_code = input;
        self
    }
    /// <p>The postal code of the candidate address.</p>
    pub fn get_postal_code(&self) -> &::std::option::Option<::std::string::String> {
        &self.postal_code
    }
    /// <p>The zip + 4 or postal code +4 of the candidate address.</p>
    pub fn postal_code_plus4(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.postal_code_plus4 = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The zip + 4 or postal code +4 of the candidate address.</p>
    pub fn set_postal_code_plus4(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.postal_code_plus4 = input;
        self
    }
    /// <p>The zip + 4 or postal code +4 of the candidate address.</p>
    pub fn get_postal_code_plus4(&self) -> &::std::option::Option<::std::string::String> {
        &self.postal_code_plus4
    }
    /// <p>The country of the candidate address.</p>
    pub fn country(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.country = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The country of the candidate address.</p>
    pub fn set_country(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.country = input;
        self
    }
    /// <p>The country of the candidate address.</p>
    pub fn get_country(&self) -> &::std::option::Option<::std::string::String> {
        &self.country
    }
    /// Consumes the builder and constructs a [`CandidateAddress`](crate::types::CandidateAddress).
    pub fn build(self) -> crate::types::CandidateAddress {
        crate::types::CandidateAddress {
            street_info: self.street_info,
            street_number: self.street_number,
            city: self.city,
            state: self.state,
            postal_code: self.postal_code,
            postal_code_plus4: self.postal_code_plus4,
            country: self.country,
        }
    }
}
impl ::std::fmt::Debug for CandidateAddressBuilder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("CandidateAddressBuilder");
        formatter.field("street_info", &"*** Sensitive Data Redacted ***");
        formatter.field("street_number", &"*** Sensitive Data Redacted ***");
        formatter.field("city", &"*** Sensitive Data Redacted ***");
        formatter.field("state", &"*** Sensitive Data Redacted ***");
        formatter.field("postal_code", &"*** Sensitive Data Redacted ***");
        formatter.field("postal_code_plus4", &"*** Sensitive Data Redacted ***");
        formatter.field("country", &"*** Sensitive Data Redacted ***");
        formatter.finish()
    }
}
