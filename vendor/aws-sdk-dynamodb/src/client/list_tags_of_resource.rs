// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListTagsOfResource`](crate::operation::list_tags_of_resource::builders::ListTagsOfResourceFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`resource_arn(impl Into<String>)`](crate::operation::list_tags_of_resource::builders::ListTagsOfResourceFluentBuilder::resource_arn) / [`set_resource_arn(Option<String>)`](crate::operation::list_tags_of_resource::builders::ListTagsOfResourceFluentBuilder::set_resource_arn):<br>required: **true**<br><p>The Amazon DynamoDB resource with tags to be listed. This value is an Amazon Resource Name (ARN).</p><br>
    ///   - [`next_token(impl Into<String>)`](crate::operation::list_tags_of_resource::builders::ListTagsOfResourceFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_tags_of_resource::builders::ListTagsOfResourceFluentBuilder::set_next_token):<br>required: **false**<br><p>An optional string that, if supplied, must be copied from the output of a previous call to ListTagOfResource. When provided in this manner, this API fetches the next page of results.</p><br>
    /// - On success, responds with [`ListTagsOfResourceOutput`](crate::operation::list_tags_of_resource::ListTagsOfResourceOutput) with field(s):
    ///   - [`tags(Option<Vec::<Tag>>)`](crate::operation::list_tags_of_resource::ListTagsOfResourceOutput::tags): <p>The tags currently associated with the Amazon DynamoDB resource.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_tags_of_resource::ListTagsOfResourceOutput::next_token): <p>If this value is returned, there are additional results to be displayed. To retrieve them, call ListTagsOfResource again, with NextToken set to this value.</p>
    /// - On failure, responds with [`SdkError<ListTagsOfResourceError>`](crate::operation::list_tags_of_resource::ListTagsOfResourceError)
    pub fn list_tags_of_resource(&self) -> crate::operation::list_tags_of_resource::builders::ListTagsOfResourceFluentBuilder {
        crate::operation::list_tags_of_resource::builders::ListTagsOfResourceFluentBuilder::new(self.handle.clone())
    }
}
