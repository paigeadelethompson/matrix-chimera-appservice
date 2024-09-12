// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Represents one of the following:</p>
/// <ul>
/// <li>
/// <p>A new replica to be added to an existing global table.</p></li>
/// <li>
/// <p>New parameters for an existing replica.</p></li>
/// <li>
/// <p>An existing replica to be removed from an existing global table.</p></li>
/// </ul>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ReplicaUpdate {
    /// <p>The parameters required for creating a replica on an existing global table.</p>
    pub create: ::std::option::Option<crate::types::CreateReplicaAction>,
    /// <p>The name of the existing replica to be removed.</p>
    pub delete: ::std::option::Option<crate::types::DeleteReplicaAction>,
}
impl ReplicaUpdate {
    /// <p>The parameters required for creating a replica on an existing global table.</p>
    pub fn create(&self) -> ::std::option::Option<&crate::types::CreateReplicaAction> {
        self.create.as_ref()
    }
    /// <p>The name of the existing replica to be removed.</p>
    pub fn delete(&self) -> ::std::option::Option<&crate::types::DeleteReplicaAction> {
        self.delete.as_ref()
    }
}
impl ReplicaUpdate {
    /// Creates a new builder-style object to manufacture [`ReplicaUpdate`](crate::types::ReplicaUpdate).
    pub fn builder() -> crate::types::builders::ReplicaUpdateBuilder {
        crate::types::builders::ReplicaUpdateBuilder::default()
    }
}

/// A builder for [`ReplicaUpdate`](crate::types::ReplicaUpdate).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct ReplicaUpdateBuilder {
    pub(crate) create: ::std::option::Option<crate::types::CreateReplicaAction>,
    pub(crate) delete: ::std::option::Option<crate::types::DeleteReplicaAction>,
}
impl ReplicaUpdateBuilder {
    /// <p>The parameters required for creating a replica on an existing global table.</p>
    pub fn create(mut self, input: crate::types::CreateReplicaAction) -> Self {
        self.create = ::std::option::Option::Some(input);
        self
    }
    /// <p>The parameters required for creating a replica on an existing global table.</p>
    pub fn set_create(mut self, input: ::std::option::Option<crate::types::CreateReplicaAction>) -> Self {
        self.create = input;
        self
    }
    /// <p>The parameters required for creating a replica on an existing global table.</p>
    pub fn get_create(&self) -> &::std::option::Option<crate::types::CreateReplicaAction> {
        &self.create
    }
    /// <p>The name of the existing replica to be removed.</p>
    pub fn delete(mut self, input: crate::types::DeleteReplicaAction) -> Self {
        self.delete = ::std::option::Option::Some(input);
        self
    }
    /// <p>The name of the existing replica to be removed.</p>
    pub fn set_delete(mut self, input: ::std::option::Option<crate::types::DeleteReplicaAction>) -> Self {
        self.delete = input;
        self
    }
    /// <p>The name of the existing replica to be removed.</p>
    pub fn get_delete(&self) -> &::std::option::Option<crate::types::DeleteReplicaAction> {
        &self.delete
    }
    /// Consumes the builder and constructs a [`ReplicaUpdate`](crate::types::ReplicaUpdate).
    pub fn build(self) -> crate::types::ReplicaUpdate {
        crate::types::ReplicaUpdate {
            create: self.create,
            delete: self.delete,
        }
    }
}
