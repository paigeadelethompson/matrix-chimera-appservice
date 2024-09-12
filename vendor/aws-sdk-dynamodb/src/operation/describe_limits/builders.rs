// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_limits::_describe_limits_output::DescribeLimitsOutputBuilder;

pub use crate::operation::describe_limits::_describe_limits_input::DescribeLimitsInputBuilder;

impl crate::operation::describe_limits::builders::DescribeLimitsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::describe_limits::DescribeLimitsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_limits::DescribeLimitsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.describe_limits();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DescribeLimits`.
///
/// <p>Returns the current provisioned-capacity quotas for your Amazon Web Services account in a Region, both for the Region as a whole and for any one DynamoDB table that you create there.</p>
/// <p>When you establish an Amazon Web Services account, the account has initial quotas on the maximum read capacity units and write capacity units that you can provision across all of your DynamoDB tables in a given Region. Also, there are per-table quotas that apply when you create a table there. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Limits.html">Service, Account, and Table Quotas</a> page in the <i>Amazon DynamoDB Developer Guide</i>.</p>
/// <p>Although you can increase these quotas by filing a case at <a href="https://console.aws.amazon.com/support/home#/">Amazon Web Services Support Center</a>, obtaining the increase is not instantaneous. The <code>DescribeLimits</code> action lets you write code to compare the capacity you are currently using to those quotas imposed by your account so that you have enough time to apply for an increase before you hit a quota.</p>
/// <p>For example, you could use one of the Amazon Web Services SDKs to do the following:</p>
/// <ol>
/// <li>
/// <p>Call <code>DescribeLimits</code> for a particular Region to obtain your current account quotas on provisioned capacity there.</p></li>
/// <li>
/// <p>Create a variable to hold the aggregate read capacity units provisioned for all your tables in that Region, and one to hold the aggregate write capacity units. Zero them both.</p></li>
/// <li>
/// <p>Call <code>ListTables</code> to obtain a list of all your DynamoDB tables.</p></li>
/// <li>
/// <p>For each table name listed by <code>ListTables</code>, do the following:</p>
/// <ul>
/// <li>
/// <p>Call <code>DescribeTable</code> with the table name.</p></li>
/// <li>
/// <p>Use the data returned by <code>DescribeTable</code> to add the read capacity units and write capacity units provisioned for the table itself to your variables.</p></li>
/// <li>
/// <p>If the table has one or more global secondary indexes (GSIs), loop over these GSIs and add their provisioned capacity values to your variables as well.</p></li>
/// </ul></li>
/// <li>
/// <p>Report the account quotas for that Region returned by <code>DescribeLimits</code>, along with the total current provisioned capacity levels you have calculated.</p></li>
/// </ol>
/// <p>This will let you see whether you are getting close to your account-level quotas.</p>
/// <p>The per-table quotas apply only when you are creating a new table. They restrict the sum of the provisioned capacity of the new table itself and all its global secondary indexes.</p>
/// <p>For existing tables and their GSIs, DynamoDB doesn't let you increase provisioned capacity extremely rapidly, but the only quota that applies is that the aggregate provisioned capacity over all your tables and GSIs cannot exceed either of the per-account quotas.</p><note>
/// <p><code>DescribeLimits</code> should only be called periodically. You can expect throttling errors if you call it more than once in a minute.</p>
/// </note>
/// <p>The <code>DescribeLimits</code> Request element has no content.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DescribeLimitsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::describe_limits::builders::DescribeLimitsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::describe_limits::DescribeLimitsOutput,
        crate::operation::describe_limits::DescribeLimitsError,
    > for DescribeLimitsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::describe_limits::DescribeLimitsOutput,
            crate::operation::describe_limits::DescribeLimitsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DescribeLimitsFluentBuilder {
    /// Creates a new `DescribeLimitsFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DescribeLimits as a reference.
    pub fn as_input(&self) -> &crate::operation::describe_limits::builders::DescribeLimitsInputBuilder {
        &self.inner
    }
    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::describe_limits::DescribeLimitsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_limits::DescribeLimitsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::describe_limits::DescribeLimits::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::describe_limits::DescribeLimits::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::describe_limits::DescribeLimitsOutput,
        crate::operation::describe_limits::DescribeLimitsError,
        Self,
    > {
        crate::client::customize::CustomizableOperation::new(self)
    }
    pub(crate) fn config_override(mut self, config_override: impl ::std::convert::Into<crate::config::Builder>) -> Self {
        self.set_config_override(::std::option::Option::Some(config_override.into()));
        self
    }

    pub(crate) fn set_config_override(&mut self, config_override: ::std::option::Option<crate::config::Builder>) -> &mut Self {
        self.config_override = config_override;
        self
    }
}
