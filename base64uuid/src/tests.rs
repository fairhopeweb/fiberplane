use super::*;
use serde_json;

#[test]
fn from_uuid() {
    let known_id = Uuid::parse_str("b0c1ee86-6f46-4f1b-8d8b-7849e75dbcee").unwrap();
    let as_b64 = Base64Uuid::from(known_id);
    assert_eq!(as_b64.to_string(), "sMHuhm9GTxuNi3hJ51287g");
}

#[test]
fn from_str() {
    let id = Base64Uuid::parse_str("sMHuhm9GTxuNi3hJ51287g").unwrap();
    assert_eq!(id.to_string(), "sMHuhm9GTxuNi3hJ51287g");

    let id = Base64Uuid::parse_str("b0c1ee86-6f46-4f1b-8d8b-7849e75dbcee").unwrap();
    assert_eq!(id.to_string(), "sMHuhm9GTxuNi3hJ51287g");
}

#[test]
fn as_uuid() {
    let id = Base64Uuid::parse_str("sMHuhm9GTxuNi3hJ51287g").unwrap();
    assert_eq!(
        id.as_uuid(),
        &Uuid::parse_str("b0c1ee86-6f46-4f1b-8d8b-7849e75dbcee").unwrap()
    );
}

#[test]
fn serialization() {
    #[derive(Serialize)]
    struct Person {
        id: Base64Uuid,
    }

    let person = Person {
        id: Base64Uuid::parse_str("b0c1ee86-6f46-4f1b-8d8b-7849e75dbcee").unwrap(),
    };
    let string = serde_json::to_string(&person).unwrap();
    assert_eq!(string, "{\"id\":\"sMHuhm9GTxuNi3hJ51287g\"}");
}

#[test]
fn deserialization() {
    #[derive(Deserialize)]
    struct Person {
        id: Base64Uuid,
    }

    let string = "{\"id\":\"sMHuhm9GTxuNi3hJ51287g\"}";
    let person: Person = serde_json::from_str(string).unwrap();
    assert_eq!(person.id.to_string(), "sMHuhm9GTxuNi3hJ51287g");
}

#[test]
fn debug_format() {
    let uuid = Base64Uuid::parse_str("b0c1ee86-6f46-4f1b-8d8b-7849e75dbcee").unwrap();
    let debug = format!("{:?}", uuid);
    assert_eq!(&debug, "sMHuhm9GTxuNi3hJ51287g")
}

#[test]
fn no_start_dash() {
    // As the UUID is randomly generated, sample it 10'000 times to get a good sense if it works or not
    for _ in 0..10000 {
        let uuid = Base64Uuid::new();
        let str = uuid.to_string();

        assert!(!str.starts_with('-'), "Base64 UUID started with dash");
    }
}
