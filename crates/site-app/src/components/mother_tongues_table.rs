use leptos::*;

use crate::functions::fetch::fetch_all_mother_tongues;

const DEFAULT_FETCH_LIMIT: usize = 5;

#[island]
pub fn MotherTonguesTable() -> impl IntoView {
  let (query_term, set_query_term) = create_signal(String::new());
  let (offset, _set_offset) = create_signal(0_u32);
  let tongues = create_resource(
    move || {
      with!(|query_term, offset| {
        let term = if query_term.is_empty() {
          None
        } else {
          Some(query_term)
        };
        (term.cloned(), *offset, DEFAULT_FETCH_LIMIT as _)
      })
    },
    move |(term, offset, count)| fetch_all_mother_tongues(term, offset, count),
  );

  view! {
    <div class="flex flex-col gap-4">
      <div class="flex flex-row gap-4 items-center">
        <input class="input" placeholder="Search..."
          on:input=move |ev| set_query_term(event_target_value(&ev))
          prop:value=query_term
        />
        <div class="flex-1" />
      </div>
      <InnerMotherTonguesTable>
        <Transition fallback=SuspenseMotherTonguesTable>
          {move || tongues().map(|data| match data {
            Ok((data, _count)) => view! {
              <tbody>
                <For
                  each=move || data.clone() key={|t| t.id}
                  children={ move |d| view! { <MotherTonguesTableRow d={d} /> } }
                />
              </tbody>
            },
            Err(_) => todo!(),
          })}
        </Transition>
      </InnerMotherTonguesTable>
    </div>
  }
}

#[component]
fn Tooltip(tooltip: String, children: Children) -> impl IntoView {
  view! {
    <span class="tooltip tooltip-bottom max-w-32" data-tooltip={ tooltip }>
      { children() }
    </span>
  }
}

#[component]
fn MotherTonguesTableRow(d: core_types::MotherTongue) -> impl IntoView {
  view! {
    <tr>
      <th class="truncate">
        <Tooltip tooltip={ d.name.clone() }>
          { d.name }
        </Tooltip>
      </th>
      <td class="truncate">
        <Tooltip tooltip={ d.description.clone() }>
          { d.description }
        </Tooltip>
      </td>
    </tr>
  }
}

#[component]
fn InnerMotherTonguesTable(children: Children) -> impl IntoView {
  view! {
    <table class="table max-w-full">
      <thead>
        <th>"Name"</th>
        <th>"Description"</th>
      </thead>
      { children() }
    </table>
  }
}

#[component]
fn SuspenseMotherTonguesTable() -> impl IntoView {
  view! {
    <tbody>
      <tr>
        <td><div class="skeleton-pulse h-5 rounded-md"></div></td>
        <td><div class="skeleton-pulse h-5 rounded-md"></div></td>
      </tr>
      <tr>
        <td><div class="skeleton-pulse h-5 rounded-md"></div></td>
        <td><div class="skeleton-pulse h-5 rounded-md"></div></td>
      </tr>
      <tr>
        <td><div class="skeleton-pulse h-5 rounded-md"></div></td>
        <td><div class="skeleton-pulse h-5 rounded-md"></div></td>
      </tr>
    </tbody>
  }
}
