extern crate etcd;
extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate native_tls;
extern crate tokio_core;

use futures::{Future, Stream};
use tokio_core::reactor::Core;

use test::TestClient;

mod test;

#[test]
fn health() {
    let core = Core::new().unwrap();
    let mut client = TestClient::no_destructor(core);

    let work = client.health().collect().and_then(|member_info| {
        for (health, _) in member_info {
            assert_eq!(health.health, "true");
        }

        Ok(())
    });

    assert!(client.run(work).is_ok());
}
#[test]
fn versions() {
    let core = Core::new().unwrap();
    let mut client = TestClient::no_destructor(core);

    let work = client.versions().collect().and_then(|member_info| {
        for (version_info, _) in member_info {
            assert_eq!(version_info.cluster_version, "2.3.0");
            assert_eq!(version_info.server_version, "2.3.7");
        }

        Ok(())
    });

    assert!(client.run(work).is_ok());
}
