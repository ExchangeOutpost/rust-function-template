use extism_pdk::{WithReturnCode, host_fn};

#[host_fn]
extern "ExtismHost" {
    fn add_notification(notification_type: String, notification_target: String, body: String);
}

#[allow(dead_code)]
pub fn schedule_webhook(path: &str, body: &str) -> Result<(), WithReturnCode<extism_pdk::Error>> {
    unsafe {
        let res = add_notification("webhook".into(), path.into(), body.into());
        if res.is_err() {
            return Err(WithReturnCode::new(
                extism_pdk::Error::new(std::io::Error::new(std::io::ErrorKind::Other, "impossible to send notification")),
                6,
            ));
        }
        Ok(())
    }
}

#[allow(dead_code)]
pub fn schedule_email(email: &str, body: &str) -> Result<(), WithReturnCode<extism_pdk::Error>> {
    unsafe {
        let res = add_notification("email".into(), email.into(), body.into());
        if res.is_err() {
            return Err(WithReturnCode::new(
                extism_pdk::Error::new(std::io::Error::new(std::io::ErrorKind::Other, "impossible to send notification")),
                7,
            ));
        }
        Ok(())
    }
}
