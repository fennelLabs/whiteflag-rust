use super::auth::{acquire_auth_lock, check_auth_lock, release_auth_lock};

#[test]
fn test_auth_lock_system() {
    acquire_auth_lock();
    assert!(check_auth_lock());
    assert!(release_auth_lock());
}
