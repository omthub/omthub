use leptos::*;

pub fn get_auth_context() -> core_types::LoggedInUser {
  // flatten Option<LoggedInUser(Option<PublicUser>)> to LoggedInUser
  core_types::LoggedInUser(
    use_context::<core_types::LoggedInUser>().and_then(|s| s.0),
  )
}
