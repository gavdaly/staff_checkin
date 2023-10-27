pub fn archive_day() {
    close_open_users();
}

/// Finds users that haven't checked out in the database and sends then a text message.
fn close_open_users() {
    let users_open = Vec::new(); //session.update(end_time: session.start_time, state: :editable)
    users_open.forEach(|user| {
        let text = "You were still signed in at the end of the day, please check your hours";
        // send SMS('+1' + session.user.phone_number, text)
    })
}
