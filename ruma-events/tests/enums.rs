use std::convert::TryFrom;

use matches::assert_matches;
use ruma_identifiers::{EventId, RoomAliasId, RoomId, UserId};
use serde_json::{from_value as from_json_value, json, Value as JsonValue};

use ruma_events::{
    room::{
        aliases::AliasesEventContent,
        message::{MessageEventContent, TextMessageEventContent},
        power_levels::PowerLevelsEventContent,
    },
    AnyEvent, AnyMessageEvent, AnyMessageEventStub, AnyRoomEvent, AnyRoomEventStub, AnyStateEvent,
    AnyStateEventContent, AnyStateEventStub, MessageEvent, MessageEventStub, StateEvent,
    StateEventStub,
};

fn message_event() -> JsonValue {
    json!({
        "content": {
            "body": "baba",
            "format": "org.matrix.custom.html",
            "formatted_body": "<strong>baba</strong>",
            "msgtype": "m.text"
        },
        "event_id": "$152037280074GZeOm:localhost",
        "origin_server_ts": 1,
        "sender": "@example:localhost",
        "room_id": "!room:room.com",
        "type": "m.room.message",
        "unsigned": {
            "age": 1
        }
    })
}

fn message_event_stub() -> JsonValue {
    json!({
        "content": {
            "body": "baba",
            "format": "org.matrix.custom.html",
            "formatted_body": "<strong>baba</strong>",
            "msgtype": "m.text"
        },
        "event_id": "$152037280074GZeOm:localhost",
        "origin_server_ts": 1,
        "sender": "@example:localhost",
        "type": "m.room.message",
        "unsigned": {
            "age": 1
        }
    })
}

fn aliases_event() -> JsonValue {
    json!({
        "content": {
            "aliases": ["#somewhere:localhost"]
        },
        "event_id": "$152037280074GZeOm:localhost",
        "origin_server_ts": 1,
        "sender": "@example:localhost",
        "state_key": "",
        "room_id": "!room:room.com",
        "type": "m.room.aliases",
        "unsigned": {
            "age": 1
        }
    })
}

fn aliases_event_stub() -> JsonValue {
    json!({
        "content": {
            "aliases": ["#somewhere:localhost"]
        },
        "event_id": "$152037280074GZeOm:localhost",
        "origin_server_ts": 1,
        "sender": "@example:localhost",
        "state_key": "",
        "type": "m.room.aliases",
        "unsigned": {
            "age": 1
        }
    })
}

#[test]
fn power_event_stub_deserialization() {
    let json_data = json!({
        "content": {
            "ban": 50,
            "events": {
                "m.room.avatar": 50,
                "m.room.canonical_alias": 50,
                "m.room.history_visibility": 100,
                "m.room.name": 50,
                "m.room.power_levels": 100
            },
            "events_default": 0,
            "invite": 0,
            "kick": 50,
            "redact": 50,
            "state_default": 50,
            "users": {
                "@example:localhost": 100
            },
            "users_default": 0
        },
        "event_id": "$15139375512JaHAW:localhost",
        "origin_server_ts": 45,
        "sender": "@example:localhost",
        "state_key": "",
        "type": "m.room.power_levels",
        "unsigned": {
            "age": 45
        }
    });

    assert_matches!(
        from_json_value::<AnyRoomEventStub>(json_data),
        Ok(AnyRoomEventStub::State(
            AnyStateEventStub::RoomPowerLevels(StateEventStub {
                content: PowerLevelsEventContent {
                    ban, ..
                },
                ..
            })
        ))
        if ban == js_int::Int::new(50).unwrap()
    );
}

#[test]
fn message_event_stub_deserialization() {
    let json_data = message_event_stub();

    assert_matches!(
        from_json_value::<AnyRoomEventStub>(json_data),
        Ok(AnyRoomEventStub::Message(
            AnyMessageEventStub::RoomMessage(MessageEventStub {
                content: MessageEventContent::Text(TextMessageEventContent {
                    body,
                    formatted: Some(formatted),
                    relates_to: None,
                }),
                ..
            })
        ))
        if body == "baba" && formatted.body == "<strong>baba</strong>"
    );
}

#[test]
fn aliases_event_stub_deserialization() {
    let json_data = aliases_event_stub();

    assert_matches!(
        from_json_value::<AnyRoomEventStub>(json_data),
        Ok(AnyRoomEventStub::State(
            AnyStateEventStub::RoomAliases(StateEventStub {
                content: AliasesEventContent {
                    aliases,
                },
                ..
            })
        ))
        if aliases == vec![ RoomAliasId::try_from("#somewhere:localhost").unwrap() ]
    );
}

#[test]
fn message_room_event_deserialization() {
    let json_data = message_event();

    assert_matches!(
        from_json_value::<AnyRoomEvent>(json_data),
        Ok(AnyRoomEvent::Message(
            AnyMessageEvent::RoomMessage(MessageEvent {
                content: MessageEventContent::Text(TextMessageEventContent {
                    body,
                    formatted: Some(formatted),
                    relates_to: None,
                }),
                ..
            })
        ))
        if body == "baba" && formatted.body == "<strong>baba</strong>"
    );
}

#[test]
fn alias_room_event_deserialization() {
    let json_data = aliases_event();

    assert_matches!(
        from_json_value::<AnyRoomEvent>(json_data),
        Ok(AnyRoomEvent::State(
            AnyStateEvent::RoomAliases(StateEvent {
                content: AliasesEventContent {
                    aliases,
                },
                ..
            })
        ))
        if aliases == vec![ RoomAliasId::try_from("#somewhere:localhost").unwrap() ]
    );
}

#[test]
fn message_event_deserialization() {
    let json_data = message_event();

    assert_matches!(
        from_json_value::<AnyEvent>(json_data),
        Ok(AnyEvent::Message(
            AnyMessageEvent::RoomMessage(MessageEvent {
                content: MessageEventContent::Text(TextMessageEventContent {
                    body,
                    formatted: Some(formatted),
                    relates_to: None,
                }),
                ..
            })
        ))
        if body == "baba" && formatted.body == "<strong>baba</strong>"
    );
}

#[test]
fn alias_event_deserialization() {
    let json_data = aliases_event();

    assert_matches!(
        from_json_value::<AnyEvent>(json_data),
        Ok(AnyEvent::State(
            AnyStateEvent::RoomAliases(StateEvent {
                content: AliasesEventContent {
                    aliases,
                },
                ..
            })
        ))
        if aliases == vec![ RoomAliasId::try_from("#somewhere:localhost").unwrap() ]
    );
}

#[test]
fn alias_event_field_access() {
    let json_data = aliases_event();

    assert_matches!(
        from_json_value::<AnyEvent>(json_data.clone()),
        Ok(AnyEvent::State(state_event))
        if state_event.state_key() == ""
            && state_event.room_id() == &RoomId::try_from("!room:room.com").unwrap()
            && state_event.event_id() == &EventId::try_from("$152037280074GZeOm:localhost").unwrap()
            && state_event.sender() == &UserId::try_from("@example:localhost").unwrap()
    );

    let deser = from_json_value::<AnyStateEvent>(json_data).unwrap();
    if let AnyStateEventContent::RoomAliases(AliasesEventContent { aliases }) = deser.content() {
        assert_eq!(aliases, vec![RoomAliasId::try_from("#somewhere:localhost").unwrap()])
    } else {
        panic!("the `Any*Event` enum's accessor methods may have been altered")
    }
}
