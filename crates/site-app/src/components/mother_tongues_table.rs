use leptos::*;

use crate::functions::fetch::fetch_all_mother_tongues;

const DEFAULT_FETCH_LIMIT: usize = 25;

#[island]
pub fn MotherTonguesTable() -> impl IntoView {
  let (query_term, set_query_term) = create_signal(String::new());
  let (current_page, _set_current_page) = create_signal(0_u32);
  let tongues = create_resource(
    move || {
      with!(|query_term, current_page| {
        let term = if query_term.is_empty() {
          None
        } else {
          Some(query_term)
        };
        (
          term.cloned(),
          *current_page * DEFAULT_FETCH_LIMIT as u32,
          DEFAULT_FETCH_LIMIT as _,
        )
      })
    },
    move |(term, offset, count)| fetch_all_mother_tongues(term, offset, count),
  );

  let table_element = move || {
    tongues().map(|d| match d {
      Ok((data, count)) => {
        let page_count =
          (count as f32 / DEFAULT_FETCH_LIMIT as f32).ceil() as usize;
        view! {
          <InnerMotherTonguesTable>
            <tbody>
              <For
                each=move || data.clone() key={|t| t.id}
                children={ move |d| view! { <MotherTonguesTableRow d={d} /> } }
              />
            </tbody>
          </InnerMotherTonguesTable>
        }
        .into_view()
      }
      Err(_) => view! {
        <p>"Something went wrong. We apologize. Try reloading the page."</p>
      }
      .into_view(),
    })
  };

  view! {
    <div class="flex flex-col gap-4">
      <div class="flex flex-row gap-4 items-center">
        <input class="input" placeholder="Search..."
          on:input=move |ev| set_query_term(event_target_value(&ev))
          prop:value=query_term
        />
        <div class="flex-1" />
      </div>
      <Transition fallback=SuspenseMotherTonguesTable>
        { table_element }
      </Transition>
    </div>
  }
}

#[component]
fn Tooltip(tooltip: String, children: Children) -> impl IntoView {
  view! {
    <span class="tooltip tooltip-top max-w-32" data-tooltip={ tooltip }>
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
        <tr>
          <th class="w-64">"Name"</th>
          <th>"Description"</th>
        </tr>
      </thead>
      { children() }
    </table>
  }
}

#[component]
fn SuspenseMotherTonguesTable() -> impl IntoView {
  view! {
    <InnerMotherTonguesTable>
      <tbody>
        { (0..DEFAULT_FETCH_LIMIT).map(|_| view! {
          <tr>
            <td><div class="skeleton-pulse h-5 rounded-md"></div></td>
            <td><div class="skeleton-pulse h-5 rounded-md"></div></td>
          </tr>
        }).collect_view() }
      </tbody>
    </InnerMotherTonguesTable>
  }
}
