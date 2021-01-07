pub(crate) struct MIDIClient {
    inner: coremidi::Client,
}

impl MIDIClient {
    pub fn new() -> Self {
        Self {
            inner: coremidi::Client::new("fdsa").unwrap(),
        }
    }

    pub fn new_with_notification<F>(name: &str, callback: F) -> Self
    where
        F: FnMut(&coremidi::Notification) + Send + 'static,
    {
        Self {
            inner: coremidi::Client::new_with_notifications(name, callback).unwrap(),
        }
    }
}
