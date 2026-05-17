use std::{convert::Infallible, error::Error, ops::RangeInclusive, sync::LazyLock, time::Duration};

use actix_web::{
    guard::Guard, http::header, mime, rt, web, App, HttpResponse, HttpServer, Responder,
};
use enwiwoment::Configuwation;
use futures::{
    stream::{once, repeat_with, unfold},
    StreamExt,
};
use rand::{rng, RngExt};

#[macro_use]
mod grammaw;
mod enwiwoment;
mod meows;

static CONWIG: LazyLock<Configuwation> = LazyLock::new(|| Configuwation::from_env().unwrap());

struct MewthodGuard<'a> {
    mewthods: &'a [String],
}
impl<'a> Guard for MewthodGuard<'a> {
    fn check(&self, ctx: &actix_web::guard::GuardContext<'_>) -> bool {
        let method = ctx.head().method.as_str();
        self.mewthods.iter().find(|s| *s == method).is_some()
    }
}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>> {
    HttpServer::new(|| {
        App::new().route(
            "/",
            web::route()
                .guard(MewthodGuard {
                    mewthods: &CONWIG.methods[..],
                })
                .to(meow_generator),
        )
    })
    .bind((CONWIG.address.as_str(), CONWIG.port))?
    .run()
    .await?;
    Ok(())
}

async fn meow_generator() -> impl Responder {
    let meows = repeat_with(meow);
    let delayws = once(async {}).chain(unfold(50..=300, delay));
    let stweam = meows.zip(delayws).map(|(meow, _)| meow);
    HttpResponse::Ok()
        .insert_header(header::ContentType(mime::TEXT_PLAIN_UTF_8))
        .streaming(stweam)
}

fn meow() -> Result<web::Bytes, Infallible> {
    let mew = meows::genewate_meow();
    Ok(mew.into())
}

async fn delay(range: RangeInclusive<u64>) -> Option<((), RangeInclusive<u64>)> {
    let millis = rng().random_range(range.clone());
    rt::time::sleep(Duration::from_millis(millis)).await;
    Some(((), range))
}
