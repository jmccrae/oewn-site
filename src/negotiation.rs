use rocket::request::{FromRequest, Request, Outcome};
use rocket::response::{Redirect, Responder};
use rocket::response::content::RawHtml;

pub enum ContentNegotiation { Html, RdfXml, Turtle, Json }

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ContentNegotiation {
    type Error = String;
    async fn from_request(request: &'r Request<'_>) -> Outcome<ContentNegotiation, String> {
        for value in request.headers().get("Accept") {
            if value.starts_with("text/html") {
                return Outcome::Success(ContentNegotiation::Html);
            } else if value.starts_with("application/rdf+xml") {
                return Outcome::Success(ContentNegotiation::RdfXml);
            } else if value.starts_with("text/turtle") {
                return Outcome::Success(ContentNegotiation::Turtle);
            } else if value.starts_with("application/x-turtle") {
                return Outcome::Success(ContentNegotiation::Turtle);
            } else if value.starts_with("application/json") {
                return Outcome::Success(ContentNegotiation::Json);
            } else if value.starts_with("application/javascript") {
                return Outcome::Success(ContentNegotiation::Json);
            }
        }
        Outcome::Success(ContentNegotiation::Html)
    }
}

#[derive(Responder)]
pub enum NegotiatedResponse {
    Redirect(Redirect),
    Html(RawHtml<&'static str>),
    HtmlDyn(RawHtml<String>)
}


pub fn negotiated(idx : &'static str, key : &str, index: &'static str, neg : ContentNegotiation) -> NegotiatedResponse {
    if key.ends_with(".rdf") {
        negotiated(idx,&key[0..(key.len()-4)], index, ContentNegotiation::RdfXml)
    } else if key.ends_with(".ttl") {
        negotiated(idx,&key[0..(key.len()-4)], index, ContentNegotiation::Turtle)
    } else if key.ends_with(".json") {
        negotiated(idx,&key[0..(key.len()-5)], index, ContentNegotiation::Json)
    } else if key.ends_with(".html") {
        negotiated(idx,&key[0..(key.len()-5)], index, ContentNegotiation::Html)
    } else {
        match neg {
            ContentNegotiation::Html => { 
                NegotiatedResponse::Html(RawHtml(index))
            },
            ContentNegotiation::RdfXml => {
                NegotiatedResponse::Redirect(Redirect::to(format!("/rdf/{}/{}", idx, key)))
            },
            ContentNegotiation::Turtle => {
                NegotiatedResponse::Redirect(Redirect::to(format!("/ttl/{}/{}", idx, key)))
            },
            ContentNegotiation::Json => {
                NegotiatedResponse::Redirect(Redirect::to(format!("/json/{}/{}", idx, key)))
            }
        }
    }
}

 
