use std::{fmt::Write, io::Cursor};

use rocket::{
    http::ContentType,
    request::Request,
    response::{self, Responder, Response},
};

/// A structure adding HTML Content-Type to the response.
pub struct Html<T>(T)
where
    T: AsRef<[u8]> + Unpin + Send + 'static;

impl<T> From<T> for Html<T>
where
    T: AsRef<[u8]> + Unpin + Send + 'static,
{
    fn from(x: T) -> Self {
        Self(x)
    }
}

#[rocket::async_trait]
impl<'r, T> Responder<'r, 'static> for Html<T>
where
    T: AsRef<[u8]> + Unpin + Send + 'static,
{
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        Response::build()
            .header(ContentType::HTML)
            .sized_body(None, Cursor::new(self.0))
            .ok()
    }
}

/// Write an `li` HTML tag for the given entity to the given string.
pub fn entity_to_li(
    entity_kind: &str,
    id: u32,
    name: &str,
    response: &mut String,
) -> Result<(), std::fmt::Error> {
    write!(
        response,
        r#"<li>
        <a href="https://orna.guide/admin/{entity_kind}s/{entity_kind}/{id}/change"><pre>#{id:04}</pre></a>: {name}
        </li>"#
    )
}

/// Write a list of the given entities to the given string, using the given formatter.
pub fn make_list<Iter, Formatter, T>(
    iter: Iter,
    title: &str,
    formatter: Formatter,
    response: &mut String,
) -> Result<(), std::fmt::Error>
where
    Iter: Iterator<Item = T>,
    Formatter: Fn(T, &mut String) -> Result<(), std::fmt::Error>,
{
    let mut iter = iter.peekable();

    if iter.peek().is_some() {
        write!(response, "<h2>{title}</h2>\n<p><ul>")?;

        for entity in iter {
            formatter(entity, response)?;
        }

        writeln!(response, "</ul></p>")?;
    }

    Ok(())
}

/// A basic style for the HTML pages.
pub const STYLE: &str = r#"<style>
    pre { display: inline; margin: 0; }
    html { color: #FFFFFF; background-color: #313339; }
    a { color: #11A6E1; }
</style>"#;
