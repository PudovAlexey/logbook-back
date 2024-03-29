// use std::collections::HashMap;
use axum::{extract::Multipart as AxumMultipart, Json};
use serde::Deserialize;
use serde_json::Value;

enum MultipartField {
    Image,
    Crop,
    None
}

impl MultipartField {
   fn new(value: String) -> Self {
    if value == "image" {
        MultipartField::Image
    } else if value == "crop" {
        MultipartField::Crop
    } else {
        MultipartField::None
    }
   }
}

#[derive(Debug, Deserialize)]

pub struct CropParams {
   pub x: u32,
   pub y: u32,
   pub width: u32,
   pub height: u32
}


#[derive(Debug)]
pub struct ImageMultipart {
   pub image_vec: Vec<u8>,
   pub crop: CropParams,
   pub filename: String,
}

impl ImageMultipart {
  pub async fn new(mut value: AxumMultipart) -> Self {
        let mut image: Vec<u8> = Vec::new();
        let mut filename = String::new();
        let mut crop = CropParams {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        };

        while let Some(mut field) = value.next_field().await.unwrap() {
            let name = field.name().unwrap().to_string();

            let multipart = MultipartField::new(name);

            match multipart {
                MultipartField::Crop => {
                    let value = field.bytes().await.unwrap();
                    let value_str = std::str::from_utf8(&value).unwrap();
                    crop = serde_json::from_str(value_str).unwrap();

                },
                MultipartField::Image => {
                    filename = String::from(field.file_name().unwrap());
                    
                    while let Some(chunk) = field.chunk().await.unwrap() {
                        image.extend_from_slice(chunk.as_ref());
                    }
                },
                (_) => {}
            }
            
        }

        ImageMultipart {
            image_vec: image,
            crop,
            filename,
        }
    }
}