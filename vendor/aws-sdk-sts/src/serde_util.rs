// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn assumed_role_user_correct_errors(
    mut builder: crate::types::builders::AssumedRoleUserBuilder,
) -> crate::types::builders::AssumedRoleUserBuilder {
    if builder.assumed_role_id.is_none() {
        builder.assumed_role_id = Some(Default::default())
    }
    if builder.arn.is_none() {
        builder.arn = Some(Default::default())
    }
    builder
}

pub(crate) fn credentials_correct_errors(mut builder: crate::types::builders::CredentialsBuilder) -> crate::types::builders::CredentialsBuilder {
    if builder.access_key_id.is_none() {
        builder.access_key_id = Some(Default::default())
    }
    if builder.secret_access_key.is_none() {
        builder.secret_access_key = Some(Default::default())
    }
    if builder.session_token.is_none() {
        builder.session_token = Some(Default::default())
    }
    if builder.expiration.is_none() {
        builder.expiration = Some(::aws_smithy_types::DateTime::from_fractional_secs(0, 0_f64))
    }
    builder
}

pub(crate) fn federated_user_correct_errors(
    mut builder: crate::types::builders::FederatedUserBuilder,
) -> crate::types::builders::FederatedUserBuilder {
    if builder.federated_user_id.is_none() {
        builder.federated_user_id = Some(Default::default())
    }
    if builder.arn.is_none() {
        builder.arn = Some(Default::default())
    }
    builder
}
