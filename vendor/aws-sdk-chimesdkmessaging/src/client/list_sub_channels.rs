// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListSubChannels`](crate::operation::list_sub_channels::builders::ListSubChannelsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_sub_channels::builders::ListSubChannelsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`channel_arn(impl Into<String>)`](crate::operation::list_sub_channels::builders::ListSubChannelsFluentBuilder::channel_arn) / [`set_channel_arn(Option<String>)`](crate::operation::list_sub_channels::builders::ListSubChannelsFluentBuilder::set_channel_arn):<br>required: **true**<br><p>The ARN of elastic channel.</p><br>
    ///   - [`chime_bearer(impl Into<String>)`](crate::operation::list_sub_channels::builders::ListSubChannelsFluentBuilder::chime_bearer) / [`set_chime_bearer(Option<String>)`](crate::operation::list_sub_channels::builders::ListSubChannelsFluentBuilder::set_chime_bearer):<br>required: **true**<br><p>The <code>AppInstanceUserArn</code> of the user making the API call.</p><br>
    ///   - [`max_results(i32)`](crate::operation::list_sub_channels::builders::ListSubChannelsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_sub_channels::builders::ListSubChannelsFluentBuilder::set_max_results):<br>required: **false**<br><p>The maximum number of sub-channels that you want to return.</p><br>
    ///   - [`next_token(impl Into<String>)`](crate::operation::list_sub_channels::builders::ListSubChannelsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_sub_channels::builders::ListSubChannelsFluentBuilder::set_next_token):<br>required: **false**<br><p>The token passed by previous API calls until all requested sub-channels are returned.</p><br>
    /// - On success, responds with [`ListSubChannelsOutput`](crate::operation::list_sub_channels::ListSubChannelsOutput) with field(s):
    ///   - [`channel_arn(Option<String>)`](crate::operation::list_sub_channels::ListSubChannelsOutput::channel_arn): <p>The ARN of elastic channel.</p>
    ///   - [`sub_channels(Option<Vec::<SubChannelSummary>>)`](crate::operation::list_sub_channels::ListSubChannelsOutput::sub_channels): <p>The information about each sub-channel.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_sub_channels::ListSubChannelsOutput::next_token): <p>The token passed by previous API calls until all requested sub-channels are returned.</p>
    /// - On failure, responds with [`SdkError<ListSubChannelsError>`](crate::operation::list_sub_channels::ListSubChannelsError)
    pub fn list_sub_channels(&self) -> crate::operation::list_sub_channels::builders::ListSubChannelsFluentBuilder {
        crate::operation::list_sub_channels::builders::ListSubChannelsFluentBuilder::new(self.handle.clone())
    }
}
