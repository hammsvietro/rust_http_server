pub struct HttpRequest<'a> {
  pub version: &'a str,
  pub query: &'a str
}