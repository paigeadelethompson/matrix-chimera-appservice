// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct StartDeviceAuthorizationOutput {
    /// <p>The short-lived code that is used by the device when polling for a session token.</p>
    pub device_code: ::std::option::Option<::std::string::String>,
    /// <p>A one-time user verification code. This is needed to authorize an in-use device.</p>
    pub user_code: ::std::option::Option<::std::string::String>,
    /// <p>The URI of the verification page that takes the <code>userCode</code> to authorize the device.</p>
    pub verification_uri: ::std::option::Option<::std::string::String>,
    /// <p>An alternate URL that the client can use to automatically launch a browser. This process skips the manual step in which the user visits the verification page and enters their code.</p>
    pub verification_uri_complete: ::std::option::Option<::std::string::String>,
    /// <p>Indicates the number of seconds in which the verification code will become invalid.</p>
    pub expires_in: i32,
    /// <p>Indicates the number of seconds the client must wait between attempts when polling for a session.</p>
    pub interval: i32,
    _request_id: Option<String>,
}
impl StartDeviceAuthorizationOutput {
    /// <p>The short-lived code that is used by the device when polling for a session token.</p>
    pub fn device_code(&self) -> ::std::option::Option<&str> {
        self.device_code.as_deref()
    }
    /// <p>A one-time user verification code. This is needed to authorize an in-use device.</p>
    pub fn user_code(&self) -> ::std::option::Option<&str> {
        self.user_code.as_deref()
    }
    /// <p>The URI of the verification page that takes the <code>userCode</code> to authorize the device.</p>
    pub fn verification_uri(&self) -> ::std::option::Option<&str> {
        self.verification_uri.as_deref()
    }
    /// <p>An alternate URL that the client can use to automatically launch a browser. This process skips the manual step in which the user visits the verification page and enters their code.</p>
    pub fn verification_uri_complete(&self) -> ::std::option::Option<&str> {
        self.verification_uri_complete.as_deref()
    }
    /// <p>Indicates the number of seconds in which the verification code will become invalid.</p>
    pub fn expires_in(&self) -> i32 {
        self.expires_in
    }
    /// <p>Indicates the number of seconds the client must wait between attempts when polling for a session.</p>
    pub fn interval(&self) -> i32 {
        self.interval
    }
}
impl ::aws_types::request_id::RequestId for StartDeviceAuthorizationOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl StartDeviceAuthorizationOutput {
    /// Creates a new builder-style object to manufacture [`StartDeviceAuthorizationOutput`](crate::operation::start_device_authorization::StartDeviceAuthorizationOutput).
    pub fn builder() -> crate::operation::start_device_authorization::builders::StartDeviceAuthorizationOutputBuilder {
        crate::operation::start_device_authorization::builders::StartDeviceAuthorizationOutputBuilder::default()
    }
}

/// A builder for [`StartDeviceAuthorizationOutput`](crate::operation::start_device_authorization::StartDeviceAuthorizationOutput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct StartDeviceAuthorizationOutputBuilder {
    pub(crate) device_code: ::std::option::Option<::std::string::String>,
    pub(crate) user_code: ::std::option::Option<::std::string::String>,
    pub(crate) verification_uri: ::std::option::Option<::std::string::String>,
    pub(crate) verification_uri_complete: ::std::option::Option<::std::string::String>,
    pub(crate) expires_in: ::std::option::Option<i32>,
    pub(crate) interval: ::std::option::Option<i32>,
    _request_id: Option<String>,
}
impl StartDeviceAuthorizationOutputBuilder {
    /// <p>The short-lived code that is used by the device when polling for a session token.</p>
    pub fn device_code(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.device_code = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The short-lived code that is used by the device when polling for a session token.</p>
    pub fn set_device_code(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.device_code = input;
        self
    }
    /// <p>The short-lived code that is used by the device when polling for a session token.</p>
    pub fn get_device_code(&self) -> &::std::option::Option<::std::string::String> {
        &self.device_code
    }
    /// <p>A one-time user verification code. This is needed to authorize an in-use device.</p>
    pub fn user_code(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.user_code = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A one-time user verification code. This is needed to authorize an in-use device.</p>
    pub fn set_user_code(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.user_code = input;
        self
    }
    /// <p>A one-time user verification code. This is needed to authorize an in-use device.</p>
    pub fn get_user_code(&self) -> &::std::option::Option<::std::string::String> {
        &self.user_code
    }
    /// <p>The URI of the verification page that takes the <code>userCode</code> to authorize the device.</p>
    pub fn verification_uri(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.verification_uri = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The URI of the verification page that takes the <code>userCode</code> to authorize the device.</p>
    pub fn set_verification_uri(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.verification_uri = input;
        self
    }
    /// <p>The URI of the verification page that takes the <code>userCode</code> to authorize the device.</p>
    pub fn get_verification_uri(&self) -> &::std::option::Option<::std::string::String> {
        &self.verification_uri
    }
    /// <p>An alternate URL that the client can use to automatically launch a browser. This process skips the manual step in which the user visits the verification page and enters their code.</p>
    pub fn verification_uri_complete(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.verification_uri_complete = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>An alternate URL that the client can use to automatically launch a browser. This process skips the manual step in which the user visits the verification page and enters their code.</p>
    pub fn set_verification_uri_complete(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.verification_uri_complete = input;
        self
    }
    /// <p>An alternate URL that the client can use to automatically launch a browser. This process skips the manual step in which the user visits the verification page and enters their code.</p>
    pub fn get_verification_uri_complete(&self) -> &::std::option::Option<::std::string::String> {
        &self.verification_uri_complete
    }
    /// <p>Indicates the number of seconds in which the verification code will become invalid.</p>
    pub fn expires_in(mut self, input: i32) -> Self {
        self.expires_in = ::std::option::Option::Some(input);
        self
    }
    /// <p>Indicates the number of seconds in which the verification code will become invalid.</p>
    pub fn set_expires_in(mut self, input: ::std::option::Option<i32>) -> Self {
        self.expires_in = input;
        self
    }
    /// <p>Indicates the number of seconds in which the verification code will become invalid.</p>
    pub fn get_expires_in(&self) -> &::std::option::Option<i32> {
        &self.expires_in
    }
    /// <p>Indicates the number of seconds the client must wait between attempts when polling for a session.</p>
    pub fn interval(mut self, input: i32) -> Self {
        self.interval = ::std::option::Option::Some(input);
        self
    }
    /// <p>Indicates the number of seconds the client must wait between attempts when polling for a session.</p>
    pub fn set_interval(mut self, input: ::std::option::Option<i32>) -> Self {
        self.interval = input;
        self
    }
    /// <p>Indicates the number of seconds the client must wait between attempts when polling for a session.</p>
    pub fn get_interval(&self) -> &::std::option::Option<i32> {
        &self.interval
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`StartDeviceAuthorizationOutput`](crate::operation::start_device_authorization::StartDeviceAuthorizationOutput).
    pub fn build(self) -> crate::operation::start_device_authorization::StartDeviceAuthorizationOutput {
        crate::operation::start_device_authorization::StartDeviceAuthorizationOutput {
            device_code: self.device_code,
            user_code: self.user_code,
            verification_uri: self.verification_uri,
            verification_uri_complete: self.verification_uri_complete,
            expires_in: self.expires_in.unwrap_or_default(),
            interval: self.interval.unwrap_or_default(),
            _request_id: self._request_id,
        }
    }
}