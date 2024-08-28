use std::collections::HashMap;

use axum::extract::{Query, Request};
use axum::{response::IntoResponse, routing::get, Router};
use fake::faker::company::en::*;
use fake::faker::internet::en::*;
use fake::faker::name::en::*;
use fake::faker::phone_number::en::*;
use fake::{Dummy, Fake, Faker};
use maud::html;
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

#[derive(Debug, Dummy, Clone)]
struct Contact {
  #[dummy(faker = "1..1000")]
  id: u64,

  #[dummy(faker = "Name()")]
  name: String,

  #[dummy(faker = "CompanyName()")]
  company: String,

  #[dummy(faker = "FreeEmail()")]
  email: String,

  #[dummy(faker = "PhoneNumber()")]
  phone: String,
}

#[derive(Debug)]
struct TableFilter {
  sort: String,
  order: String,
  page: u32,
  per_page: u32,
}

fn base(html: maud::Markup) -> impl IntoResponse {
  html! {
    html {
      head {
        meta charset="utf-8" {}
        title { "Contacts" }

        link rel="stylesheet" href="https://unpkg.com/@picocss/pico@2.0" {}
        script src="https://unpkg.com/htmx.org@2.0" {}
      }
      body class="container" {
        { (html) }
      }
    }
  }
}

fn get_contacts(n: usize, qs: &TableFilter) -> (Vec<Contact>, u32) {
  let mut items: Vec<Contact> = Vec::with_capacity(n);
  let mut rng = ChaCha8Rng::seed_from_u64(42);
  for _ in 0..n {
    items.push(Faker.fake_with_rng(&mut rng));
  }

  items.sort_by(|a, b| {
    let cmp = match qs.sort.as_str() {
      "id" => a.id.cmp(&b.id),
      "name" => a.name.cmp(&b.name),
      "company" => a.company.cmp(&b.company),
      "email" => a.email.cmp(&b.email),
      "phone" => a.phone.cmp(&b.phone),
      _ => a.id.cmp(&b.id),
    };

    if qs.order == "asc" {
      cmp
    } else {
      cmp.reverse()
    }
  });

  let pages = (items.len() as f64 / qs.per_page as f64).ceil() as u32;
  let range_s = (qs.page as usize - 1) * qs.per_page as usize;
  let range_e = qs.page as usize * qs.per_page as usize;
  let items = items[range_s..range_e].to_vec();

  (items, pages)
}

fn parse_table_filter(req: &Request) -> TableFilter {
  let qs: Query<HashMap<String, String>> = Query::try_from_uri(req.uri()).unwrap();
  let sort = qs.get("sort").unwrap_or(&"name".to_string()).to_string();
  let order = qs.get("order").unwrap_or(&"desc".to_string()).to_string();
  let page = qs.get("page").unwrap_or(&"1".to_string()).parse().unwrap();
  let per_page = qs.get("per_page").unwrap_or(&"10".to_string()).parse().unwrap();
  TableFilter { sort, order, page, per_page }
}

fn th(title: &str, qs: &TableFilter) -> maud::Markup {
  let id = title.to_lowercase();
  let order = if qs.sort == id && qs.order == "desc" { "asc" } else { "desc" };
  let url = format!("/?sort={}&order={}&page=1&per_page={}", id, order, qs.per_page);

  html! {
    th scope="col" style="cursor: pointer;"
      hx-trigger="click"
      hx-get=(url)
      hx-target="#contacts_table"
      hx-swap="outerHTML"
    {
      (title)
      @if qs.sort == id {
        span style="margin-left: 0.5em;" {
          @if qs.order == "asc" { "↑" } @else { "↓" }
        }
      }
    }
  }
}

fn pagination_link(qs: &TableFilter, i: u32) -> maud::Markup {
  let url = format!("/?sort={}&order={}&page={}&per_page={}", qs.sort, qs.order, i, qs.per_page);
  html! {
    li {
      a href=(url)
        hx-trigger="click"
        hx-get=(url)
        hx-target="#contacts_table"
        hx-swap="outerHTML"
      { (i) }
    }
  }
}

fn pagination_delim() -> maud::Markup {
  html! {
    li {
      span { "..." }
    }
  }
}

// https://stackoverflow.com/a/70263913
fn calc_pagination(page: usize, total: usize, len: usize) -> Vec<Option<usize>> {
  use std::cmp::{max, min};

  let len = if len == 0 { 5 } else { len };
  let total = max(total, page);
  let start =
    max(1, min(page as isize - ((len - 3) as isize / 2), total as isize - len as isize + 2))
      as usize;
  let end = min(total, max(page + (len - 2) / 2, len - 1));

  let mut result = Vec::new();

  if start > 2 {
    result.push(Some(1));
    result.push(None);
  } else if start > 1 {
    result.push(Some(1));
  }

  for i in start..=end {
    result.push(Some(i));
  }

  if end < total - 1 {
    result.push(None);
    result.push(Some(total));
  } else if end < total {
    result.push(Some(total));
  }

  result
}

fn pagination(pages: u32, qs: &TableFilter) -> maud::Markup {
  let items = calc_pagination(qs.page as usize, pages as usize, 5);

  html!(
    nav {
      ul {
        @for item in items {
          @if let Some(i) = item {
            (pagination_link(qs, i as u32))
          } @else {
            (pagination_delim())
          }
        }
      }
    }
  )
}

fn get_contacts_table(contacts: Vec<Contact>, qs: &TableFilter, pages: u32) -> maud::Markup {
  html! {
    div id="contacts_table" {
      table  {
        thead {
          tr {
            (th("ID", &qs))
            (th("Name", &qs))
            (th("Company", &qs))
            (th("Email", &qs))
            (th("Phone", &qs))
          }
        }
        tbody {
          @for contact in contacts {
            tr {
              td { (contact.id) }
              td { (contact.name) }
              td { (contact.company) }
              td { (contact.email) }
              td { (contact.phone) }
            }
          }
        }
      }

    (pagination(pages, &qs))
    }
  }
}

fn get_hx_target(req: &Request) -> Option<&str> {
  match req.headers().get("hx-target") {
    Some(x) => Some(x.to_str().unwrap_or_default()),
    None => None,
  }
}

async fn index_page(req: Request) -> impl IntoResponse {
  let qs = parse_table_filter(&req);
  let (contacts, pages) = get_contacts(1000, &qs);

  match get_hx_target(&req) {
    Some("contacts_table") => return get_contacts_table(contacts, &qs, pages).into_response(),
    _ => {}
  }

  base(html! {
    nav {
      ul {
        li { a href="/dashboard" { "Dashboard" } }
        li { a href="/" { "Contacts" } }
        li { a href="/settings" { "Settings" } }
      }
    }

    h1 { "Contacts" }
    (get_contacts_table(contacts, &qs, pages))
  })
  .into_response()
}

#[tokio::main]
async fn main() {
  let service = Router::new().route("/", get(index_page)).into_make_service();
  let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await.unwrap();
  axum::serve(listener, service).await.unwrap();
}
