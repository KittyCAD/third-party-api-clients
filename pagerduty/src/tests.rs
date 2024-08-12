use anyhow::Result;
use pretty_assertions::assert_eq;

use crate::types::{EscalationPolicy, EscalationPolicyType};

const TEST_SERVICE_NAME: &str = "TEST SERVICE";

async fn search_test_service(client: &crate::Client) -> Result<crate::types::Service> {
    // Let's try to search for the service by email.
    let services = client.list_services().await?;

    for service in services {
        if service.name == TEST_SERVICE_NAME {
            return Ok(service);
        }
    }

    anyhow::bail!("Expected 1 service, got 0: []");
}

/// Ensure our state is clean before we start.
async fn pre_start_cleanup(client: &crate::Client) -> Result<()> {
    // Delete the test service if it exists.
    match search_test_service(client).await {
        Ok(service) => client.delete_service(&service.id.to_string()).await?,
        Err(err) => {
            if err.to_string() != "Expected 1 service, got 0: []" {
                anyhow::bail!("Unexpected error cleaning up service: {}", err);
            }
        }
    }

    Ok(())
}

#[tokio::test]
#[ignore] // We ignore since we no longer have a pager duty account.
async fn test_services() {
    let client = crate::Client::new_from_env();

    // Ensure we have a good environment.
    pre_start_cleanup(&client).await.unwrap();

    // Get the default escalation policy.
    let escalation_policies = client.list_escalation_policies().await.unwrap();
    let mut default_escalation_policy = String::new();
    for escalation_policy in escalation_policies {
        if escalation_policy.name == "Default" {
            default_escalation_policy = escalation_policy.id.to_string();
            break;
        }
    }

    assert!(!default_escalation_policy.is_empty());

    // Let's create a new service.
    let new_service = crate::types::Service {
        name: TEST_SERVICE_NAME.to_string(),
        escalation_policy: EscalationPolicy {
            id: default_escalation_policy,
            type_: Some(EscalationPolicyType::EscalationPolicyReference),
            ..Default::default()
        },
        ..Default::default()
    };

    let mut service = client.create_service(&new_service).await.unwrap();

    assert_eq!(service.name, TEST_SERVICE_NAME);

    assert!(!service.id.is_empty());

    // Let's update the service.
    service.description = "Updated".to_string();
    service = client.update_service(&service).await.unwrap();

    assert_eq!(service.description, "Updated");

    // Let's get the service by ID.
    service = client.get_service(&service.id.to_string()).await.unwrap();

    assert_eq!(service.name, TEST_SERVICE_NAME);
    assert_eq!(service.description, "Updated");

    service = search_test_service(&client).await.unwrap();

    assert_eq!(service.name, TEST_SERVICE_NAME);
    assert_eq!(service.description, "Updated");

    // Delete the service.
    client
        .delete_service(&service.id.to_string())
        .await
        .unwrap();

    let result = search_test_service(&client).await;
    assert!(result.is_err());

    if let Err(err) = result {
        assert_eq!(err.to_string(), "Expected 1 service, got 0: []");
    }
}
