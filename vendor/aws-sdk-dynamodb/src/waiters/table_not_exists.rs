// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

///
/// Fluent builder for the `table_not_exists` waiter.
///
/// This builder is intended to be used similar to the other fluent builders for
/// normal operations on the client. However, instead of a `send` method, it has
/// a `wait` method that takes a maximum amount of time to wait.
///
/// Construct this fluent builder using the client by importing the
/// [`Waiters`](crate::client::Waiters) trait and calling the methods
/// prefixed with `wait_until`.
///
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct TableNotExistsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::describe_table::builders::DescribeTableInputBuilder,
}
impl TableNotExistsFluentBuilder {
    /// Creates a new `TableNotExistsFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
        }
    }
    /// Access the DescribeTable as a reference.
    pub fn as_input(&self) -> &crate::operation::describe_table::builders::DescribeTableInputBuilder {
        &self.inner
    }
    /// Wait for `table_not_exists`
    pub async fn wait(
        self,
        max_wait: ::std::time::Duration,
    ) -> ::std::result::Result<
        crate::waiters::table_not_exists::TableNotExistsFinalPoll,
        crate::waiters::table_not_exists::WaitUntilTableNotExistsError,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::waiters::error::WaiterError::construction_failure)?;
        let runtime_plugins = crate::operation::describe_table::DescribeTable::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            ::std::option::Option::None,
        )
        .with_operation_plugin(crate::sdk_feature_tracker::waiter::WaiterFeatureTrackerRuntimePlugin::new());
        let mut cfg = ::aws_smithy_types::config_bag::ConfigBag::base();
        let runtime_components_builder = runtime_plugins
            .apply_client_configuration(&mut cfg)
            .map_err(::aws_smithy_runtime_api::client::waiters::error::WaiterError::construction_failure)?;
        let time_components = runtime_components_builder.into_time_components();
        let sleep_impl = time_components.sleep_impl().expect("a sleep impl is required by waiters");
        let time_source = time_components.time_source().expect("a time source is required by waiters");

        let acceptor = move |result: ::std::result::Result<
            &crate::operation::describe_table::DescribeTableOutput,
            &crate::operation::describe_table::DescribeTableError,
        >| {
            // Matches: {"errorType":"ResourceNotFoundException"}
            if crate::waiters::matchers::match_describe_table_1cce2c05524fb92d4(result) {
                return ::aws_smithy_runtime::client::waiters::AcceptorState::Success;
            }
            ::aws_smithy_runtime::client::waiters::AcceptorState::NoAcceptorsMatched
        };
        let operation = move || {
            let input = input.clone();
            let runtime_plugins = runtime_plugins.clone();
            async move { crate::operation::describe_table::DescribeTable::orchestrate(&runtime_plugins, input).await }
        };
        let orchestrator = ::aws_smithy_runtime::client::waiters::WaiterOrchestrator::builder()
            .min_delay(::std::time::Duration::from_secs(20))
            .max_delay(::std::time::Duration::from_secs(120))
            .max_wait(max_wait)
            .time_source(time_source)
            .sleep_impl(sleep_impl)
            .acceptor(acceptor)
            .operation(operation)
            .build();
        ::aws_smithy_runtime::client::waiters::attach_waiter_tracing_span(orchestrator.orchestrate()).await
    }
    /// <p>The name of the table to describe. You can also provide the Amazon Resource Name (ARN) of the table in this parameter.</p>
    pub fn table_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.table_name(input.into());
        self
    }
    /// <p>The name of the table to describe. You can also provide the Amazon Resource Name (ARN) of the table in this parameter.</p>
    pub fn set_table_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_table_name(input);
        self
    }
    /// <p>The name of the table to describe. You can also provide the Amazon Resource Name (ARN) of the table in this parameter.</p>
    pub fn get_table_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_table_name()
    }
}

/// Successful return type for the `table_not_exists` waiter.
pub type TableNotExistsFinalPoll = ::aws_smithy_runtime_api::client::waiters::FinalPoll<
    crate::operation::describe_table::DescribeTableOutput,
    ::aws_smithy_runtime_api::client::result::SdkError<
        crate::operation::describe_table::DescribeTableError,
        ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
    >,
>;

/// Error type for the `table_not_exists` waiter.
pub type WaitUntilTableNotExistsError = ::aws_smithy_runtime_api::client::waiters::error::WaiterError<
    crate::operation::describe_table::DescribeTableOutput,
    crate::operation::describe_table::DescribeTableError,
>;
