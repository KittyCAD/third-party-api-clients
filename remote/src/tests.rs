use pretty_assertions::assert_eq;

#[tokio::test]
#[ignore]
async fn test_remote_employments() {
    let mut client = crate::Client::new_from_env();
    client.set_base_url("https://gateway.remote-sandbox.com");

    let rand_num = rand::random::<u32>();
    let test_email = format!("bob{rand_num}@bobson.com");

    let managers = client
        .company_managers()
        .get_index(None, None, None)
        .await
        .unwrap()
        .data
        .unwrap()
        .company_managers
        .expect("There should be some company managers");

    let first_manager = managers.first().unwrap();
    let company_id = first_manager.company_id.as_ref().unwrap().clone();

    let todays_date = chrono::Utc::now().date_naive();
    let start_date = todays_date + chrono::Duration::days(7);
    let new_employee_response = client
        .employments()
        .post_create(&crate::types::EmploymentBasicParams {
            company_id: company_id.clone(),
            country_code: Some("AUS".to_string()),
            full_name: "Bob Bobson".to_string(),
            job_title: Some("Weebler of Bobs".to_string()),
            personal_email: test_email.to_string(),
            provisional_start_date: Some(start_date),
            type_: crate::types::EmploymentBasicParamsType::Employee,
        })
        .await;
    let new_employee_response = match new_employee_response {
        Ok(x) => x,
        Err(e) => match e {
            crate::types::error::Error::UnexpectedResponse(resp) => {
                let t = resp.text().await.unwrap();
                panic!("{}", t);
            }
            e => panic!("{:?}", e),
        },
    };
    let new_employment_data = new_employee_response.data.expect("Has data");
    let new_employment = new_employment_data
        .employment
        .expect("Employment key was empty on response");
    println!("emp: {new_employment:?}");

    let employments = client
        .employments()
        .get_index(
            None,
            None,      // page
            Some(100), // page_size
        )
        .await
        .expect("Employments to be listed");
    println!("emps: {employments}");

    let employment_response = client
        .employments()
        .get_show(&new_employment.id)
        .await
        .expect("User found");

    println!("emp: {:?}", employment_response.data);
    let employment_data = employment_response.data.expect("Has data");
    let employment = employment_data
        .employment
        .expect("Employment key was empty on response");

    assert_eq!(employment.id, new_employment.id);

    let timeoffs = client
        .time_off()
        .get_index_timeoff(None, None, None, None, None, None, None)
        .await
        .expect("Timeoffs failed to fetch")
        .data
        .expect("Data should exist")
        .timeoffs
        .expect("And timeoffs should not be none");

    println!("timeoffs: {timeoffs:?}");

    // Type gen for create still broken, need to work on generator
    // let timeoff_created = client.time_off().post_create_timeoff(
    //     &crate::types::CreateApprovedTimeoffParams {
    //     }
    // ).await.expect("Create should succeed");
}
