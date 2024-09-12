// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`PutResourcePolicy`](crate::operation::put_resource_policy::builders::PutResourcePolicyFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`resource_arn(impl Into<String>)`](crate::operation::put_resource_policy::builders::PutResourcePolicyFluentBuilder::resource_arn) / [`set_resource_arn(Option<String>)`](crate::operation::put_resource_policy::builders::PutResourcePolicyFluentBuilder::set_resource_arn):<br>required: **true**<br><p>The Amazon Resource Name (ARN) of the DynamoDB resource to which the policy will be attached. The resources you can specify include tables and streams.</p> <p>You can control index permissions using the base table's policy. To specify the same permission level for your table and its indexes, you can provide both the table and index Amazon Resource Name (ARN)s in the <code>Resource</code> field of a given <code>Statement</code> in your policy document. Alternatively, to specify different permissions for your table, indexes, or both, you can define multiple <code>Statement</code> fields in your policy document.</p><br>
    ///   - [`policy(impl Into<String>)`](crate::operation::put_resource_policy::builders::PutResourcePolicyFluentBuilder::policy) / [`set_policy(Option<String>)`](crate::operation::put_resource_policy::builders::PutResourcePolicyFluentBuilder::set_policy):<br>required: **true**<br><p>An Amazon Web Services resource-based policy document in JSON format.</p> <ul>  <li>   <p>The maximum size supported for a resource-based policy document is 20 KB. DynamoDB counts whitespaces when calculating the size of a policy against this limit.</p></li>  <li>   <p>Within a resource-based policy, if the action for a DynamoDB service-linked role (SLR) to replicate data for a global table is denied, adding or deleting a replica will fail with an error.</p></li> </ul> <p>For a full list of all considerations that apply while attaching a resource-based policy, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/rbac-considerations.html">Resource-based policy considerations</a>.</p><br>
    ///   - [`expected_revision_id(impl Into<String>)`](crate::operation::put_resource_policy::builders::PutResourcePolicyFluentBuilder::expected_revision_id) / [`set_expected_revision_id(Option<String>)`](crate::operation::put_resource_policy::builders::PutResourcePolicyFluentBuilder::set_expected_revision_id):<br>required: **false**<br><p>A string value that you can use to conditionally update your policy. You can provide the revision ID of your existing policy to make mutating requests against that policy.</p><note>  <p>When you provide an expected revision ID, if the revision ID of the existing policy on the resource doesn't match or if there's no policy attached to the resource, your request will be rejected with a <code>PolicyNotFoundException</code>.</p> </note> <p>To conditionally attach a policy when no policy exists for the resource, specify <code>NO_POLICY</code> for the revision ID.</p><br>
    ///   - [`confirm_remove_self_resource_access(bool)`](crate::operation::put_resource_policy::builders::PutResourcePolicyFluentBuilder::confirm_remove_self_resource_access) / [`set_confirm_remove_self_resource_access(Option<bool>)`](crate::operation::put_resource_policy::builders::PutResourcePolicyFluentBuilder::set_confirm_remove_self_resource_access):<br>required: **false**<br><p>Set this parameter to <code>true</code> to confirm that you want to remove your permissions to change the policy of this resource in the future.</p><br>
    /// - On success, responds with [`PutResourcePolicyOutput`](crate::operation::put_resource_policy::PutResourcePolicyOutput) with field(s):
    ///   - [`revision_id(Option<String>)`](crate::operation::put_resource_policy::PutResourcePolicyOutput::revision_id): <p>A unique string that represents the revision ID of the policy. If you're comparing revision IDs, make sure to always use string comparison logic.</p>
    /// - On failure, responds with [`SdkError<PutResourcePolicyError>`](crate::operation::put_resource_policy::PutResourcePolicyError)
    pub fn put_resource_policy(&self) -> crate::operation::put_resource_policy::builders::PutResourcePolicyFluentBuilder {
        crate::operation::put_resource_policy::builders::PutResourcePolicyFluentBuilder::new(self.handle.clone())
    }
}
