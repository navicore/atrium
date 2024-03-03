// This file is generated by atrium-codegen. DO NOT EDIT.
//!Definitions for the `app.bsky.actor` namespace.
pub mod defs;
pub mod get_preferences;
pub mod get_profile;
pub mod get_profiles;
pub mod get_suggestions;
pub mod profile;
pub mod put_preferences;
pub mod search_actors;
pub mod search_actors_typeahead;
#[derive(Debug)]
pub struct Profile;
impl crate::types::Collection for Profile {
    const NSID: &'static str = "app.bsky.actor.profile";
    type Record = profile::Record;
}
