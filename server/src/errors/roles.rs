use thiserror::Error;

#[derive(Error, Debug)]
pub enum RoleError {
    #[error("Role not found")]
    RoleNotFound,
    #[error("Role has exists")]
    RoleHasExists,
    #[error("Current Role has user Please delete the user role relation")]
    CurrentRoleHasUser,
}