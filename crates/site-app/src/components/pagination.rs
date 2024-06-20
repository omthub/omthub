use leptos::*;

#[component]
pub fn Pagination(
  #[prop(optional, into)] class: MaybeProp<String>,
  total_pages: MaybeSignal<u32>,
  current_page: MaybeSignal<u32>,
  #[prop(optional)] set_page: Option<WriteSignal<u32>>,
  #[prop(optional)] max_pages_to_display: MaybeProp<u32>,
) -> impl IntoView {
  let class = move || format!("pagination {}", class.get().unwrap_or_default());

  let range = create_memo(move |_| {
    select_page_range(
      current_page.get(),
      max_pages_to_display.get().unwrap_or(5),
      total_pages.get(),
    )
  });

  let left_one_page = move || {
    set_page.map(|w| w.update(|i| *i -= 1));
  };
  let right_one_page = move || {
    set_page.map(|w| w.update(|i| *i += 1));
  };

  let left_button = move || {
    view! {
      <button
        class="btn" disabled={move || current_page() == 0}
        on:click={move |_| left_one_page()}
      >
        <LeftArrowSvg />
      </button>
    }
  };
  let right_button = move || {
    view! {
      <button
        class="btn" disabled={move || current_page() == total_pages() - 1}
        on:click={move |_| right_one_page()}
      >
        <RightArrowSvg />
      </button>
    }
  };

  let page_buttons = move || {
    range
      .get()
      .into_iter()
      .map(|i| {
        view! {
          <button
            class="btn" class=("btn-active", {i == current_page.get()})
            on:click={move |_| { set_page.map(|w| w(i)); }}
          >
            { i + 1 }
          </button>
        }
      })
      .collect_view()
  };

  view! {
    <div class=class>
      { left_button }
      { page_buttons }
      { right_button }
    </div>
  }
}

#[component]
fn LeftArrowSvg() -> impl IntoView {
  view! {
    <svg width="18" height="18" viewBox="0 0 20 20" fill="none" xmlns="http://www.w3.org/2000/svg">
      <path fill-rule="evenodd" clip-rule="evenodd" d="M12.2574 5.59165C11.9324 5.26665 11.4074 5.26665 11.0824 5.59165L7.25742 9.41665C6.93242 9.74165 6.93242 10.2667 7.25742 10.5917L11.0824 14.4167C11.4074 14.7417 11.9324 14.7417 12.2574 14.4167C12.5824 14.0917 12.5824 13.5667 12.2574 13.2417L9.02409 9.99998L12.2574 6.76665C12.5824 6.44165 12.5741 5.90832 12.2574 5.59165Z" fill="#969696" />
    </svg>
  }
}

#[component]
fn RightArrowSvg() -> impl IntoView {
  view! {
    <svg width="18" height="18" viewBox="0 0 20 20" fill="none" xmlns="http://www.w3.org/2000/svg">
      <path fill-rule="evenodd" clip-rule="evenodd" d="M7.74375 5.2448C7.41875 5.5698 7.41875 6.0948 7.74375 6.4198L10.9771 9.65314L7.74375 12.8865C7.41875 13.2115 7.41875 13.7365 7.74375 14.0615C8.06875 14.3865 8.59375 14.3865 8.91875 14.0615L12.7437 10.2365C13.0687 9.91147 13.0687 9.38647 12.7437 9.06147L8.91875 5.23647C8.60208 4.9198 8.06875 4.9198 7.74375 5.2448Z" fill="#969696" />
    </svg>
  }
}

fn select_page_range(
  current: u32,
  count_to_select: u32,
  total_pages: u32,
) -> Vec<u32> {
  // if we're only selecting one page
  if count_to_select == 1 {
    return vec![current];
  }

  // if we're selecting at least all of the pages
  if count_to_select >= total_pages {
    return (0..total_pages).collect();
  }

  // how many pages on the left and right of the current to select
  // equal if count_to_select is odd, with right_margin greater if it's even
  let left_margin = (count_to_select - 1) / 2;
  let right_margin = (count_to_select - 1) / 2 + (count_to_select - 1) % 2;

  // if we don't have enough margin on the left
  if current < left_margin {
    return (0..count_to_select).collect();
  }

  // if we don't have enough margin on the right
  if current > total_pages - right_margin - 1 {
    return (total_pages - count_to_select..total_pages).collect();
  }

  // general case
  (current - left_margin..current + right_margin + 1).collect()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn select_page_range_works() {
    assert_eq!(select_page_range(2, 1, 6), vec![2]);
    assert_eq!(select_page_range(2, 6, 6), vec![0, 1, 2, 3, 4, 5]);
    assert_eq!(select_page_range(2, 7, 6), vec![0, 1, 2, 3, 4, 5]);
    assert_eq!(select_page_range(0, 5, 10), vec![0, 1, 2, 3, 4]);
    assert_eq!(select_page_range(2, 4, 10), vec![1, 2, 3, 4]);
    assert_eq!(select_page_range(8, 5, 10), vec![5, 6, 7, 8, 9]);
    assert_eq!(select_page_range(6, 4, 10), vec![5, 6, 7, 8]);
  }
}
