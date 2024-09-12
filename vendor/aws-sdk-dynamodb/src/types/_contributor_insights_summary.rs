// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Represents a Contributor Insights summary entry.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ContributorInsightsSummary {
    /// <p>Name of the table associated with the summary.</p>
    pub table_name: ::std::option::Option<::std::string::String>,
    /// <p>Name of the index associated with the summary, if any.</p>
    pub index_name: ::std::option::Option<::std::string::String>,
    /// <p>Describes the current status for contributor insights for the given table and index, if applicable.</p>
    pub contributor_insights_status: ::std::option::Option<crate::types::ContributorInsightsStatus>,
}
impl ContributorInsightsSummary {
    /// <p>Name of the table associated with the summary.</p>
    pub fn table_name(&self) -> ::std::option::Option<&str> {
        self.table_name.as_deref()
    }
    /// <p>Name of the index associated with the summary, if any.</p>
    pub fn index_name(&self) -> ::std::option::Option<&str> {
        self.index_name.as_deref()
    }
    /// <p>Describes the current status for contributor insights for the given table and index, if applicable.</p>
    pub fn contributor_insights_status(&self) -> ::std::option::Option<&crate::types::ContributorInsightsStatus> {
        self.contributor_insights_status.as_ref()
    }
}
impl ContributorInsightsSummary {
    /// Creates a new builder-style object to manufacture [`ContributorInsightsSummary`](crate::types::ContributorInsightsSummary).
    pub fn builder() -> crate::types::builders::ContributorInsightsSummaryBuilder {
        crate::types::builders::ContributorInsightsSummaryBuilder::default()
    }
}

/// A builder for [`ContributorInsightsSummary`](crate::types::ContributorInsightsSummary).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct ContributorInsightsSummaryBuilder {
    pub(crate) table_name: ::std::option::Option<::std::string::String>,
    pub(crate) index_name: ::std::option::Option<::std::string::String>,
    pub(crate) contributor_insights_status: ::std::option::Option<crate::types::ContributorInsightsStatus>,
}
impl ContributorInsightsSummaryBuilder {
    /// <p>Name of the table associated with the summary.</p>
    pub fn table_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.table_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Name of the table associated with the summary.</p>
    pub fn set_table_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.table_name = input;
        self
    }
    /// <p>Name of the table associated with the summary.</p>
    pub fn get_table_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.table_name
    }
    /// <p>Name of the index associated with the summary, if any.</p>
    pub fn index_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.index_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Name of the index associated with the summary, if any.</p>
    pub fn set_index_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.index_name = input;
        self
    }
    /// <p>Name of the index associated with the summary, if any.</p>
    pub fn get_index_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.index_name
    }
    /// <p>Describes the current status for contributor insights for the given table and index, if applicable.</p>
    pub fn contributor_insights_status(mut self, input: crate::types::ContributorInsightsStatus) -> Self {
        self.contributor_insights_status = ::std::option::Option::Some(input);
        self
    }
    /// <p>Describes the current status for contributor insights for the given table and index, if applicable.</p>
    pub fn set_contributor_insights_status(mut self, input: ::std::option::Option<crate::types::ContributorInsightsStatus>) -> Self {
        self.contributor_insights_status = input;
        self
    }
    /// <p>Describes the current status for contributor insights for the given table and index, if applicable.</p>
    pub fn get_contributor_insights_status(&self) -> &::std::option::Option<crate::types::ContributorInsightsStatus> {
        &self.contributor_insights_status
    }
    /// Consumes the builder and constructs a [`ContributorInsightsSummary`](crate::types::ContributorInsightsSummary).
    pub fn build(self) -> crate::types::ContributorInsightsSummary {
        crate::types::ContributorInsightsSummary {
            table_name: self.table_name,
            index_name: self.index_name,
            contributor_insights_status: self.contributor_insights_status,
        }
    }
}
