// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn reflens_list_account_roles_output_output_next_token(
    input: &crate::operation::list_account_roles::ListAccountRolesOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn reflens_list_accounts_output_output_next_token(
    input: &crate::operation::list_accounts::ListAccountsOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_list_account_roles_output_output_role_list(
    input: crate::operation::list_account_roles::ListAccountRolesOutput,
) -> ::std::option::Option<::std::vec::Vec<crate::types::RoleInfo>> {
    let input = match input.role_list {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_list_accounts_output_output_account_list(
    input: crate::operation::list_accounts::ListAccountsOutput,
) -> ::std::option::Option<::std::vec::Vec<crate::types::AccountInfo>> {
    let input = match input.account_list {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}
