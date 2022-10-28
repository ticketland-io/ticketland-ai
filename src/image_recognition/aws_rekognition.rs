use aws_types::Credentials;
use aws_smithy_types::Blob;
use aws_sdk_rekognition::{
  Client,
  model::{Image, ModerationLabel},
};
use eyre::Result;

pub struct AwsRekognition {
  client: Client,
}

impl AwsRekognition {
  pub async fn new(access_key: &str, secret_key: &str) -> Self {
    let credentials = Credentials::new(access_key, secret_key, None, None, "");

    let config = aws_config::from_env()
    .credentials_provider(credentials)
    .load()
    .await;

    let client = Client::new(&config);

    Self {client}
  }

  pub async fn recognise(&self, image_content: Vec<u8>) -> Result<Option<Vec<ModerationLabel>>> {
    // this allows 5MB images; for larger images use `s3_object`. This requires that the image is already present in an S3 bucket.
    let image = Image::builder()
    .set_bytes(Some(Blob::new(image_content)))
    .build();

    let moderations_labels = self.client
    .detect_moderation_labels()
    .set_image(Some(image))
    .send()
    .await?
    .moderation_labels()
    .map(|labels| labels.to_vec());

    Ok(moderations_labels)
  }
}
