use cfg_if::cfg_if;
use http::status::StatusCode;
use leptos::*;
#[cfg(feature = "ssr")]
use leptos_axum::ResponseOptions;
use thiserror::Error;

#[derive(Clone, Debug, Error)]
pub enum AppError {
  #[error("Not Found")]
  NotFound,
}

impl AppError {
  pub fn status_code(&self) -> StatusCode {
    match self {
      AppError::NotFound => StatusCode::NOT_FOUND,
    }
  }
}

#[component]
pub fn ErrorTemplate(
  #[prop(optional)] outside_errors: Option<Errors>,
  #[prop(optional)] errors: Option<RwSignal<Errors>>,
) -> impl IntoView {
  let errors = match outside_errors {
    Some(e) => create_rw_signal(e),
    None => match errors {
      Some(e) => e,
      None => panic!("No Errors found and we expected errors!"),
    },
  };
  // Get Errors from Signal
  let errors = errors.get_untracked();

  // Downcast lets us take a type that implements `std::error::Error`
  let errors: Vec<AppError> = errors
    .into_iter()
    .filter_map(|(_k, v)| v.downcast_ref::<AppError>().cloned())
    .collect();
  println!("Errors: {errors:#?}");

  // Only the response code for the first error is actually sent from the
  // server this may be customized by the specific application
  cfg_if! { if #[cfg(feature="ssr")] {
      let response = use_context::<ResponseOptions>();
      if let Some(response) = response {
          response.set_status(errors[0].status_code());
      }
  }}

  view! {
    <div class="flex flex-col gap-4">
      <p class="text-4xl tracking-tight font-semibold">
        {if errors.len() > 1 { "Server Errors" } else { "Server Error" }}
      </p>
      <For
        each=move || { errors.clone().into_iter().enumerate() }
        key=|(index, _error)| *index
        children=move |error| {
          let error_string = error.1.to_string();
          let error_code = error.1.status_code();
          view! {
            <div>
              <p class="text-2xl tracking-tight font-semibold">{error_code.to_string()}</p>
              <p>"Error: " {error_string}</p>
            </div>
          }
        }
      />
    </div>
  }
}
