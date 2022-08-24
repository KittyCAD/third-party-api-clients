
use pretty_assertions::assert_eq;

#[tokio::test]
async fn test_front_contacts() {
    let client = crate::Client::new_from_env();

    let handles = vec![crate::types::ContactHandle {
        source: crate::types::Source::Email,
        handle: "test@buttz.com".to_string(),
    }];

    let new_contact = crate::types::CreateContact {
        name: Some("test_name".to_string()),
        description: None,
        avatar: None,
        is_spammer: None,
        links: None,
        group_names: None,
        custom_fields: None,
        handles: Some(handles),
    };

    let contact = client.contacts().create(&new_contact).await.unwrap();

    if let Some(name) = contact.name {
        assert_eq!(name, "test_name".to_string());
    }
    
    // new_contact.name = Some("update_name".to_string());
    let contact_id = contact.id.unwrap();
    let updated_contact_body = crate::types::Contact {
        name: Some("update_name".to_string()),
        description: None,
        avatar: None,
        is_spammer: None,
        links: None,
        group_names: None,
        custom_fields: None,
    };
    
    client.contacts().update(&contact_id, &updated_contact_body).await;
    let updated_contact = client.contacts().get(&contact_id).await.unwrap();
    
    if let Some(name) = updated_contact.name {
        assert_eq!(name, "update_name".to_string());
    }
    
    client.contacts().delete(&contact_id).await;
    
    let deleted_contact = client.contacts().get(&contact_id).await;

    assert!(deleted_contact.is_err());

}
