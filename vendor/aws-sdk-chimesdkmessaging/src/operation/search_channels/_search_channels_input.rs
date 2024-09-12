// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
pub struct SearchChannelsInput {
    /// <p>The <code>AppInstanceUserArn</code> of the user making the API call.</p>
    pub chime_bearer: ::std::option::Option<::std::string::String>,
    /// <p>A list of the <code>Field</code> objects in the channel being searched.</p>
    pub fields: ::std::option::Option<::std::vec::Vec<crate::types::SearchField>>,
    /// <p>The maximum number of channels that you want returned.</p>
    pub max_results: ::std::option::Option<i32>,
    /// <p>The token returned from previous API requests until the number of channels is reached.</p>
    pub next_token: ::std::option::Option<::std::string::String>,
}
impl SearchChannelsInput {
    /// <p>The <code>AppInstanceUserArn</code> of the user making the API call.</p>
    pub fn chime_bearer(&self) -> ::std::option::Option<&str> {
        self.chime_bearer.as_deref()
    }
    /// <p>A list of the <code>Field</code> objects in the channel being searched.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.fields.is_none()`.
    pub fn fields(&self) -> &[crate::types::SearchField] {
        self.fields.as_deref().unwrap_or_default()
    }
    /// <p>The maximum number of channels that you want returned.</p>
    pub fn max_results(&self) -> ::std::option::Option<i32> {
        self.max_results
    }
    /// <p>The token returned from previous API requests until the number of channels is reached.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl ::std::fmt::Debug for SearchChannelsInput {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("SearchChannelsInput");
        formatter.field("chime_bearer", &self.chime_bearer);
        formatter.field("fields", &self.fields);
        formatter.field("max_results", &self.max_results);
        formatter.field("next_token", &"*** Sensitive Data Redacted ***");
        formatter.finish()
    }
}
impl SearchChannelsInput {
    /// Creates a new builder-style object to manufacture [`SearchChannelsInput`](crate::operation::search_channels::SearchChannelsInput).
    pub fn builder() -> crate::operation::search_channels::builders::SearchChannelsInputBuilder {
        crate::operation::search_channels::builders::SearchChannelsInputBuilder::default()
    }
}

/// A builder for [`SearchChannelsInput`](crate::operation::search_channels::SearchChannelsInput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
#[non_exhaustive]
pub struct SearchChannelsInputBuilder {
    pub(crate) chime_bearer: ::std::option::Option<::std::string::String>,
    pub(crate) fields: ::std::option::Option<::std::vec::Vec<crate::types::SearchField>>,
    pub(crate) max_results: ::std::option::Option<i32>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
}
impl SearchChannelsInputBuilder {
    /// <p>The <code>AppInstanceUserArn</code> of the user making the API call.</p>
    pub fn chime_bearer(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.chime_bearer = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The <code>AppInstanceUserArn</code> of the user making the API call.</p>
    pub fn set_chime_bearer(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.chime_bearer = input;
        self
    }
    /// <p>The <code>AppInstanceUserArn</code> of the user making the API call.</p>
    pub fn get_chime_bearer(&self) -> &::std::option::Option<::std::string::String> {
        &self.chime_bearer
    }
    /// Appends an item to `fields`.
    ///
    /// To override the contents of this collection use [`set_fields`](Self::set_fields).
    ///
    /// <p>A list of the <code>Field</code> objects in the channel being searched.</p>
    pub fn fields(mut self, input: crate::types::SearchField) -> Self {
        let mut v = self.fields.unwrap_or_default();
        v.push(input);
        self.fields = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of the <code>Field</code> objects in the channel being searched.</p>
    pub fn set_fields(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::SearchField>>) -> Self {
        self.fields = input;
        self
    }
    /// <p>A list of the <code>Field</code> objects in the channel being searched.</p>
    pub fn get_fields(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::SearchField>> {
        &self.fields
    }
    /// <p>The maximum number of channels that you want returned.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.max_results = ::std::option::Option::Some(input);
        self
    }
    /// <p>The maximum number of channels that you want returned.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.max_results = input;
        self
    }
    /// <p>The maximum number of channels that you want returned.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        &self.max_results
    }
    /// <p>The token returned from previous API requests until the number of channels is reached.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The token returned from previous API requests until the number of channels is reached.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// <p>The token returned from previous API requests until the number of channels is reached.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        &self.next_token
    }
    /// Consumes the builder and constructs a [`SearchChannelsInput`](crate::operation::search_channels::SearchChannelsInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::operation::search_channels::SearchChannelsInput, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::operation::search_channels::SearchChannelsInput {
            chime_bearer: self.chime_bearer,
            fields: self.fields,
            max_results: self.max_results,
            next_token: self.next_token,
        })
    }
}
impl ::std::fmt::Debug for SearchChannelsInputBuilder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("SearchChannelsInputBuilder");
        formatter.field("chime_bearer", &self.chime_bearer);
        formatter.field("fields", &self.fields);
        formatter.field("max_results", &self.max_results);
        formatter.field("next_token", &"*** Sensitive Data Redacted ***");
        formatter.finish()
    }
}
