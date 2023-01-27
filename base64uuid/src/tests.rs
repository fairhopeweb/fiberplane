use super::*;

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
    let debug = format!("{uuid:?}");
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

#[test]
fn serialize_bytes_for_non_human_readable() {
    let uuid = Base64Uuid::parse_str("b0c1ee86-6f46-4f1b-8d8b-7849e75dbcee").unwrap();
    let serialized = rmp_serde::to_vec(&uuid).unwrap();
    assert_eq!(serialized.len(), 18); // 16 bytes + 2 bytes for the prefix
    assert_eq!(serialized, rmp_serde::to_vec(&uuid.0).unwrap());
}

#[test]
fn deserialize_bytes_from_non_human_readable() {
    let uuid = Base64Uuid::parse_str("b0c1ee86-6f46-4f1b-8d8b-7849e75dbcee").unwrap();
    let serialized = rmp_serde::to_vec(&uuid).unwrap();
    let deserialized = rmp_serde::from_slice::<Base64Uuid>(&serialized).unwrap();
    assert_eq!(deserialized, uuid);
}

#[test]
fn partial_eq_uuid() {
    let id_a = Base64Uuid::new();
    let id_b: Uuid = id_a.into();

    assert_eq!(id_a, id_b);
    assert_eq!(id_b, id_a);
}

#[test]
fn partial_eq_string() {
    let id_a = Base64Uuid::new();
    let id_b = id_a.to_string();

    assert_eq!(id_a, id_b);
    assert_eq!(id_b, id_a);
}

#[test]
fn partial_eq_str() {
    let id_a = Base64Uuid::new();

    let view = id_a.to_string();
    let id_b = view.as_str();

    assert_eq!(id_a, id_b);
    assert_eq!(id_b, id_a);
}

#[test]
fn partial_eq_owned_cow_str() {
    let id_a = Base64Uuid::new();
    let id_b = Cow::Owned(id_a.to_string());

    assert_eq!(id_a, id_b);
    assert_eq!(id_b, id_a);
}

#[test]
fn partial_eq_borrowed_cow_str() {
    let id_a = Base64Uuid::new();

    let string_view = id_a.to_string();
    let str_view = string_view.as_str();
    let id_b = Cow::Borrowed(str_view);

    assert_eq!(id_a, id_b);
    assert_eq!(id_b, id_a);
}

#[test]
fn deref() {
    let id_a = Base64Uuid::new();
    let id_b: Uuid = id_a.into();

    assert_eq!(*id_a, id_b);
}
