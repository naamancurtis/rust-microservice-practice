extern crate actix_web;
use actix_service::{Service, ServiceExt, Transform};
use actix_session::{CookieSession, Session};
use actix_web::http::header::ContentEncoding::Identity;
use actix_web::middleware::Logger;
use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error, HttpResponse};
use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use futures::future::{ok, FutureResult};
use futures::{Future, Poll};
use std::cell::RefCell;
use std::ops::Add;

#[derive(Default)]
struct State(RefCell<i64>);

pub struct Counter(RefCell<i64>);

impl Counter {
    pub fn new() -> Self {
        Counter(RefCell::new(0 as i64))
    }
}

impl<S, B> Transform<S> for Counter
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = CountMiddleware<S>;
    type InitError = ();
    type Future = FutureResult<Self::Transform, Self::InitError>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(CountMiddleware {
            service,
            count: Counter(self.0.clone()),
        })
    }
}

pub struct CountMiddleware<S> {
    service: S,
    count: Counter,
}

impl<S, B> Service for CountMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Box<dyn Future<Item = Self::Response, Error = Self::Error>>;

    fn poll_ready(&mut self) -> Poll<(), Self::Error> {
        self.service.poll_ready()
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        *self.count.0.get_mut() = *self.count.0.get_mut() + 1 as i64;
        println!("The count is now: {}", self.count.0.clone().into_inner());
        Box::new(self.service.call(req).and_then(|res| Ok(res)))
    }
}

fn index() -> impl Responder {
    HttpResponse::Ok().body("Welcome to the microservice")
}

fn main() {
    env_logger::init();
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(CookieSession::signed(&[0; 32]).secure(false))
            .wrap(Counter::new())
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8080")
    .unwrap()
    .run()
    .unwrap();
}
