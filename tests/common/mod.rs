use std::sync::Once;

static INIT: Once = Once::new();

pub async fn set_up<T: FnOnce()>(f: T) {
    INIT.call_once(|| {
        f();
    });
}
