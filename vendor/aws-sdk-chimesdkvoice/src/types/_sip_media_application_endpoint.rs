// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The endpoint assigned to a SIP media application.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
pub struct SipMediaApplicationEndpoint {
    /// <p>Valid Amazon Resource Name (ARN) of the Lambda function, version, or alias. The function must be created in the same AWS Region as the SIP media application.</p>
    pub lambda_arn: ::std::option::Option<::std::string::String>,
}
impl SipMediaApplicationEndpoint {
    /// <p>Valid Amazon Resource Name (ARN) of the Lambda function, version, or alias. The function must be created in the same AWS Region as the SIP media application.</p>
    pub fn lambda_arn(&self) -> ::std::option::Option<&str> {
        self.lambda_arn.as_deref()
    }
}
impl ::std::fmt::Debug for SipMediaApplicationEndpoint {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("SipMediaApplicationEndpoint");
        formatter.field("lambda_arn", &"*** Sensitive Data Redacted ***");
        formatter.finish()
    }
}
impl SipMediaApplicationEndpoint {
    /// Creates a new builder-style object to manufacture [`SipMediaApplicationEndpoint`](crate::types::SipMediaApplicationEndpoint).
    pub fn builder() -> crate::types::builders::SipMediaApplicationEndpointBuilder {
        crate::types::builders::SipMediaApplicationEndpointBuilder::default()
    }
}

/// A builder for [`SipMediaApplicationEndpoint`](crate::types::SipMediaApplicationEndpoint).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
#[non_exhaustive]
pub struct SipMediaApplicationEndpointBuilder {
    pub(crate) lambda_arn: ::std::option::Option<::std::string::String>,
}
impl SipMediaApplicationEndpointBuilder {
    /// <p>Valid Amazon Resource Name (ARN) of the Lambda function, version, or alias. The function must be created in the same AWS Region as the SIP media application.</p>
    pub fn lambda_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.lambda_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Valid Amazon Resource Name (ARN) of the Lambda function, version, or alias. The function must be created in the same AWS Region as the SIP media application.</p>
    pub fn set_lambda_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.lambda_arn = input;
        self
    }
    /// <p>Valid Amazon Resource Name (ARN) of the Lambda function, version, or alias. The function must be created in the same AWS Region as the SIP media application.</p>
    pub fn get_lambda_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.lambda_arn
    }
    /// Consumes the builder and constructs a [`SipMediaApplicationEndpoint`](crate::types::SipMediaApplicationEndpoint).
    pub fn build(self) -> crate::types::SipMediaApplicationEndpoint {
        crate::types::SipMediaApplicationEndpoint { lambda_arn: self.lambda_arn }
    }
}
impl ::std::fmt::Debug for SipMediaApplicationEndpointBuilder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("SipMediaApplicationEndpointBuilder");
        formatter.field("lambda_arn", &"*** Sensitive Data Redacted ***");
        formatter.finish()
    }
}
