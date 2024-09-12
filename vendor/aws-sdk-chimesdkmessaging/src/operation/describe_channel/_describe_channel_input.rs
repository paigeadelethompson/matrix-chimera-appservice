// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeChannelInput {
    /// <p>The ARN of the channel.</p>
    pub channel_arn: ::std::option::Option<::std::string::String>,
    /// <p>The ARN of the <code>AppInstanceUser</code> or <code>AppInstanceBot</code> that makes the API call.</p>
    pub chime_bearer: ::std::option::Option<::std::string::String>,
}
impl DescribeChannelInput {
    /// <p>The ARN of the channel.</p>
    pub fn channel_arn(&self) -> ::std::option::Option<&str> {
        self.channel_arn.as_deref()
    }
    /// <p>The ARN of the <code>AppInstanceUser</code> or <code>AppInstanceBot</code> that makes the API call.</p>
    pub fn chime_bearer(&self) -> ::std::option::Option<&str> {
        self.chime_bearer.as_deref()
    }
}
impl DescribeChannelInput {
    /// Creates a new builder-style object to manufacture [`DescribeChannelInput`](crate::operation::describe_channel::DescribeChannelInput).
    pub fn builder() -> crate::operation::describe_channel::builders::DescribeChannelInputBuilder {
        crate::operation::describe_channel::builders::DescribeChannelInputBuilder::default()
    }
}

/// A builder for [`DescribeChannelInput`](crate::operation::describe_channel::DescribeChannelInput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct DescribeChannelInputBuilder {
    pub(crate) channel_arn: ::std::option::Option<::std::string::String>,
    pub(crate) chime_bearer: ::std::option::Option<::std::string::String>,
}
impl DescribeChannelInputBuilder {
    /// <p>The ARN of the channel.</p>
    /// This field is required.
    pub fn channel_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.channel_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ARN of the channel.</p>
    pub fn set_channel_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.channel_arn = input;
        self
    }
    /// <p>The ARN of the channel.</p>
    pub fn get_channel_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.channel_arn
    }
    /// <p>The ARN of the <code>AppInstanceUser</code> or <code>AppInstanceBot</code> that makes the API call.</p>
    /// This field is required.
    pub fn chime_bearer(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.chime_bearer = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ARN of the <code>AppInstanceUser</code> or <code>AppInstanceBot</code> that makes the API call.</p>
    pub fn set_chime_bearer(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.chime_bearer = input;
        self
    }
    /// <p>The ARN of the <code>AppInstanceUser</code> or <code>AppInstanceBot</code> that makes the API call.</p>
    pub fn get_chime_bearer(&self) -> &::std::option::Option<::std::string::String> {
        &self.chime_bearer
    }
    /// Consumes the builder and constructs a [`DescribeChannelInput`](crate::operation::describe_channel::DescribeChannelInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::operation::describe_channel::DescribeChannelInput, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::operation::describe_channel::DescribeChannelInput {
            channel_arn: self.channel_arn,
            chime_bearer: self.chime_bearer,
        })
    }
}
