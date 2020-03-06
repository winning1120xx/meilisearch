use serde_json::json;
use assert_json_diff::assert_json_eq;

mod common;

#[test]
fn create_index_with_name() {
    let mut server = common::Server::with_uid("movies");

    // 1 - Create a new index
    // Index with only a name "movies"
    // POST: /indexes

    let body = json!({
        "name": "movies",
    });

    let (res1_value, status_code) = server.create_index(body);
    assert_eq!(status_code, 201);
    assert_eq!(res1_value.as_object().unwrap().len(), 5);
    let r1_name = res1_value["name"].as_str().unwrap();
    let r1_uid = res1_value["uid"].as_str().unwrap();
    let r1_created_at = res1_value["createdAt"].as_str().unwrap();
    let r1_updated_at = res1_value["updatedAt"].as_str().unwrap();

    assert_eq!(r1_name, "movies");
    assert_eq!(r1_uid.len(), 8);
    assert!(r1_created_at.len() > 1);
    assert!(r1_updated_at.len() > 1);

    // 2 - Check the list of indexes
    // Must have 1 index with the exact same content that the request 1
    // GET: /indexes

    let (res2_value, status_code) = server.list_indexes();
    assert_eq!(status_code, 200);
    assert_eq!(res2_value.as_array().unwrap().len(), 1);
    assert_eq!(res2_value[0].as_object().unwrap().len(), 5);
    let r2_name = res2_value[0]["name"].as_str().unwrap();
    let r2_uid = res2_value[0]["uid"].as_str().unwrap();
    let r2_created_at = res2_value[0]["createdAt"].as_str().unwrap();
    let r2_updated_at = res2_value[0]["updatedAt"].as_str().unwrap();

    assert_eq!(r2_name, r1_name);
    assert_eq!(r2_uid.len(), r1_uid.len());
    assert_eq!(r2_created_at.len(), r1_created_at.len());
    assert_eq!(r2_updated_at.len(), r1_updated_at.len());
}

#[test]
fn create_index_with_uid() {
    let mut server = common::Server::with_uid("movies");

    // 1 - Create a new index
    // Index with only an uid "movies"
    // POST: /indexes

    let body = json!({
        "uid": "movies",
    });

    let (res1_value, status_code) = server.create_index(body);
    assert_eq!(status_code, 201);
    assert_eq!(res1_value.as_object().unwrap().len(), 5);
    let r1_name = res1_value["name"].as_str().unwrap();
    let r1_uid = res1_value["uid"].as_str().unwrap();
    let r1_created_at = res1_value["createdAt"].as_str().unwrap();
    let r1_updated_at = res1_value["updatedAt"].as_str().unwrap();

    assert_eq!(r1_name, "movies");
    assert_eq!(r1_uid, "movies");
    assert!(r1_created_at.len() > 1);
    assert!(r1_updated_at.len() > 1);

    // 2 - Check the list of indexes
    // Must have 1 index with the exact same content that the request 1
    // GET: /indexes

    let (res2_value, status_code) = server.list_indexes();
    assert_eq!(status_code, 200);
    assert_eq!(res2_value.as_array().unwrap().len(), 1);
    assert_eq!(res2_value[0].as_object().unwrap().len(), 5);
    let r2_name = res2_value[0]["name"].as_str().unwrap();
    let r2_uid = res2_value[0]["uid"].as_str().unwrap();
    let r2_created_at = res2_value[0]["createdAt"].as_str().unwrap();
    let r2_updated_at = res2_value[0]["updatedAt"].as_str().unwrap();

    assert_eq!(r2_name, r1_name);
    assert_eq!(r2_uid, r1_uid);
    assert_eq!(r2_created_at.len(), r1_created_at.len());
    assert_eq!(r2_updated_at.len(), r1_updated_at.len());
}

#[test]
fn create_index_with_name_and_uid() {
    let mut server = common::Server::with_uid("movies");

    // 1 - Create a new index
    // Index with a name "Films" and an uid "fn_movies"
    // POST: /indexes

    let body = json!({
        "name": "Films",
        "uid": "fr_movies",
    });
    let (res1_value, status_code) = server.create_index(body);
    assert_eq!(status_code, 201);

    assert_eq!(res1_value.as_object().unwrap().len(), 5);
    let r1_name = res1_value["name"].as_str().unwrap();
    let r1_uid = res1_value["uid"].as_str().unwrap();
    let r1_created_at = res1_value["createdAt"].as_str().unwrap();
    let r1_updated_at = res1_value["updatedAt"].as_str().unwrap();

    assert_eq!(r1_name, "Films");
    assert_eq!(r1_uid, "fr_movies");
    assert!(r1_created_at.len() > 1);
    assert!(r1_updated_at.len() > 1);

    // 2 - Check the list of indexes
    // Must have 1 index with the exact same content that the request 1
    // GET: /indexes

    let (res2_value, status_code) = server.list_indexes();
    assert_eq!(status_code, 200);

    assert_eq!(res2_value.as_array().unwrap().len(), 1);
    assert_eq!(res2_value[0].as_object().unwrap().len(), 5);
    let r2_name = res2_value[0]["name"].as_str().unwrap();
    let r2_uid = res2_value[0]["uid"].as_str().unwrap();
    let r2_created_at = res2_value[0]["createdAt"].as_str().unwrap();
    let r2_updated_at = res2_value[0]["updatedAt"].as_str().unwrap();

    assert_eq!(r2_name, r1_name);
    assert_eq!(r2_uid, r1_uid);
    assert_eq!(r2_created_at.len(), r1_created_at.len());
    assert_eq!(r2_updated_at.len(), r1_updated_at.len());
}

#[test]
fn rename_index() {
    let mut server = common::Server::with_uid("movies");
    // 1 - Create a new index
    // Index with only a name "movies"
    // POST: /indexes

    let body = json!({
        "name": "movies",
        "uid": "movies",
    });

    let (res1_value, status_code) = server.create_index(body);
    assert_eq!(status_code, 201);

    assert_eq!(res1_value.as_object().unwrap().len(), 5);
    let r1_name = res1_value["name"].as_str().unwrap();
    let r1_uid = res1_value["uid"].as_str().unwrap();
    let r1_created_at = res1_value["createdAt"].as_str().unwrap();
    let r1_updated_at = res1_value["updatedAt"].as_str().unwrap();

    assert_eq!(r1_name, "movies");
    assert_eq!(r1_uid.len(), 6);
    assert!(r1_created_at.len() > 1);
    assert!(r1_updated_at.len() > 1);

    // 2 - Update an index name
    // Update "movies" to "TV Shows"
    // PUT: /indexes/:uid

    let body = json!({
        "name": "TV Shows",
    });

    let (res2_value, status_code) = server.update_index(body);
    assert_eq!(status_code, 200);

    assert_eq!(res2_value.as_object().unwrap().len(), 5);
    let r2_name = res2_value["name"].as_str().unwrap();
    let r2_uid = res2_value["uid"].as_str().unwrap();
    let r2_created_at = res2_value["createdAt"].as_str().unwrap();
    let r2_updated_at = res2_value["updatedAt"].as_str().unwrap();

    assert_eq!(r2_name, "TV Shows");
    assert_eq!(r2_uid, r1_uid);
    assert_eq!(r2_created_at, r1_created_at);
    assert!(r2_updated_at.len() > 1);

    // 3 - Check the list of indexes
    // Must have 1 index with the exact same content that the request 2
    // GET: /indexes

    let (res3_value, status_code) = server.list_indexes();
    assert_eq!(status_code, 200);

    assert_eq!(res3_value.as_array().unwrap().len(), 1);
    assert_eq!(res3_value[0].as_object().unwrap().len(), 5);
    let r3_name = res3_value[0]["name"].as_str().unwrap();
    let r3_uid = res3_value[0]["uid"].as_str().unwrap();
    let r3_created_at = res3_value[0]["createdAt"].as_str().unwrap();
    let r3_updated_at = res3_value[0]["updatedAt"].as_str().unwrap();

    assert_eq!(r3_name, r2_name);
    assert_eq!(r3_uid.len(), r1_uid.len());
    assert_eq!(r3_created_at.len(), r1_created_at.len());
    assert_eq!(r3_updated_at.len(), r2_updated_at.len());
}

#[test]
fn delete_index_and_recreate_it() {
    let mut server = common::Server::with_uid("movies");

    // 1 - Create a new index
    // Index with only a name "movies"
    // POST: /indexes

    let body = json!({
        "name": "movies",
        "uid": "movies",
    });

    let (res1_value, status_code) = server.create_index(body);
    assert_eq!(status_code, 201);

    assert_eq!(res1_value.as_object().unwrap().len(), 5);
    let r1_name = res1_value["name"].as_str().unwrap();
    let r1_uid = res1_value["uid"].as_str().unwrap();
    let r1_created_at = res1_value["createdAt"].as_str().unwrap();
    let r1_updated_at = res1_value["updatedAt"].as_str().unwrap();

    assert_eq!(r1_name, "movies");
    assert_eq!(r1_uid.len(), 6);
    assert!(r1_created_at.len() > 1);
    assert!(r1_updated_at.len() > 1);

    // 2 - Check the list of indexes
    // Must have 1 index with the exact same content that the request 1
    // GET: /indexes

    let (res2_value, status_code) = server.list_indexes();
    assert_eq!(status_code, 200);

    assert_eq!(res2_value.as_array().unwrap().len(), 1);
    assert_eq!(res2_value[0].as_object().unwrap().len(), 5);
    let r2_name = res2_value[0]["name"].as_str().unwrap();
    let r2_uid = res2_value[0]["uid"].as_str().unwrap();
    let r2_created_at = res2_value[0]["createdAt"].as_str().unwrap();
    let r2_updated_at = res2_value[0]["updatedAt"].as_str().unwrap();

    assert_eq!(r2_name, r1_name);
    assert_eq!(r2_uid.len(), r1_uid.len());
    assert_eq!(r2_created_at.len(), r1_created_at.len());
    assert_eq!(r2_updated_at.len(), r1_updated_at.len());

    // 3- Delete an index
    // Update "movies" to "TV Shows"
    // DELETE: /indexes/:uid

    let (_res2_value, status_code) = server.delete_index();
    assert_eq!(status_code, 204);

    // 4 - Check the list of indexes
    // Must have 0 index
    // GET: /indexes

    let (res2_value, status_code) = server.list_indexes();
    assert_eq!(status_code, 200);

    assert_eq!(res2_value.as_array().unwrap().len(), 0);

    // 5 - Create a new index
    // Index with only a name "movies"
    // POST: /indexes

    let body = json!({
        "name": "movies",
    });

    let (res1_value, status_code) = server.create_index(body);
    assert_eq!(status_code, 201);

    assert_eq!(res1_value.as_object().unwrap().len(), 5);
    let r1_name = res1_value["name"].as_str().unwrap();
    let r1_uid = res1_value["uid"].as_str().unwrap();
    let r1_created_at = res1_value["createdAt"].as_str().unwrap();
    let r1_updated_at = res1_value["updatedAt"].as_str().unwrap();

    assert_eq!(r1_name, "movies");
    assert_eq!(r1_uid.len(), 8);
    assert!(r1_created_at.len() > 1);
    assert!(r1_updated_at.len() > 1);

    // 6 - Check the list of indexes
    // Must have 1 index with the exact same content that the request 1
    // GET: /indexes

    let (res2_value, status_code) = server.list_indexes();
    assert_eq!(status_code, 200);

    assert_eq!(res2_value.as_array().unwrap().len(), 1);
    assert_eq!(res2_value[0].as_object().unwrap().len(), 5);
    let r2_name = res2_value[0]["name"].as_str().unwrap();
    let r2_uid = res2_value[0]["uid"].as_str().unwrap();
    let r2_created_at = res2_value[0]["createdAt"].as_str().unwrap();
    let r2_updated_at = res2_value[0]["updatedAt"].as_str().unwrap();

    assert_eq!(r2_name, r1_name);
    assert_eq!(r2_uid.len(), r1_uid.len());
    assert_eq!(r2_created_at.len(), r1_created_at.len());
    assert_eq!(r2_updated_at.len(), r1_updated_at.len());
}

#[test]
fn check_multiples_indexes() {
    let mut server = common::Server::with_uid("movies");

    // 1 - Create a new index
    // Index with only a name "movies"
    // POST: /indexes

    let body = json!({
        "name": "movies",
    });

    let (res1_value, status_code) = server.create_index(body);
    assert_eq!(status_code, 201);

    assert_eq!(res1_value.as_object().unwrap().len(), 5);
    let r1_name = res1_value["name"].as_str().unwrap();
    let r1_uid = res1_value["uid"].as_str().unwrap();
    let r1_created_at = res1_value["createdAt"].as_str().unwrap();
    let r1_updated_at = res1_value["updatedAt"].as_str().unwrap();

    assert_eq!(r1_name, "movies");
    assert_eq!(r1_uid.len(), 8);
    assert!(r1_created_at.len() > 1);
    assert!(r1_updated_at.len() > 1);

    // 2 - Check the list of indexes
    // Must have 1 index with the exact same content that the request 1
    // GET: /indexes

    let (res2_value, status_code) = server.list_indexes();
    assert_eq!(status_code, 200);

    assert_eq!(res2_value.as_array().unwrap().len(), 1);
    assert_eq!(res2_value[0].as_object().unwrap().len(), 5);
    let r2_0_name = res2_value[0]["name"].as_str().unwrap();
    let r2_0_uid = res2_value[0]["uid"].as_str().unwrap();
    let r2_0_created_at = res2_value[0]["createdAt"].as_str().unwrap();
    let r2_0_updated_at = res2_value[0]["updatedAt"].as_str().unwrap();

    assert_eq!(r2_0_name, r1_name);
    assert_eq!(r2_0_uid.len(), r1_uid.len());
    assert_eq!(r2_0_created_at.len(), r1_created_at.len());
    assert_eq!(r2_0_updated_at.len(), r1_updated_at.len());

    // 3 - Create a new index
    // Index with only a name "films"
    // POST: /indexes

    let body = json!({
        "name": "films",
    });

    let (res3_value, status_code) = server.create_index(body);
    assert_eq!(status_code, 201);

    assert_eq!(res3_value.as_object().unwrap().len(), 5);
    let r3_name = res3_value["name"].as_str().unwrap();
    let r3_uid = res3_value["uid"].as_str().unwrap();
    let r3_created_at = res3_value["createdAt"].as_str().unwrap();
    let r3_updated_at = res3_value["updatedAt"].as_str().unwrap();

    assert_eq!(r3_name, "films");
    assert_eq!(r3_uid.len(), 8);
    assert!(r3_created_at.len() > 1);
    assert!(r3_updated_at.len() > 1);

    // 4 - Check the list of indexes
    // Must have 2 index with the exact same content that the request 1 and 3
    // GET: /indexes

    let (res4_value, status_code) = server.list_indexes();
    assert_eq!(status_code, 200);

    assert_eq!(res4_value.as_array().unwrap().len(), 2);

    assert_eq!(res4_value[0].as_object().unwrap().len(), 5);
    let r4_0_name = res4_value[0]["name"].as_str().unwrap();
    let r4_0_uid = res4_value[0]["uid"].as_str().unwrap();
    let r4_0_created_at = res4_value[0]["createdAt"].as_str().unwrap();
    let r4_0_updated_at = res4_value[0]["updatedAt"].as_str().unwrap();

    assert_eq!(res4_value[1].as_object().unwrap().len(), 5);
    let r4_1_name = res4_value[1]["name"].as_str().unwrap();
    let r4_1_uid = res4_value[1]["uid"].as_str().unwrap();
    let r4_1_created_at = res4_value[1]["createdAt"].as_str().unwrap();
    let r4_1_updated_at = res4_value[1]["updatedAt"].as_str().unwrap();

    if r4_0_name == r1_name {
        assert_eq!(r4_0_name, r1_name);
        assert_eq!(r4_0_uid.len(), r1_uid.len());
        assert_eq!(r4_0_created_at.len(), r1_created_at.len());
        assert_eq!(r4_0_updated_at.len(), r1_updated_at.len());
    } else {
        assert_eq!(r4_0_name, r3_name);
        assert_eq!(r4_0_uid.len(), r3_uid.len());
        assert_eq!(r4_0_created_at.len(), r3_created_at.len());
        assert_eq!(r4_0_updated_at.len(), r3_updated_at.len());
    }

    if r4_1_name == r1_name {
        assert_eq!(r4_1_name, r1_name);
        assert_eq!(r4_1_uid.len(), r1_uid.len());
        assert_eq!(r4_1_created_at.len(), r1_created_at.len());
        assert_eq!(r4_1_updated_at.len(), r1_updated_at.len());
    } else {
        assert_eq!(r4_1_name, r3_name);
        assert_eq!(r4_1_uid.len(), r3_uid.len());
        assert_eq!(r4_1_created_at.len(), r3_created_at.len());
        assert_eq!(r4_1_updated_at.len(), r3_updated_at.len());
    }
}

#[test]
fn create_index_failed() {
    let mut server = common::Server::with_uid("movies");

    // 2 - Push index creation with empty json body
    // POST: /indexes

    let body = json!({});

    let (res_value, status_code) = server.create_index(body);
    assert_eq!(status_code, 400);

    let message = res_value["message"].as_str().unwrap();
    assert_eq!(res_value.as_object().unwrap().len(), 1);
    assert_eq!(message, "Index creation must have an uid");

    // 3 - Create a index with extra data
    // POST: /indexes

    let body = json!({
        "name": "movies",
        "active": true
    });

    let (res_value, status_code) = server.create_index(body);
    assert_eq!(status_code, 400);

    let message = res_value["message"].as_str().unwrap();
    assert_eq!(res_value.as_object().unwrap().len(), 1);
    assert_eq!(message, "invalid data");

    // 3 - Create a index with wrong data type
    // POST: /indexes

    let body = json!({
        "name": "movies",
        "uid": 0
    });

    let (res_value, status_code) = server.create_index(body);
    assert_eq!(status_code, 400);

    let message = res_value["message"].as_str().unwrap();
    assert_eq!(res_value.as_object().unwrap().len(), 1);
    assert_eq!(message, "invalid data");
}



// Resolve issue https://github.com/meilisearch/MeiliSearch/issues/492
#[test]
fn create_index_with_identifier_and_index() {
    let mut server = common::Server::with_uid("movies");

    let body = json!({
        "uid": "movies",
        "identifier": "id",
    });

    let (_response, status_code) = server.create_index(body);
    assert_eq!(status_code, 201);

    let body = json!([{
        "id": 123,
        "text": "The mask"
    }]);

    server.add_or_replace_multiple_documents(body.clone());

    let (response, _status_code) = server.get_document(123);

    let expect = json!({
        "id": 123,
        "text": "The mask"
    });

    assert_json_eq!(response, expect, ordered: false);
}

// Resolve issue https://github.com/meilisearch/MeiliSearch/issues/497
#[test]
fn create_index_with_invalid_uid() {
    let mut server = common::Server::with_uid("");

    let body = json!({
        "uid": "the movies"
    });

    let (response, status_code) = server.create_index(body);
    assert_eq!(status_code, 400);

    let message = response["message"].as_str().unwrap();
    assert_eq!(response.as_object().unwrap().len(), 1);
    assert_eq!(message, "Index must have a valid uid; Index uid can be of type integer or string only composed of alphanumeric characters, hyphens (-) and underscores (_).");

    let body = json!({
        "uid": "%$#"
    });

    let (response, status_code) = server.create_index(body);
    assert_eq!(status_code, 400);

    let message = response["message"].as_str().unwrap();
    assert_eq!(response.as_object().unwrap().len(), 1);
    assert_eq!(message, "Index must have a valid uid; Index uid can be of type integer or string only composed of alphanumeric characters, hyphens (-) and underscores (_).");

    let body = json!({
        "uid": "the~movies"
    });

    let (response, status_code) = server.create_index(body);
    assert_eq!(status_code, 400);

    let message = response["message"].as_str().unwrap();
    assert_eq!(response.as_object().unwrap().len(), 1);
    assert_eq!(message, "Index must have a valid uid; Index uid can be of type integer or string only composed of alphanumeric characters, hyphens (-) and underscores (_).");

    let body = json!({
        "uid": "🎉"
    });

    let (response, status_code) = server.create_index(body);
    assert_eq!(status_code, 400);

    let message = response["message"].as_str().unwrap();
    assert_eq!(response.as_object().unwrap().len(), 1);
    assert_eq!(message, "Index must have a valid uid; Index uid can be of type integer or string only composed of alphanumeric characters, hyphens (-) and underscores (_).");
}
