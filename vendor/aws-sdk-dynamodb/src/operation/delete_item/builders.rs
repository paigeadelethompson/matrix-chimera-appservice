// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_item::_delete_item_output::DeleteItemOutputBuilder;

pub use crate::operation::delete_item::_delete_item_input::DeleteItemInputBuilder;

impl crate::operation::delete_item::builders::DeleteItemInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::delete_item::DeleteItemOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::delete_item::DeleteItemError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.delete_item();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DeleteItem`.
///
/// <p>Deletes a single item in a table by primary key. You can perform a conditional delete operation that deletes the item if it exists, or if it has an expected attribute value.</p>
/// <p>In addition to deleting an item, you can also return the item's attribute values in the same operation, using the <code>ReturnValues</code> parameter.</p>
/// <p>Unless you specify conditions, the <code>DeleteItem</code> is an idempotent operation; running it multiple times on the same item or attribute does <i>not</i> result in an error response.</p>
/// <p>Conditional deletes are useful for deleting items only if specific conditions are met. If those conditions are met, DynamoDB performs the delete. Otherwise, the item is not deleted.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DeleteItemFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::delete_item::builders::DeleteItemInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::delete_item::DeleteItemOutput,
        crate::operation::delete_item::DeleteItemError,
    > for DeleteItemFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::delete_item::DeleteItemOutput,
            crate::operation::delete_item::DeleteItemError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DeleteItemFluentBuilder {
    /// Creates a new `DeleteItemFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DeleteItem as a reference.
    pub fn as_input(&self) -> &crate::operation::delete_item::builders::DeleteItemInputBuilder {
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
        crate::operation::delete_item::DeleteItemOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::delete_item::DeleteItemError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::delete_item::DeleteItem::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::delete_item::DeleteItem::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::delete_item::DeleteItemOutput,
        crate::operation::delete_item::DeleteItemError,
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
    /// <p>The name of the table from which to delete the item. You can also provide the Amazon Resource Name (ARN) of the table in this parameter.</p>
    pub fn table_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.table_name(input.into());
        self
    }
    /// <p>The name of the table from which to delete the item. You can also provide the Amazon Resource Name (ARN) of the table in this parameter.</p>
    pub fn set_table_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_table_name(input);
        self
    }
    /// <p>The name of the table from which to delete the item. You can also provide the Amazon Resource Name (ARN) of the table in this parameter.</p>
    pub fn get_table_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_table_name()
    }
    ///
    /// Adds a key-value pair to `Key`.
    ///
    /// To override the contents of this collection use [`set_key`](Self::set_key).
    ///
    /// <p>A map of attribute names to <code>AttributeValue</code> objects, representing the primary key of the item to delete.</p>
    /// <p>For the primary key, you must provide all of the key attributes. For example, with a simple primary key, you only need to provide a value for the partition key. For a composite primary key, you must provide values for both the partition key and the sort key.</p>
    pub fn key(mut self, k: impl ::std::convert::Into<::std::string::String>, v: crate::types::AttributeValue) -> Self {
        self.inner = self.inner.key(k.into(), v);
        self
    }
    /// <p>A map of attribute names to <code>AttributeValue</code> objects, representing the primary key of the item to delete.</p>
    /// <p>For the primary key, you must provide all of the key attributes. For example, with a simple primary key, you only need to provide a value for the partition key. For a composite primary key, you must provide values for both the partition key and the sort key.</p>
    pub fn set_key(mut self, input: ::std::option::Option<::std::collections::HashMap<::std::string::String, crate::types::AttributeValue>>) -> Self {
        self.inner = self.inner.set_key(input);
        self
    }
    /// <p>A map of attribute names to <code>AttributeValue</code> objects, representing the primary key of the item to delete.</p>
    /// <p>For the primary key, you must provide all of the key attributes. For example, with a simple primary key, you only need to provide a value for the partition key. For a composite primary key, you must provide values for both the partition key and the sort key.</p>
    pub fn get_key(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, crate::types::AttributeValue>> {
        self.inner.get_key()
    }
    ///
    /// Adds a key-value pair to `Expected`.
    ///
    /// To override the contents of this collection use [`set_expected`](Self::set_expected).
    ///
    /// <p>This is a legacy parameter. Use <code>ConditionExpression</code> instead. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/LegacyConditionalParameters.Expected.html">Expected</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    pub fn expected(mut self, k: impl ::std::convert::Into<::std::string::String>, v: crate::types::ExpectedAttributeValue) -> Self {
        self.inner = self.inner.expected(k.into(), v);
        self
    }
    /// <p>This is a legacy parameter. Use <code>ConditionExpression</code> instead. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/LegacyConditionalParameters.Expected.html">Expected</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    pub fn set_expected(
        mut self,
        input: ::std::option::Option<::std::collections::HashMap<::std::string::String, crate::types::ExpectedAttributeValue>>,
    ) -> Self {
        self.inner = self.inner.set_expected(input);
        self
    }
    /// <p>This is a legacy parameter. Use <code>ConditionExpression</code> instead. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/LegacyConditionalParameters.Expected.html">Expected</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    pub fn get_expected(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, crate::types::ExpectedAttributeValue>> {
        self.inner.get_expected()
    }
    /// <p>This is a legacy parameter. Use <code>ConditionExpression</code> instead. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/LegacyConditionalParameters.ConditionalOperator.html">ConditionalOperator</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    pub fn conditional_operator(mut self, input: crate::types::ConditionalOperator) -> Self {
        self.inner = self.inner.conditional_operator(input);
        self
    }
    /// <p>This is a legacy parameter. Use <code>ConditionExpression</code> instead. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/LegacyConditionalParameters.ConditionalOperator.html">ConditionalOperator</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    pub fn set_conditional_operator(mut self, input: ::std::option::Option<crate::types::ConditionalOperator>) -> Self {
        self.inner = self.inner.set_conditional_operator(input);
        self
    }
    /// <p>This is a legacy parameter. Use <code>ConditionExpression</code> instead. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/LegacyConditionalParameters.ConditionalOperator.html">ConditionalOperator</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    pub fn get_conditional_operator(&self) -> &::std::option::Option<crate::types::ConditionalOperator> {
        self.inner.get_conditional_operator()
    }
    /// <p>Use <code>ReturnValues</code> if you want to get the item attributes as they appeared before they were deleted. For <code>DeleteItem</code>, the valid values are:</p>
    /// <ul>
    /// <li>
    /// <p><code>NONE</code> - If <code>ReturnValues</code> is not specified, or if its value is <code>NONE</code>, then nothing is returned. (This setting is the default for <code>ReturnValues</code>.)</p></li>
    /// <li>
    /// <p><code>ALL_OLD</code> - The content of the old item is returned.</p></li>
    /// </ul>
    /// <p>There is no additional cost associated with requesting a return value aside from the small network and processing overhead of receiving a larger response. No read capacity units are consumed.</p><note>
    /// <p>The <code>ReturnValues</code> parameter is used by several DynamoDB operations; however, <code>DeleteItem</code> does not recognize any values other than <code>NONE</code> or <code>ALL_OLD</code>.</p>
    /// </note>
    pub fn return_values(mut self, input: crate::types::ReturnValue) -> Self {
        self.inner = self.inner.return_values(input);
        self
    }
    /// <p>Use <code>ReturnValues</code> if you want to get the item attributes as they appeared before they were deleted. For <code>DeleteItem</code>, the valid values are:</p>
    /// <ul>
    /// <li>
    /// <p><code>NONE</code> - If <code>ReturnValues</code> is not specified, or if its value is <code>NONE</code>, then nothing is returned. (This setting is the default for <code>ReturnValues</code>.)</p></li>
    /// <li>
    /// <p><code>ALL_OLD</code> - The content of the old item is returned.</p></li>
    /// </ul>
    /// <p>There is no additional cost associated with requesting a return value aside from the small network and processing overhead of receiving a larger response. No read capacity units are consumed.</p><note>
    /// <p>The <code>ReturnValues</code> parameter is used by several DynamoDB operations; however, <code>DeleteItem</code> does not recognize any values other than <code>NONE</code> or <code>ALL_OLD</code>.</p>
    /// </note>
    pub fn set_return_values(mut self, input: ::std::option::Option<crate::types::ReturnValue>) -> Self {
        self.inner = self.inner.set_return_values(input);
        self
    }
    /// <p>Use <code>ReturnValues</code> if you want to get the item attributes as they appeared before they were deleted. For <code>DeleteItem</code>, the valid values are:</p>
    /// <ul>
    /// <li>
    /// <p><code>NONE</code> - If <code>ReturnValues</code> is not specified, or if its value is <code>NONE</code>, then nothing is returned. (This setting is the default for <code>ReturnValues</code>.)</p></li>
    /// <li>
    /// <p><code>ALL_OLD</code> - The content of the old item is returned.</p></li>
    /// </ul>
    /// <p>There is no additional cost associated with requesting a return value aside from the small network and processing overhead of receiving a larger response. No read capacity units are consumed.</p><note>
    /// <p>The <code>ReturnValues</code> parameter is used by several DynamoDB operations; however, <code>DeleteItem</code> does not recognize any values other than <code>NONE</code> or <code>ALL_OLD</code>.</p>
    /// </note>
    pub fn get_return_values(&self) -> &::std::option::Option<crate::types::ReturnValue> {
        self.inner.get_return_values()
    }
    /// <p>Determines the level of detail about either provisioned or on-demand throughput consumption that is returned in the response:</p>
    /// <ul>
    /// <li>
    /// <p><code>INDEXES</code> - The response includes the aggregate <code>ConsumedCapacity</code> for the operation, together with <code>ConsumedCapacity</code> for each table and secondary index that was accessed.</p>
    /// <p>Note that some operations, such as <code>GetItem</code> and <code>BatchGetItem</code>, do not access any indexes at all. In these cases, specifying <code>INDEXES</code> will only return <code>ConsumedCapacity</code> information for table(s).</p></li>
    /// <li>
    /// <p><code>TOTAL</code> - The response includes only the aggregate <code>ConsumedCapacity</code> for the operation.</p></li>
    /// <li>
    /// <p><code>NONE</code> - No <code>ConsumedCapacity</code> details are included in the response.</p></li>
    /// </ul>
    pub fn return_consumed_capacity(mut self, input: crate::types::ReturnConsumedCapacity) -> Self {
        self.inner = self.inner.return_consumed_capacity(input);
        self
    }
    /// <p>Determines the level of detail about either provisioned or on-demand throughput consumption that is returned in the response:</p>
    /// <ul>
    /// <li>
    /// <p><code>INDEXES</code> - The response includes the aggregate <code>ConsumedCapacity</code> for the operation, together with <code>ConsumedCapacity</code> for each table and secondary index that was accessed.</p>
    /// <p>Note that some operations, such as <code>GetItem</code> and <code>BatchGetItem</code>, do not access any indexes at all. In these cases, specifying <code>INDEXES</code> will only return <code>ConsumedCapacity</code> information for table(s).</p></li>
    /// <li>
    /// <p><code>TOTAL</code> - The response includes only the aggregate <code>ConsumedCapacity</code> for the operation.</p></li>
    /// <li>
    /// <p><code>NONE</code> - No <code>ConsumedCapacity</code> details are included in the response.</p></li>
    /// </ul>
    pub fn set_return_consumed_capacity(mut self, input: ::std::option::Option<crate::types::ReturnConsumedCapacity>) -> Self {
        self.inner = self.inner.set_return_consumed_capacity(input);
        self
    }
    /// <p>Determines the level of detail about either provisioned or on-demand throughput consumption that is returned in the response:</p>
    /// <ul>
    /// <li>
    /// <p><code>INDEXES</code> - The response includes the aggregate <code>ConsumedCapacity</code> for the operation, together with <code>ConsumedCapacity</code> for each table and secondary index that was accessed.</p>
    /// <p>Note that some operations, such as <code>GetItem</code> and <code>BatchGetItem</code>, do not access any indexes at all. In these cases, specifying <code>INDEXES</code> will only return <code>ConsumedCapacity</code> information for table(s).</p></li>
    /// <li>
    /// <p><code>TOTAL</code> - The response includes only the aggregate <code>ConsumedCapacity</code> for the operation.</p></li>
    /// <li>
    /// <p><code>NONE</code> - No <code>ConsumedCapacity</code> details are included in the response.</p></li>
    /// </ul>
    pub fn get_return_consumed_capacity(&self) -> &::std::option::Option<crate::types::ReturnConsumedCapacity> {
        self.inner.get_return_consumed_capacity()
    }
    /// <p>Determines whether item collection metrics are returned. If set to <code>SIZE</code>, the response includes statistics about item collections, if any, that were modified during the operation are returned in the response. If set to <code>NONE</code> (the default), no statistics are returned.</p>
    pub fn return_item_collection_metrics(mut self, input: crate::types::ReturnItemCollectionMetrics) -> Self {
        self.inner = self.inner.return_item_collection_metrics(input);
        self
    }
    /// <p>Determines whether item collection metrics are returned. If set to <code>SIZE</code>, the response includes statistics about item collections, if any, that were modified during the operation are returned in the response. If set to <code>NONE</code> (the default), no statistics are returned.</p>
    pub fn set_return_item_collection_metrics(mut self, input: ::std::option::Option<crate::types::ReturnItemCollectionMetrics>) -> Self {
        self.inner = self.inner.set_return_item_collection_metrics(input);
        self
    }
    /// <p>Determines whether item collection metrics are returned. If set to <code>SIZE</code>, the response includes statistics about item collections, if any, that were modified during the operation are returned in the response. If set to <code>NONE</code> (the default), no statistics are returned.</p>
    pub fn get_return_item_collection_metrics(&self) -> &::std::option::Option<crate::types::ReturnItemCollectionMetrics> {
        self.inner.get_return_item_collection_metrics()
    }
    /// <p>A condition that must be satisfied in order for a conditional <code>DeleteItem</code> to succeed.</p>
    /// <p>An expression can contain any of the following:</p>
    /// <ul>
    /// <li>
    /// <p>Functions: <code>attribute_exists | attribute_not_exists | attribute_type | contains | begins_with | size</code></p>
    /// <p>These function names are case-sensitive.</p></li>
    /// <li>
    /// <p>Comparison operators: <code>= | &lt;&gt; | &lt; | &gt; | &lt;= | &gt;= | BETWEEN | IN </code></p></li>
    /// <li>
    /// <p>Logical operators: <code>AND | OR | NOT</code></p></li>
    /// </ul>
    /// <p>For more information about condition expressions, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.SpecifyingConditions.html">Condition Expressions</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    pub fn condition_expression(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.condition_expression(input.into());
        self
    }
    /// <p>A condition that must be satisfied in order for a conditional <code>DeleteItem</code> to succeed.</p>
    /// <p>An expression can contain any of the following:</p>
    /// <ul>
    /// <li>
    /// <p>Functions: <code>attribute_exists | attribute_not_exists | attribute_type | contains | begins_with | size</code></p>
    /// <p>These function names are case-sensitive.</p></li>
    /// <li>
    /// <p>Comparison operators: <code>= | &lt;&gt; | &lt; | &gt; | &lt;= | &gt;= | BETWEEN | IN </code></p></li>
    /// <li>
    /// <p>Logical operators: <code>AND | OR | NOT</code></p></li>
    /// </ul>
    /// <p>For more information about condition expressions, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.SpecifyingConditions.html">Condition Expressions</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    pub fn set_condition_expression(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_condition_expression(input);
        self
    }
    /// <p>A condition that must be satisfied in order for a conditional <code>DeleteItem</code> to succeed.</p>
    /// <p>An expression can contain any of the following:</p>
    /// <ul>
    /// <li>
    /// <p>Functions: <code>attribute_exists | attribute_not_exists | attribute_type | contains | begins_with | size</code></p>
    /// <p>These function names are case-sensitive.</p></li>
    /// <li>
    /// <p>Comparison operators: <code>= | &lt;&gt; | &lt; | &gt; | &lt;= | &gt;= | BETWEEN | IN </code></p></li>
    /// <li>
    /// <p>Logical operators: <code>AND | OR | NOT</code></p></li>
    /// </ul>
    /// <p>For more information about condition expressions, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.SpecifyingConditions.html">Condition Expressions</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    pub fn get_condition_expression(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_condition_expression()
    }
    ///
    /// Adds a key-value pair to `ExpressionAttributeNames`.
    ///
    /// To override the contents of this collection use [`set_expression_attribute_names`](Self::set_expression_attribute_names).
    ///
    /// <p>One or more substitution tokens for attribute names in an expression. The following are some use cases for using <code>ExpressionAttributeNames</code>:</p>
    /// <ul>
    /// <li>
    /// <p>To access an attribute whose name conflicts with a DynamoDB reserved word.</p></li>
    /// <li>
    /// <p>To create a placeholder for repeating occurrences of an attribute name in an expression.</p></li>
    /// <li>
    /// <p>To prevent special characters in an attribute name from being misinterpreted in an expression.</p></li>
    /// </ul>
    /// <p>Use the <b>#</b> character in an expression to dereference an attribute name. For example, consider the following attribute name:</p>
    /// <ul>
    /// <li>
    /// <p><code>Percentile</code></p></li>
    /// </ul>
    /// <p>The name of this attribute conflicts with a reserved word, so it cannot be used directly in an expression. (For the complete list of reserved words, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/ReservedWords.html">Reserved Words</a> in the <i>Amazon DynamoDB Developer Guide</i>). To work around this, you could specify the following for <code>ExpressionAttributeNames</code>:</p>
    /// <ul>
    /// <li>
    /// <p><code>{"#P":"Percentile"}</code></p></li>
    /// </ul>
    /// <p>You could then use this substitution in an expression, as in this example:</p>
    /// <ul>
    /// <li>
    /// <p><code>#P = :val</code></p></li>
    /// </ul><note>
    /// <p>Tokens that begin with the <b>:</b> character are <i>expression attribute values</i>, which are placeholders for the actual value at runtime.</p>
    /// </note>
    /// <p>For more information on expression attribute names, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.AccessingItemAttributes.html">Specifying Item Attributes</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    pub fn expression_attribute_names(
        mut self,
        k: impl ::std::convert::Into<::std::string::String>,
        v: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.expression_attribute_names(k.into(), v.into());
        self
    }
    /// <p>One or more substitution tokens for attribute names in an expression. The following are some use cases for using <code>ExpressionAttributeNames</code>:</p>
    /// <ul>
    /// <li>
    /// <p>To access an attribute whose name conflicts with a DynamoDB reserved word.</p></li>
    /// <li>
    /// <p>To create a placeholder for repeating occurrences of an attribute name in an expression.</p></li>
    /// <li>
    /// <p>To prevent special characters in an attribute name from being misinterpreted in an expression.</p></li>
    /// </ul>
    /// <p>Use the <b>#</b> character in an expression to dereference an attribute name. For example, consider the following attribute name:</p>
    /// <ul>
    /// <li>
    /// <p><code>Percentile</code></p></li>
    /// </ul>
    /// <p>The name of this attribute conflicts with a reserved word, so it cannot be used directly in an expression. (For the complete list of reserved words, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/ReservedWords.html">Reserved Words</a> in the <i>Amazon DynamoDB Developer Guide</i>). To work around this, you could specify the following for <code>ExpressionAttributeNames</code>:</p>
    /// <ul>
    /// <li>
    /// <p><code>{"#P":"Percentile"}</code></p></li>
    /// </ul>
    /// <p>You could then use this substitution in an expression, as in this example:</p>
    /// <ul>
    /// <li>
    /// <p><code>#P = :val</code></p></li>
    /// </ul><note>
    /// <p>Tokens that begin with the <b>:</b> character are <i>expression attribute values</i>, which are placeholders for the actual value at runtime.</p>
    /// </note>
    /// <p>For more information on expression attribute names, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.AccessingItemAttributes.html">Specifying Item Attributes</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    pub fn set_expression_attribute_names(
        mut self,
        input: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_expression_attribute_names(input);
        self
    }
    /// <p>One or more substitution tokens for attribute names in an expression. The following are some use cases for using <code>ExpressionAttributeNames</code>:</p>
    /// <ul>
    /// <li>
    /// <p>To access an attribute whose name conflicts with a DynamoDB reserved word.</p></li>
    /// <li>
    /// <p>To create a placeholder for repeating occurrences of an attribute name in an expression.</p></li>
    /// <li>
    /// <p>To prevent special characters in an attribute name from being misinterpreted in an expression.</p></li>
    /// </ul>
    /// <p>Use the <b>#</b> character in an expression to dereference an attribute name. For example, consider the following attribute name:</p>
    /// <ul>
    /// <li>
    /// <p><code>Percentile</code></p></li>
    /// </ul>
    /// <p>The name of this attribute conflicts with a reserved word, so it cannot be used directly in an expression. (For the complete list of reserved words, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/ReservedWords.html">Reserved Words</a> in the <i>Amazon DynamoDB Developer Guide</i>). To work around this, you could specify the following for <code>ExpressionAttributeNames</code>:</p>
    /// <ul>
    /// <li>
    /// <p><code>{"#P":"Percentile"}</code></p></li>
    /// </ul>
    /// <p>You could then use this substitution in an expression, as in this example:</p>
    /// <ul>
    /// <li>
    /// <p><code>#P = :val</code></p></li>
    /// </ul><note>
    /// <p>Tokens that begin with the <b>:</b> character are <i>expression attribute values</i>, which are placeholders for the actual value at runtime.</p>
    /// </note>
    /// <p>For more information on expression attribute names, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.AccessingItemAttributes.html">Specifying Item Attributes</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    pub fn get_expression_attribute_names(
        &self,
    ) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        self.inner.get_expression_attribute_names()
    }
    ///
    /// Adds a key-value pair to `ExpressionAttributeValues`.
    ///
    /// To override the contents of this collection use [`set_expression_attribute_values`](Self::set_expression_attribute_values).
    ///
    /// <p>One or more values that can be substituted in an expression.</p>
    /// <p>Use the <b>:</b> (colon) character in an expression to dereference an attribute value. For example, suppose that you wanted to check whether the value of the <i>ProductStatus</i> attribute was one of the following:</p>
    /// <p><code>Available | Backordered | Discontinued</code></p>
    /// <p>You would first need to specify <code>ExpressionAttributeValues</code> as follows:</p>
    /// <p><code>{ ":avail":{"S":"Available"}, ":back":{"S":"Backordered"}, ":disc":{"S":"Discontinued"} }</code></p>
    /// <p>You could then use these values in an expression, such as this:</p>
    /// <p><code>ProductStatus IN (:avail, :back, :disc)</code></p>
    /// <p>For more information on expression attribute values, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.SpecifyingConditions.html">Condition Expressions</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    pub fn expression_attribute_values(mut self, k: impl ::std::convert::Into<::std::string::String>, v: crate::types::AttributeValue) -> Self {
        self.inner = self.inner.expression_attribute_values(k.into(), v);
        self
    }
    /// <p>One or more values that can be substituted in an expression.</p>
    /// <p>Use the <b>:</b> (colon) character in an expression to dereference an attribute value. For example, suppose that you wanted to check whether the value of the <i>ProductStatus</i> attribute was one of the following:</p>
    /// <p><code>Available | Backordered | Discontinued</code></p>
    /// <p>You would first need to specify <code>ExpressionAttributeValues</code> as follows:</p>
    /// <p><code>{ ":avail":{"S":"Available"}, ":back":{"S":"Backordered"}, ":disc":{"S":"Discontinued"} }</code></p>
    /// <p>You could then use these values in an expression, such as this:</p>
    /// <p><code>ProductStatus IN (:avail, :back, :disc)</code></p>
    /// <p>For more information on expression attribute values, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.SpecifyingConditions.html">Condition Expressions</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    pub fn set_expression_attribute_values(
        mut self,
        input: ::std::option::Option<::std::collections::HashMap<::std::string::String, crate::types::AttributeValue>>,
    ) -> Self {
        self.inner = self.inner.set_expression_attribute_values(input);
        self
    }
    /// <p>One or more values that can be substituted in an expression.</p>
    /// <p>Use the <b>:</b> (colon) character in an expression to dereference an attribute value. For example, suppose that you wanted to check whether the value of the <i>ProductStatus</i> attribute was one of the following:</p>
    /// <p><code>Available | Backordered | Discontinued</code></p>
    /// <p>You would first need to specify <code>ExpressionAttributeValues</code> as follows:</p>
    /// <p><code>{ ":avail":{"S":"Available"}, ":back":{"S":"Backordered"}, ":disc":{"S":"Discontinued"} }</code></p>
    /// <p>You could then use these values in an expression, such as this:</p>
    /// <p><code>ProductStatus IN (:avail, :back, :disc)</code></p>
    /// <p>For more information on expression attribute values, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.SpecifyingConditions.html">Condition Expressions</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    pub fn get_expression_attribute_values(
        &self,
    ) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, crate::types::AttributeValue>> {
        self.inner.get_expression_attribute_values()
    }
    /// <p>An optional parameter that returns the item attributes for a <code>DeleteItem</code> operation that failed a condition check.</p>
    /// <p>There is no additional cost associated with requesting a return value aside from the small network and processing overhead of receiving a larger response. No read capacity units are consumed.</p>
    pub fn return_values_on_condition_check_failure(mut self, input: crate::types::ReturnValuesOnConditionCheckFailure) -> Self {
        self.inner = self.inner.return_values_on_condition_check_failure(input);
        self
    }
    /// <p>An optional parameter that returns the item attributes for a <code>DeleteItem</code> operation that failed a condition check.</p>
    /// <p>There is no additional cost associated with requesting a return value aside from the small network and processing overhead of receiving a larger response. No read capacity units are consumed.</p>
    pub fn set_return_values_on_condition_check_failure(
        mut self,
        input: ::std::option::Option<crate::types::ReturnValuesOnConditionCheckFailure>,
    ) -> Self {
        self.inner = self.inner.set_return_values_on_condition_check_failure(input);
        self
    }
    /// <p>An optional parameter that returns the item attributes for a <code>DeleteItem</code> operation that failed a condition check.</p>
    /// <p>There is no additional cost associated with requesting a return value aside from the small network and processing overhead of receiving a larger response. No read capacity units are consumed.</p>
    pub fn get_return_values_on_condition_check_failure(&self) -> &::std::option::Option<crate::types::ReturnValuesOnConditionCheckFailure> {
        self.inner.get_return_values_on_condition_check_failure()
    }
}