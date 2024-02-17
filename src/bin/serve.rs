#![warn(unused_variables)]
#![warn(unused_imports)]

use ntex::channel::condition::Condition;
use ntex::codec::{BytesCodec, Encoder};
use ntex_helmet::Helmet;
use ntex::web::dev::AppConfig;
use ntex::web::WebServiceFactory;
use ntex::{
    http::h1::control::PeerGone,
    server::ServiceConfig,
    service::{fn_factory_with_config, fn_service},
    web::{
        self, guard,
        middleware::{Compress, DefaultHeaders, Logger},
        scope, to,
        App,  HttpResponse, HttpServer,
    },
};
use ntex_cors::Cors;
use ntex_identity::{
    CookieIdentityPolicy, IdentityService,
};
use ntex_session::{self, CookieSession};
use of_net::{
    state::AppState,
    routes::{about, index, users},
};
use tracing::{Instrument, Span};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

#[ntex::main]
pub async fn main() -> anyhow::Result<(), anyhow::Error> {
    std::env::set_var("RUSTFLAGS", "1");
    // let mut b = SslAcceptor::mozilla_intermediate(SslMethod::tls())?;
    // b.set_private_key_file("key.pem", SslFiletype::PEM)?;
    // b.set_certificate_chain_file("cert.pem")?;
    tracing_subscriber::fmt().finish();
    HttpServer::new(|| {
        let ip = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127,0,0,1)), 8088);
        let ac = AppConfig::new(false, ip, "127.0.0.1".into());
        let _jsoncfg = web::types::JsonConfig::default().limit(4096);
        App::new()
            .instrument(Span::current())
                .into_inner()
            .state(ac.clone())
            .case_insensitive_routing()
            .default_service(to(index))
            .external_resource("/clp","https://clp.is")
            .external_resource("/store", "https://ode.si")
            .wrap(Cors::default())
            .wrap(CookieSession::signed(&[0;32])
                .name("clp-sess")
                .path("/")
                .secure(true)
            )
            // .wrap(Helmet::default())
            //     .add(ContentSecurityPolicy::default())
            //     .add(CrossOriginOpenerPolicy::same_origin_allow_popups())
            //     .add(CrossOriginEmbedderPolicy::credentialless())
            //     .add(XContentTypeOptions::nosniff())
            // ))
            // .add(ControlMessage::PeerGone(PeerGone::into(self)))
            .wrap(DefaultHeaders::new().header("X-Version", "0.2"))
            .wrap(IdentityService::<CookieIdentityPolicy>::new(
                CookieIdentityPolicy::new(&[0;32])
                .secure(true)
                .path("/")
                .name("clp-id")
            ))
            .wrap(Compress::default())
            .wrap(Logger::default())
            .service(scope("/")
                    .guard(guard::Host("www.reddit.com"))
                    .route("", to(index))
                    .route("/about", to(about)), // .register(jsoncfg),
            )
            .service(scope("/group")
                    .guard(guard::Host("www.rust-lang.org"))
                    .route("", to(|| async { HttpResponse::Ok().body("group") }))
                    .route("/explore", to(|| async { HttpResponse::Ok().body("Ex") })
            )
            .service(scope("/user")
                    .guard(guard::Host("www.rust-lang.org"))
                    .route("", to(users))
                    .route("/explore", to(|| async { HttpResponse::Ok().body("Ex") })),
            )
            .service(scope("/admin")
                    .guard(guard::Host("www.rust-lang.org"))
                    .route("", to(index)),
            )
        )
        }
    )
    .workers(8)
    .bind(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127,0,0,1)), 8888))?
    // .bind_openssl("127.0.0.1:8030")
    .run()
    .await?;
    Ok(())
}
