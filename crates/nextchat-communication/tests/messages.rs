use nextchat_communication::CommunicationMessage;

#[test]
fn test_message_parse_from_string() {
    let text_message = String::from("/event_name argument1 argument2 argument3");

    let message = CommunicationMessage::from_string(text_message.clone());
    assert!(message.is_ok());

    let message = message.unwrap();
    assert_eq!(message.to_string(), text_message);
    assert_eq!(message.get_name(), String::from("event_name"));
    assert_eq!(
        message.get_arguments(),
        [
            String::from("argument1"),
            String::from("argument2"),
            String::from("argument3")
        ]
        .to_vec()
    );
}

#[test]
fn test_message_parse_from_string_error() {
    if let Err(error) = CommunicationMessage::from_string(String::from("-event_name argument1")) {
        assert_eq!(
            error.as_str(),
            "The message format is incorrect. `/{name} {argument1} {argument2}`",
        );
        return;
    }

    assert!(false);
}
