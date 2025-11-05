
pub fn set_hook() {
    std::panic::set_hook(Box::new(move |info| {
        let message = format!("{info}");
        log::error!("{}", message.replace('\n', " "));
        log::logger().flush();
    }));
}
