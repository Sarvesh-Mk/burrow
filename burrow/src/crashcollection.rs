pub fn sentry_initialization() {
    let _guard = sentry::init(("https://10ed3803388cca16f3f7ff233d31a974@o4505808334422016.ingest.sentry.io/4505808347332608", sentry::ClientOptions {
        release: sentry::release_name!(),
        ..Default::default()
    }));
}
