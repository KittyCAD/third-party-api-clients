use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug)]
pub struct Uploads {
    pub client: Client,
}

impl Uploads {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "Creates an upload\n\n```rust,no_run\nasync fn example_uploads_create() -> anyhow::Result<()> {\n    let client = discourse_api::Client::new_from_env();\n    let result: discourse_api::types::CreateUploadResponse = client\n        .uploads()\n        .create(\n            vec![discourse_api::types::multipart::Attachment {\n                name: \"thing\".to_string(),\n                filepath: Some(\"myfile.json\".into()),\n                content_type: Some(\"application/json\".to_string()),\n                data: std::fs::read(\"myfile.json\").unwrap(),\n            }],\n            &discourse_api::types::CreateUploadRequestBody {\n                type_: discourse_api::types::Type::CustomEmoji,\n                user_id: Some(4 as i64),\n                synchronous: Some(true),\n                file: Some(bytes::Bytes::from(\"some-string\")),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create<'a>(
        &'a self,
        attachments: Vec<crate::types::multipart::Attachment>,
        body: &crate::types::CreateUploadRequestBody,
    ) -> Result<crate::types::CreateUploadResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!("{}/{}", self.client.base_url, "uploads.json"),
        );
        req = req.bearer_auth(&self.client.token);
        use std::convert::TryInto;
        let mut form = reqwest::multipart::Form::new();
        let mut json_part = reqwest::multipart::Part::text(serde_json::to_string(&body)?);
        json_part = json_part.file_name(format!("{}.json", "body"));
        json_part = json_part.mime_str("application/json")?;
        form = form.part("body", json_part);
        for attachment in attachments {
            form = form.part(attachment.name.clone(), attachment.try_into()?);
        }

        req = req.multipart(form);
        let resp = req.send().await?;
        let status = resp.status();
        if status.is_success() {
            let text = resp.text().await.unwrap_or_default();
            serde_json::from_str(&text).map_err(|err| {
                crate::types::error::Error::from_serde_error(
                    format_serde_error::SerdeError::new(text.to_string(), err),
                    status,
                )
            })
        } else {
            let text = resp.text().await.unwrap_or_default();
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
        }
    }

    #[doc = "Initiates a direct external upload\n\nDirect external uploads bypass the usual method of creating uploads\nvia the POST /uploads route, and upload directly to an external provider,\nwhich by default is S3. This route begins the process, and will return\na unique identifier for the external upload as well as a presigned URL\nwhich is where the file binary blob should be uploaded to.\n\nOnce the upload is complete to the external service, you must call the\nPOST /complete-external-upload route using the unique identifier returned\nby this route, which will create any required Upload record in the Discourse\ndatabase and also move file from its temporary location to the final\ndestination in the external storage service.\n\nYou must have the correct permissions and CORS settings configured in your\nexternal provider. We support AWS S3 as the default. See:\n\nhttps://meta.discourse.org/t/-/210469#s3-multipart-direct-uploads-4.\n\nAn external file store must be set up and `enable_direct_s3_uploads` must\nbe set to true for this endpoint to function.\n\n\n\n```rust,no_run\nasync fn example_uploads_generate_presigned_put() -> anyhow::Result<()> {\n    let client = discourse_api::Client::new_from_env();\n    let result: discourse_api::types::GeneratePresignedPutResponse = client\n        .uploads()\n        .generate_presigned_put(&discourse_api::types::GeneratePresignedPutRequestBody {\n            type_: discourse_api::types::Type::CustomEmoji,\n            file_name: \"some-string\".to_string(),\n            file_size: 4 as i64,\n            metadata: Some(discourse_api::types::Metadata {\n                sha_1_checksum: Some(\"some-string\".to_string()),\n            }),\n        })\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn generate_presigned_put<'a>(
        &'a self,
        body: &crate::types::GeneratePresignedPutRequestBody,
    ) -> Result<crate::types::GeneratePresignedPutResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url, "uploads/generate-presigned-put.json"
            ),
        );
        req = req.bearer_auth(&self.client.token);
        req = req.json(body);
        let resp = req.send().await?;
        let status = resp.status();
        if status.is_success() {
            let text = resp.text().await.unwrap_or_default();
            serde_json::from_str(&text).map_err(|err| {
                crate::types::error::Error::from_serde_error(
                    format_serde_error::SerdeError::new(text.to_string(), err),
                    status,
                )
            })
        } else {
            let text = resp.text().await.unwrap_or_default();
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
        }
    }

    #[doc = "Completes a direct external upload\n\nCompletes an external upload initialized with /get-presigned-put. The\nfile will be moved from its temporary location in external storage to\na final destination in the S3 bucket. An Upload record will also be\ncreated in the database in most cases.\n\nIf a sha1-checksum was provided in the initial request it will also\nbe compared with the uploaded file in storage to make sure the same\nfile was uploaded. The file size will be compared for the same reason.\n\nYou must have the correct permissions and CORS settings configured in your\nexternal provider. We support AWS S3 as the default. See:\n\nhttps://meta.discourse.org/t/-/210469#s3-multipart-direct-uploads-4.\n\nAn external file store must be set up and `enable_direct_s3_uploads` must\nbe set to true for this endpoint to function.\n\n\n\n```rust,no_run\nasync fn example_uploads_complete_external() -> anyhow::Result<()> {\n    let client = discourse_api::Client::new_from_env();\n    let result: discourse_api::types::CompleteExternalUploadResponse = client\n        .uploads()\n        .complete_external(&discourse_api::types::CompleteExternalUploadRequestBody {\n            unique_identifier: \"some-string\".to_string(),\n            for_private_message: Some(\"some-string\".to_string()),\n            for_site_setting: Some(\"some-string\".to_string()),\n            pasted: Some(\"some-string\".to_string()),\n        })\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn complete_external<'a>(
        &'a self,
        body: &crate::types::CompleteExternalUploadRequestBody,
    ) -> Result<crate::types::CompleteExternalUploadResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url, "uploads/complete-external-upload.json"
            ),
        );
        req = req.bearer_auth(&self.client.token);
        req = req.json(body);
        let resp = req.send().await?;
        let status = resp.status();
        if status.is_success() {
            let text = resp.text().await.unwrap_or_default();
            serde_json::from_str(&text).map_err(|err| {
                crate::types::error::Error::from_serde_error(
                    format_serde_error::SerdeError::new(text.to_string(), err),
                    status,
                )
            })
        } else {
            let text = resp.text().await.unwrap_or_default();
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
        }
    }

    #[doc = "Creates a multipart external upload\n\nCreates a multipart upload in the external storage provider, storing\na temporary reference to the external upload similar to /get-presigned-put.\n\nYou must have the correct permissions and CORS settings configured in your\nexternal provider. We support AWS S3 as the default. See:\n\nhttps://meta.discourse.org/t/-/210469#s3-multipart-direct-uploads-4.\n\nAn external file store must be set up and `enable_direct_s3_uploads` must\nbe set to true for this endpoint to function.\n\n\n\n```rust,no_run\nasync fn example_uploads_create_multipart() -> anyhow::Result<()> {\n    let client = discourse_api::Client::new_from_env();\n    let result: discourse_api::types::CreateMultipartUploadResponse = client\n        .uploads()\n        .create_multipart(&discourse_api::types::CreateMultipartUploadRequestBody {\n            upload_type: discourse_api::types::UploadType::CustomEmoji,\n            file_name: \"some-string\".to_string(),\n            file_size: 4 as i64,\n            metadata: Some(discourse_api::types::Metadata {\n                sha_1_checksum: Some(\"some-string\".to_string()),\n            }),\n        })\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create_multipart<'a>(
        &'a self,
        body: &crate::types::CreateMultipartUploadRequestBody,
    ) -> Result<crate::types::CreateMultipartUploadResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url, "uploads/create-multipart.json"
            ),
        );
        req = req.bearer_auth(&self.client.token);
        req = req.json(body);
        let resp = req.send().await?;
        let status = resp.status();
        if status.is_success() {
            let text = resp.text().await.unwrap_or_default();
            serde_json::from_str(&text).map_err(|err| {
                crate::types::error::Error::from_serde_error(
                    format_serde_error::SerdeError::new(text.to_string(), err),
                    status,
                )
            })
        } else {
            let text = resp.text().await.unwrap_or_default();
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
        }
    }

    #[doc = "Generates batches of presigned URLs for multipart parts\n\nMultipart uploads are uploaded in chunks or parts to individual presigned\nURLs, similar to the one generated by /generate-presigned-put. The part\nnumbers provided must be between 1 and 10000. The total number of parts\nwill depend on the chunk size in bytes that you intend to use to upload\neach chunk. For example a 12MB file may have 2 5MB chunks and a final\n2MB chunk, for part numbers 1, 2, and 3.\n\nThis endpoint will return a presigned URL for each part number provided,\nwhich you can then use to send PUT requests for the binary chunk corresponding\nto that part. When the part is uploaded, the provider should return an\nETag for the part, and this should be stored along with the part number,\nbecause this is needed to complete the multipart upload.\n\nYou must have the correct permissions and CORS settings configured in your\nexternal provider. We support AWS S3 as the default. See:\n\nhttps://meta.discourse.org/t/-/210469#s3-multipart-direct-uploads-4.\n\nAn external file store must be set up and `enable_direct_s3_uploads` must\nbe set to true for this endpoint to function.\n\n\n\n```rust,no_run\nasync fn example_uploads_batch_presign_multipart_parts() -> anyhow::Result<()> {\n    let client = discourse_api::Client::new_from_env();\n    let result: discourse_api::types::BatchPresignMultipartPartsResponse = client\n        .uploads()\n        .batch_presign_multipart_parts(&discourse_api::types::BatchPresignMultipartPartsRequestBody {\n            part_numbers: vec![serde_json::Value::String(\"some-string\".to_string())],\n            unique_identifier: \"some-string\".to_string(),\n        })\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn batch_presign_multipart_parts<'a>(
        &'a self,
        body: &crate::types::BatchPresignMultipartPartsRequestBody,
    ) -> Result<crate::types::BatchPresignMultipartPartsResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url, "uploads/batch-presign-multipart-parts.json"
            ),
        );
        req = req.bearer_auth(&self.client.token);
        req = req.json(body);
        let resp = req.send().await?;
        let status = resp.status();
        if status.is_success() {
            let text = resp.text().await.unwrap_or_default();
            serde_json::from_str(&text).map_err(|err| {
                crate::types::error::Error::from_serde_error(
                    format_serde_error::SerdeError::new(text.to_string(), err),
                    status,
                )
            })
        } else {
            let text = resp.text().await.unwrap_or_default();
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
        }
    }

    #[doc = "Abort multipart upload\n\nThis endpoint aborts the multipart upload initiated with /create-multipart.\nThis should be used when cancelling the upload. It does not matter if parts\nwere already uploaded into the external storage provider.\n\nYou must have the correct permissions and CORS settings configured in your\nexternal provider. We support AWS S3 as the default. See:\n\nhttps://meta.discourse.org/t/-/210469#s3-multipart-direct-uploads-4.\n\nAn external file store must be set up and `enable_direct_s3_uploads` must\nbe set to true for this endpoint to function.\n\n\n\n```rust,no_run\nasync fn example_uploads_abort_multipart() -> anyhow::Result<()> {\n    let client = discourse_api::Client::new_from_env();\n    let result: discourse_api::types::AbortMultipartResponse = client\n        .uploads()\n        .abort_multipart(&discourse_api::types::AbortMultipartRequestBody {\n            external_upload_identifier: \"some-string\".to_string(),\n        })\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn abort_multipart<'a>(
        &'a self,
        body: &crate::types::AbortMultipartRequestBody,
    ) -> Result<crate::types::AbortMultipartResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url, "uploads/abort-multipart.json"
            ),
        );
        req = req.bearer_auth(&self.client.token);
        req = req.json(body);
        let resp = req.send().await?;
        let status = resp.status();
        if status.is_success() {
            let text = resp.text().await.unwrap_or_default();
            serde_json::from_str(&text).map_err(|err| {
                crate::types::error::Error::from_serde_error(
                    format_serde_error::SerdeError::new(text.to_string(), err),
                    status,
                )
            })
        } else {
            let text = resp.text().await.unwrap_or_default();
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
        }
    }

    #[doc = "Complete multipart upload\n\nCompletes the multipart upload in the external store, and copies the\nfile from its temporary location to its final location in the store.\nAll of the parts must have been uploaded to the external storage provider.\nAn Upload record will be completed in most cases once the file is copied\nto its final location.\n\nYou must have the correct permissions and CORS settings configured in your\nexternal provider. We support AWS S3 as the default. See:\n\nhttps://meta.discourse.org/t/-/210469#s3-multipart-direct-uploads-4.\n\nAn external file store must be set up and `enable_direct_s3_uploads` must\nbe set to true for this endpoint to function.\n\n\n\n```rust,no_run\nasync fn example_uploads_complete_multipart() -> anyhow::Result<()> {\n    let client = discourse_api::Client::new_from_env();\n    let result: discourse_api::types::CompleteMultipartResponse = client\n        .uploads()\n        .complete_multipart(&discourse_api::types::CompleteMultipartRequestBody {\n            unique_identifier: \"some-string\".to_string(),\n            parts: vec![serde_json::Value::String(\"some-string\".to_string())],\n        })\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn complete_multipart<'a>(
        &'a self,
        body: &crate::types::CompleteMultipartRequestBody,
    ) -> Result<crate::types::CompleteMultipartResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url, "uploads/complete-multipart.json"
            ),
        );
        req = req.bearer_auth(&self.client.token);
        req = req.json(body);
        let resp = req.send().await?;
        let status = resp.status();
        if status.is_success() {
            let text = resp.text().await.unwrap_or_default();
            serde_json::from_str(&text).map_err(|err| {
                crate::types::error::Error::from_serde_error(
                    format_serde_error::SerdeError::new(text.to_string(), err),
                    status,
                )
            })
        } else {
            let text = resp.text().await.unwrap_or_default();
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
        }
    }
}
