// Copyright (c) The dgc.network
// SPDX-License-Identifier: Apache-2.0

use sabre_sdk::protocol::payload::{Action, SabrePayload};
use sabre_sdk::protos::FromBytes;
//use crate::protocol::payload::{Action, SabrePayload};
//use crate::protos::FromBytes;
use sawtooth_sdk::processor::handler::ApplyError;

pub struct SabreRequestPayload {
    action: Action,
}

impl SabreRequestPayload {
    pub fn new(payload: &[u8]) -> Result<Option<SabreRequestPayload>, ApplyError> {
        let payload = match SabrePayload::from_bytes(payload) {
            Ok(payload) => payload,
            Err(_) => {
                return Err(ApplyError::InvalidTransaction(String::from(
                    "Cannot deserialize payload",
                )));
            }
        };

        let sabre_action = payload.action();
        match sabre_action {
            Action::CreateContract(create_contract) => {
                if create_contract.name().is_empty() {
                    return Err(ApplyError::InvalidTransaction(String::from(
                        "Contract name cannot be an empty string",
                    )));
                }
                if create_contract.version().is_empty() {
                    return Err(ApplyError::InvalidTransaction(String::from(
                        "Contract version cannot be an empty string",
                    )));
                }
                if create_contract.inputs().is_empty() {
                    return Err(ApplyError::InvalidTransaction(String::from(
                        "Contract inputs cannot be an empty",
                    )));
                }
                if create_contract.outputs().is_empty() {
                    return Err(ApplyError::InvalidTransaction(String::from(
                        "Contract outputs cannot be an empty",
                    )));
                }
                if create_contract.contract().is_empty() {
                    return Err(ApplyError::InvalidTransaction(String::from(
                        "Contract bytes cannot be an empty",
                    )));
                }
            }
            Action::DeleteContract(delete_contract) => {
                if delete_contract.name().is_empty() {
                    return Err(ApplyError::InvalidTransaction(String::from(
                        "Contract name cannot be an empty string",
                    )));
                }
                if delete_contract.version().is_empty() {
                    return Err(ApplyError::InvalidTransaction(String::from(
                        "Contract version cannot be an empty string",
                    )));
                }
            }
            Action::ExecuteContract(execute_contract) => {
                if execute_contract.name().is_empty() {
                    return Err(ApplyError::InvalidTransaction(String::from(
                        "Contract name cannot be an empty string",
                    )));
                }
                if execute_contract.version().is_empty() {
                    return Err(ApplyError::InvalidTransaction(String::from(
                        "Contract version cannot be an empty string",
                    )));
                }
                if execute_contract.inputs().is_empty() {
                    return Err(ApplyError::InvalidTransaction(String::from(
                        "Contract inputs cannot be an empty",
                    )));
                }
                if execute_contract.outputs().is_empty() {
                    return Err(ApplyError::InvalidTransaction(String::from(
                        "Contract outputs cannot be an empty",
                    )));
                }
                if execute_contract.payload().is_empty() {
                    return Err(ApplyError::InvalidTransaction(String::from(
                        "Contract payload cannot be an empty",
                    )));
                }
            }
            Action::CreateContractRegistry(create_contract_registry) => {
                if create_contract_registry.name().is_empty() {
                    return Err(ApplyError::InvalidTransaction(String::from(
                        "Contract Registry name cannot be an empty string",
                    )));
                }
                if create_contract_registry.owners().is_empty() {
                    return Err(ApplyError::InvalidTransaction(String::from(
                        "Contract Registry owners cannot be an empty",
                    )));
                }
            }
            Action::DeleteContractRegistry(delete_contract_registry) => {
                if delete_contract_registry.name().is_empty() {
                    return Err(ApplyError::InvalidTransaction(String::from(
                        "Contract Registry name cannot be an empty string",
                    )));
                };
            }
            Action::UpdateContractRegistryOwners(update_contract_registry_owners) => {
                if update_contract_registry_owners.name().is_empty() {
                    return Err(ApplyError::InvalidTransaction(String::from(
                        "Contract Registry name cannot be an empty string",
                    )));
                }
                if update_contract_registry_owners.owners().is_empty() {
                    return Err(ApplyError::InvalidTransaction(String::from(
                        "Contract Registry owners cannot be an empty",
                    )));
                }
            }
            Action::CreateNamespaceRegistry(create_namespace_registry) => {
                if create_namespace_registry.namespace().is_empty() {
                    return Err(ApplyError::InvalidTransaction(String::from(
                        "Namespace Registry namespace cannot be an empty string",
                    )));
                }
                if create_namespace_registry.owners().is_empty() {
                    return Err(ApplyError::InvalidTransaction(String::from(
                        "Namespace owners cannot be an empty",
                    )));
                }
            }
            Action::DeleteNamespaceRegistry(delete_namespace_registry) => {
                if delete_namespace_registry.namespace().is_empty() {
                    return Err(ApplyError::InvalidTransaction(String::from(
                        "Namespace Registry namespace cannot be an empty string",
                    )));
                }
            }
            Action::UpdateNamespaceRegistryOwners(update_namespace_registry_owners) => {
                if update_namespace_registry_owners.namespace().is_empty() {
                    return Err(ApplyError::InvalidTransaction(String::from(
                        "Namespace Registry namespace cannot be an empty string",
                    )));
                }
                if update_namespace_registry_owners.owners().is_empty() {
                    return Err(ApplyError::InvalidTransaction(String::from(
                        "Namespace owners cannot be an empty",
                    )));
                }
            }
            Action::CreateNamespaceRegistryPermission(create_namespace_registry_permission) => {
                if create_namespace_registry_permission.namespace().is_empty() {
                    return Err(ApplyError::InvalidTransaction(String::from(
                        "Namespace Registry namespace cannot be an empty string",
                    )));
                }
                if create_namespace_registry_permission
                    .contract_name()
                    .is_empty()
                {
                    return Err(ApplyError::InvalidTransaction(String::from(
                        "Contract name cannot be an empty string",
                    )));
                }
            }
            Action::DeleteNamespaceRegistryPermission(delete_namespace_registry_permission) => {
                if delete_namespace_registry_permission.namespace().is_empty() {
                    return Err(ApplyError::InvalidTransaction(String::from(
                        "Namespace Registry namespace cannot be an empty string",
                    )));
                }
                if delete_namespace_registry_permission
                    .contract_name()
                    .is_empty()
                {
                    return Err(ApplyError::InvalidTransaction(String::from(
                        "Contract name cannot be an empty string",
                    )));
                }
            }
            Action::CreateSmartPermission(create_smart_permission) => {
                if create_smart_permission.org_id().is_empty() {
                    return Err(ApplyError::InvalidTransaction(
                        "Organization ID required".into(),
                    ));
                }

                if create_smart_permission.name().is_empty() {
                    return Err(ApplyError::InvalidTransaction(
                        "Smart permission name required".into(),
                    ));
                }

                if create_smart_permission.function().is_empty() {
                    return Err(ApplyError::InvalidTransaction(
                        "Function body required".into(),
                    ));
                }
            }
            Action::UpdateSmartPermission(update_smart_permission) => {
                if update_smart_permission.org_id().is_empty() {
                    return Err(ApplyError::InvalidTransaction(
                        "Organization ID required".into(),
                    ));
                }

                if update_smart_permission.name().is_empty() {
                    return Err(ApplyError::InvalidTransaction(
                        "Smart permission name required".into(),
                    ));
                }

                if update_smart_permission.function().is_empty() {
                    return Err(ApplyError::InvalidTransaction(
                        "Function body required".into(),
                    ));
                }
            }
            Action::DeleteSmartPermission(delete_smart_permission) => {
                if delete_smart_permission.org_id().is_empty() {
                    return Err(ApplyError::InvalidTransaction(
                        "Organization ID required".into(),
                    ));
                }

                if delete_smart_permission.name().is_empty() {
                    return Err(ApplyError::InvalidTransaction(
                        "Smart permission name required".into(),
                    ));
                }
            }
            Action::CreateAccount(create_account) => {
                if create_account.org_id().is_empty() {
                    return Err(ApplyError::InvalidTransaction(
                        "Organization ID required".into(),
                    ));
                }

                if create_account.public_key().is_empty() {
                    return Err(ApplyError::InvalidTransaction(
                        "Account public_key required".into(),
                    ));
                }
            }
            Action::UpdateAccount(update_account) => {
                if update_account.org_id().is_empty() {
                    return Err(ApplyError::InvalidTransaction(
                        "Organization ID required".into(),
                    ));
                }

                if update_account.public_key().is_empty() {
                    return Err(ApplyError::InvalidTransaction(
                        "Account public_key required".into(),
                    ));
                }
            }
            Action::CreateOrganization(create_organization) => {
                if create_organization.id().is_empty() {
                    return Err(ApplyError::InvalidTransaction(
                        "Organization ID required".into(),
                    ));
                }

                if create_organization.name().is_empty() {
                    return Err(ApplyError::InvalidTransaction(
                        "Organization name required".into(),
                    ));
                }

                if create_organization.address().is_empty() {
                    return Err(ApplyError::InvalidTransaction(
                        "Organization address required".into(),
                    ));
                }
            }
            Action::UpdateOrganization(update_organization) => {
                if update_organization.id().is_empty() {
                    return Err(ApplyError::InvalidTransaction(
                        "Organization ID required".into(),
                    ));
                }

                if update_organization.name().is_empty() {
                    return Err(ApplyError::InvalidTransaction(
                        "Organization name required".into(),
                    ));
                }

                if update_organization.address().is_empty() {
                    return Err(ApplyError::InvalidTransaction(
                        "Organization address required".into(),
                    ));
                }
            }
        };

        Ok(Some(SabreRequestPayload {
            action: sabre_action.clone(),
        }))
    }

    pub fn get_action(&self) -> Action {
        self.action.clone()
    }
}
