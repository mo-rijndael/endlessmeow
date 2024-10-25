use std::{convert::Infallible, error::Error, sync::LazyLock, time::Duration};

use actix_web::{guard::Guard, rt, web, App, HttpResponse, HttpServer, Responder};
use enwiwoment::Configuwation;
use futures::{
    stream::{repeat_with, unfold},
    StreamExt,
};
use rand::{thread_rng, Rng};

#[macro_use]
mod grammaw;
mod enwiwoment;
mod meows;

static CONFIG: LazyLock<Configuwation> = LazyLock::new(|| Configuwation::from_env().unwrap());

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
                    mewthods: &CONFIG.methods[..],
                })
                .to(meow_generator),
        )
    })
    .bind((CONFIG.address.as_str(), CONFIG.port))?
    .run()
    .await?;
    Ok(())
}

async fn meow_generator() -> impl Responder {
    let meows = repeat_with(meow);
    let delays = unfold((), delay);
    let stream = meows.zip(delays).map(|(meow, _)| meow);
    HttpResponse::Ok().streaming(stream)
}

fn meow() -> Result<web::Bytes, Infallible> {
    let mew = meows::generate_meow();
    Ok(mew.into())
}
async fn delay(_: ()) -> Option<((), ())> {
    let millis = thread_rng().gen_range(50..=300);
    rt::time::sleep(Duration::from_millis(millis)).await;
    Some(((), ()))
}
