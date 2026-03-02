pub mod add_user_to_group;
pub mod create_group;
pub mod get_group;
pub mod list_groups;
pub mod remove_user_from_group;

pub struct GroupUseCaseDeps<'a, R> {
    pub repository: &'a R,
}
