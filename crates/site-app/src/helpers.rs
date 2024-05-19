use leptos::*;

pub fn get_auth_context() -> core_types::LoggedInUser {
  // flatten Option<LoggedInUser(Option<PublicUser>)> to LoggedInUser
  core_types::LoggedInUser(
    use_context::<core_types::LoggedInUser>().and_then(|s| s.0),
  )
}

pub fn validate_name(name: String) -> Option<String> {
  if name.len() < 3 {
    return Some("Name must be at least 3 characters long.".to_string());
  }

  None
}

pub fn validate_email(email: String) -> Option<String> {
  if email.is_empty() {
    return Some("An email is required.".to_string());
  }

  // Split the email into local and domain parts
  let parts: Vec<&str> = email.split('@').collect();
  if parts.len() != 2 {
    return Some(
      "Your email must contain exactly one \"@\" symbol.".to_string(),
    );
  }

  let local = parts[0];
  let domain = parts[1];

  // Check local part
  if local.is_empty() {
    return Some("The first part of the email is empty.".to_string());
  }

  if local.len() > 64 {
    return Some(
      "The first part of the email exceeds 64 characters.".to_string(),
    );
  }

  // Check for valid characters in local part
  for c in local.chars() {
    if !(c.is_alphanumeric() || c == '.' || c == '_' || c == '-' || c == '+') {
      return Some(format!("Invalid character '{}'.", c));
    }
  }

  // Check domain part
  if domain.is_empty() {
    return Some("The second part of the email is empty.".to_string());
  }

  if domain.len() > 255 {
    return Some(
      "The second part of the email exceeds 255 characters.".to_string(),
    );
  }

  // Split domain into labels
  let domain_labels: Vec<&str> = domain.split('.').collect();
  if domain_labels.len() < 2 {
    return Some(
      "The second part of the email must contain at least one '.' character."
        .to_string(),
    );
  }

  // Check each label in the domain
  for label in domain_labels {
    if label.is_empty() {
      return Some("One of the domain labels is empty.".to_string());
    }

    if label.len() > 63 {
      return Some(
        "One of the domain labels exceeds 63 characters.".to_string(),
      );
    }

    for c in label.chars() {
      if !(c.is_alphanumeric() || c == '-') {
        return Some(format!("Invalid character '{}'.", c));
      }
    }

    // Labels must not start or end with a hyphen
    if label.starts_with('-') {
      return Some(
        "One of the domain labels starts with a hyphen.".to_string(),
      );
    }

    if label.ends_with('-') {
      return Some("One of the domain labels ends with a hyphen.".to_string());
    }
  }

  None
}

pub fn validate_password(password: String) -> Option<String> {
  let min_length = 8;
  if password.len() < min_length {
    return Some(format!(
      "Password must be at least {} characters long.",
      min_length
    ));
  }
  if !password.chars().any(|c| c.is_uppercase()) {
    return Some(
      "Password must contain at least one uppercase letter.".to_string(),
    );
  }
  if !password.chars().any(|c| c.is_lowercase()) {
    return Some(
      "Password must contain at least one lowercase letter.".to_string(),
    );
  }
  if !password.chars().any(|c| c.is_digit(10)) {
    return Some("Password must contain at least one digit.".to_string());
  }
  if !password.chars().any(|c| !c.is_alphanumeric()) {
    return Some(
      "Password must contain at least one special character.".to_string(),
    );
  }

  None
}
