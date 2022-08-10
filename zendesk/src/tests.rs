use anyhow::Result;
use pretty_assertions::assert_eq;

const TEST_CONTACT_EMAIL: &str = "testing@butts.com";

async fn search_test_contact(client: &crate::Client) -> Result<crate::types::Contact> {
    // Let's try to search for the contact by email.
    let contacts = client.list_contacts(Some(TEST_CONTACT_EMAIL)).await?;

    if contacts.len() != 1 {
        anyhow::bail!("Expected 1 contact, got {}: {:?}", contacts.len(), contacts);
    }

    let first = contacts.first().ok_or_else(|| anyhow::anyhow!("No contacts found"))?;

    Ok(first.clone())
}

/// Ensure our state is clean before we start.
async fn pre_start_cleanup(client: &crate::Client) -> Result<()> {
    // Delete the test contact if it exists.
    match client.list_contacts(Some(TEST_CONTACT_EMAIL)).await {
        Ok(contacts) => {
            for contact in contacts {
                client.delete_contact(&contact.id.to_string()).await?
            }
        }
        Err(err) => {
            anyhow::bail!("Unexpected error cleaning up contact: {}", err);
        }
    }

    Ok(())
}

#[tokio::test]
async fn test_contacts() {
    let client = crate::Client::new_from_env();

    // Ensure we have a good environment.
    pre_start_cleanup(&client).await.unwrap();

    // Let's create a new contact.
    let mut new_contact = crate::types::NewContact {
        first_name: "Test".to_string(),
        last_name: "Contact".to_string(),
        email: TEST_CONTACT_EMAIL.to_string(),
        phone: "555-555-5555".to_string(),
        ..Default::default()
    };
    let mut contact = client.create_contact(&new_contact).await.unwrap();

    assert_eq!(contact.first_name, "Test");
    assert_eq!(contact.last_name, "Contact");
    assert_eq!(contact.email, TEST_CONTACT_EMAIL);
    assert_eq!(contact.phone, "555-555-5555");

    assert!(contact.id > 0);

    // Let's update the contact.
    new_contact.first_name = "Updated".to_string();
    new_contact.last_name = "Test Contact".to_string();
    contact = client
        .update_contact(&contact.id.to_string(), &new_contact)
        .await
        .unwrap();

    assert_eq!(contact.first_name, "Updated");
    assert_eq!(contact.last_name, "Test Contact");

    // Let's get the contact by ID.
    contact = client.get_contact(&contact.id.to_string()).await.unwrap();

    assert_eq!(contact.first_name, "Updated");
    assert_eq!(contact.last_name, "Test Contact");
    assert_eq!(contact.email, TEST_CONTACT_EMAIL);
    assert_eq!(contact.phone, "555-555-5555");

    // Sleep for a few seconds to ensure the contact is there.
    // For some reason, without this sleep when running `cargo test --all` the
    // next function fails.
    // But if you run `cargo test -p zendesk` it works fine.
    std::thread::sleep(std::time::Duration::from_secs(15));

    contact = search_test_contact(&client).await.unwrap();

    assert_eq!(contact.first_name, "Updated");
    assert_eq!(contact.last_name, "Test Contact");
    assert_eq!(contact.email, TEST_CONTACT_EMAIL);
    assert_eq!(contact.phone, "555-555-5555");

    // Delete the contact.
    client.delete_contact(&contact.id.to_string()).await.unwrap();

    let result = search_test_contact(&client).await;
    assert!(result.is_err());

    if let Err(err) = result {
        assert_eq!(err.to_string(), "Expected 1 contact, got 0: []");
    }
}
