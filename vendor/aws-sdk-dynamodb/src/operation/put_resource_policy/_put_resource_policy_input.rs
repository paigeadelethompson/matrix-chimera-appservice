// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct PutResourcePolicyInput {
    /// <p>The Amazon Resource Name (ARN) of the DynamoDB resource to which the policy will be attached. The resources you can specify include tables and streams.</p>
    /// <p>You can control index permissions using the base table's policy. To specify the same permission level for your table and its indexes, you can provide both the table and index Amazon Resource Name (ARN)s in the <code>Resource</code> field of a given <code>Statement</code> in your policy document. Alternatively, to specify different permissions for your table, indexes, or both, you can define multiple <code>Statement</code> fields in your policy document.</p>
    pub resource_arn: ::std::option::Option<::std::string::String>,
    /// <p>An Amazon Web Services resource-based policy document in JSON format.</p>
    /// <ul>
    /// <li>
    /// <p>The maximum size supported for a resource-based policy document is 20 KB. DynamoDB counts whitespaces when calculating the size of a policy against this limit.</p></li>
    /// <li>
    /// <p>Within a resource-based policy, if the action for a DynamoDB service-linked role (SLR) to replicate data for a global table is denied, adding or deleting a replica will fail with an error.</p></li>
    /// </ul>
    /// <p>For a full list of all considerations that apply while attaching a resource-based policy, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/rbac-considerations.html">Resource-based policy considerations</a>.</p>
    pub policy: ::std::option::Option<::std::string::String>,
    /// <p>A string value that you can use to conditionally update your policy. You can provide the revision ID of your existing policy to make mutating requests against that policy.</p><note>
    /// <p>When you provide an expected revision ID, if the revision ID of the existing policy on the resource doesn't match or if there's no policy attached to the resource, your request will be rejected with a <code>PolicyNotFoundException</code>.</p>
    /// </note>
    /// <p>To conditionally attach a policy when no policy exists for the resource, specify <code>NO_POLICY</code> for the revision ID.</p>
    pub expected_revision_id: ::std::option::Option<::std::string::String>,
    /// <p>Set this parameter to <code>true</code> to confirm that you want to remove your permissions to change the policy of this resource in the future.</p>
    pub confirm_remove_self_resource_access: ::std::option::Option<bool>,
}
impl PutResourcePolicyInput {
    /// <p>The Amazon Resource Name (ARN) of the DynamoDB resource to which the policy will be attached. The resources you can specify include tables and streams.</p>
    /// <p>You can control index permissions using the base table's policy. To specify the same permission level for your table and its indexes, you can provide both the table and index Amazon Resource Name (ARN)s in the <code>Resource</code> field of a given <code>Statement</code> in your policy document. Alternatively, to specify different permissions for your table, indexes, or both, you can define multiple <code>Statement</code> fields in your policy document.</p>
    pub fn resource_arn(&self) -> ::std::option::Option<&str> {
        self.resource_arn.as_deref()
    }
    /// <p>An Amazon Web Services resource-based policy document in JSON format.</p>
    /// <ul>
    /// <li>
    /// <p>The maximum size supported for a resource-based policy document is 20 KB. DynamoDB counts whitespaces when calculating the size of a policy against this limit.</p></li>
    /// <li>
    /// <p>Within a resource-based policy, if the action for a DynamoDB service-linked role (SLR) to replicate data for a global table is denied, adding or deleting a replica will fail with an error.</p></li>
    /// </ul>
    /// <p>For a full list of all considerations that apply while attaching a resource-based policy, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/rbac-considerations.html">Resource-based policy considerations</a>.</p>
    pub fn policy(&self) -> ::std::option::Option<&str> {
        self.policy.as_deref()
    }
    /// <p>A string value that you can use to conditionally update your policy. You can provide the revision ID of your existing policy to make mutating requests against that policy.</p><note>
    /// <p>When you provide an expected revision ID, if the revision ID of the existing policy on the resource doesn't match or if there's no policy attached to the resource, your request will be rejected with a <code>PolicyNotFoundException</code>.</p>
    /// </note>
    /// <p>To conditionally attach a policy when no policy exists for the resource, specify <code>NO_POLICY</code> for the revision ID.</p>
    pub fn expected_revision_id(&self) -> ::std::option::Option<&str> {
        self.expected_revision_id.as_deref()
    }
    /// <p>Set this parameter to <code>true</code> to confirm that you want to remove your permissions to change the policy of this resource in the future.</p>
    pub fn confirm_remove_self_resource_access(&self) -> ::std::option::Option<bool> {
        self.confirm_remove_self_resource_access
    }
}
impl PutResourcePolicyInput {
    /// Creates a new builder-style object to manufacture [`PutResourcePolicyInput`](crate::operation::put_resource_policy::PutResourcePolicyInput).
    pub fn builder() -> crate::operation::put_resource_policy::builders::PutResourcePolicyInputBuilder {
        crate::operation::put_resource_policy::builders::PutResourcePolicyInputBuilder::default()
    }
}

/// A builder for [`PutResourcePolicyInput`](crate::operation::put_resource_policy::PutResourcePolicyInput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct PutResourcePolicyInputBuilder {
    pub(crate) resource_arn: ::std::option::Option<::std::string::String>,
    pub(crate) policy: ::std::option::Option<::std::string::String>,
    pub(crate) expected_revision_id: ::std::option::Option<::std::string::String>,
    pub(crate) confirm_remove_self_resource_access: ::std::option::Option<bool>,
}
impl PutResourcePolicyInputBuilder {
    /// <p>The Amazon Resource Name (ARN) of the DynamoDB resource to which the policy will be attached. The resources you can specify include tables and streams.</p>
    /// <p>You can control index permissions using the base table's policy. To specify the same permission level for your table and its indexes, you can provide both the table and index Amazon Resource Name (ARN)s in the <code>Resource</code> field of a given <code>Statement</code> in your policy document. Alternatively, to specify different permissions for your table, indexes, or both, you can define multiple <code>Statement</code> fields in your policy document.</p>
    /// This field is required.
    pub fn resource_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.resource_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the DynamoDB resource to which the policy will be attached. The resources you can specify include tables and streams.</p>
    /// <p>You can control index permissions using the base table's policy. To specify the same permission level for your table and its indexes, you can provide both the table and index Amazon Resource Name (ARN)s in the <code>Resource</code> field of a given <code>Statement</code> in your policy document. Alternatively, to specify different permissions for your table, indexes, or both, you can define multiple <code>Statement</code> fields in your policy document.</p>
    pub fn set_resource_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.resource_arn = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the DynamoDB resource to which the policy will be attached. The resources you can specify include tables and streams.</p>
    /// <p>You can control index permissions using the base table's policy. To specify the same permission level for your table and its indexes, you can provide both the table and index Amazon Resource Name (ARN)s in the <code>Resource</code> field of a given <code>Statement</code> in your policy document. Alternatively, to specify different permissions for your table, indexes, or both, you can define multiple <code>Statement</code> fields in your policy document.</p>
    pub fn get_resource_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.resource_arn
    }
    /// <p>An Amazon Web Services resource-based policy document in JSON format.</p>
    /// <ul>
    /// <li>
    /// <p>The maximum size supported for a resource-based policy document is 20 KB. DynamoDB counts whitespaces when calculating the size of a policy against this limit.</p></li>
    /// <li>
    /// <p>Within a resource-based policy, if the action for a DynamoDB service-linked role (SLR) to replicate data for a global table is denied, adding or deleting a replica will fail with an error.</p></li>
    /// </ul>
    /// <p>For a full list of all considerations that apply while attaching a resource-based policy, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/rbac-considerations.html">Resource-based policy considerations</a>.</p>
    /// This field is required.
    pub fn policy(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.policy = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>An Amazon Web Services resource-based policy document in JSON format.</p>
    /// <ul>
    /// <li>
    /// <p>The maximum size supported for a resource-based policy document is 20 KB. DynamoDB counts whitespaces when calculating the size of a policy against this limit.</p></li>
    /// <li>
    /// <p>Within a resource-based policy, if the action for a DynamoDB service-linked role (SLR) to replicate data for a global table is denied, adding or deleting a replica will fail with an error.</p></li>
    /// </ul>
    /// <p>For a full list of all considerations that apply while attaching a resource-based policy, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/rbac-considerations.html">Resource-based policy considerations</a>.</p>
    pub fn set_policy(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.policy = input;
        self
    }
    /// <p>An Amazon Web Services resource-based policy document in JSON format.</p>
    /// <ul>
    /// <li>
    /// <p>The maximum size supported for a resource-based policy document is 20 KB. DynamoDB counts whitespaces when calculating the size of a policy against this limit.</p></li>
    /// <li>
    /// <p>Within a resource-based policy, if the action for a DynamoDB service-linked role (SLR) to replicate data for a global table is denied, adding or deleting a replica will fail with an error.</p></li>
    /// </ul>
    /// <p>For a full list of all considerations that apply while attaching a resource-based policy, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/rbac-considerations.html">Resource-based policy considerations</a>.</p>
    pub fn get_policy(&self) -> &::std::option::Option<::std::string::String> {
        &self.policy
    }
    /// <p>A string value that you can use to conditionally update your policy. You can provide the revision ID of your existing policy to make mutating requests against that policy.</p><note>
    /// <p>When you provide an expected revision ID, if the revision ID of the existing policy on the resource doesn't match or if there's no policy attached to the resource, your request will be rejected with a <code>PolicyNotFoundException</code>.</p>
    /// </note>
    /// <p>To conditionally attach a policy when no policy exists for the resource, specify <code>NO_POLICY</code> for the revision ID.</p>
    pub fn expected_revision_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.expected_revision_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A string value that you can use to conditionally update your policy. You can provide the revision ID of your existing policy to make mutating requests against that policy.</p><note>
    /// <p>When you provide an expected revision ID, if the revision ID of the existing policy on the resource doesn't match or if there's no policy attached to the resource, your request will be rejected with a <code>PolicyNotFoundException</code>.</p>
    /// </note>
    /// <p>To conditionally attach a policy when no policy exists for the resource, specify <code>NO_POLICY</code> for the revision ID.</p>
    pub fn set_expected_revision_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.expected_revision_id = input;
        self
    }
    /// <p>A string value that you can use to conditionally update your policy. You can provide the revision ID of your existing policy to make mutating requests against that policy.</p><note>
    /// <p>When you provide an expected revision ID, if the revision ID of the existing policy on the resource doesn't match or if there's no policy attached to the resource, your request will be rejected with a <code>PolicyNotFoundException</code>.</p>
    /// </note>
    /// <p>To conditionally attach a policy when no policy exists for the resource, specify <code>NO_POLICY</code> for the revision ID.</p>
    pub fn get_expected_revision_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.expected_revision_id
    }
    /// <p>Set this parameter to <code>true</code> to confirm that you want to remove your permissions to change the policy of this resource in the future.</p>
    pub fn confirm_remove_self_resource_access(mut self, input: bool) -> Self {
        self.confirm_remove_self_resource_access = ::std::option::Option::Some(input);
        self
    }
    /// <p>Set this parameter to <code>true</code> to confirm that you want to remove your permissions to change the policy of this resource in the future.</p>
    pub fn set_confirm_remove_self_resource_access(mut self, input: ::std::option::Option<bool>) -> Self {
        self.confirm_remove_self_resource_access = input;
        self
    }
    /// <p>Set this parameter to <code>true</code> to confirm that you want to remove your permissions to change the policy of this resource in the future.</p>
    pub fn get_confirm_remove_self_resource_access(&self) -> &::std::option::Option<bool> {
        &self.confirm_remove_self_resource_access
    }
    /// Consumes the builder and constructs a [`PutResourcePolicyInput`](crate::operation::put_resource_policy::PutResourcePolicyInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::operation::put_resource_policy::PutResourcePolicyInput, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::operation::put_resource_policy::PutResourcePolicyInput {
            resource_arn: self.resource_arn,
            policy: self.policy,
            expected_revision_id: self.expected_revision_id,
            confirm_remove_self_resource_access: self.confirm_remove_self_resource_access,
        })
    }
}
