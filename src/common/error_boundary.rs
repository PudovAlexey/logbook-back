use self::ErrorBoundary::BoundaryHandlers;

pub mod ErrorBoundary {
    use std::collections::HashMap;

    use axum::{response::IntoResponse, Json};
    use http::StatusCode;
    use serde::Serialize;
    use serde_json::{json, Value};

   pub trait BoundaryHandlers<T, R> {
       fn insert(self, params: T) -> Self;
       fn send(self, res: R) -> Result<impl IntoResponse, (StatusCode, Json<Value>)>;
    }

    #[derive(Clone, Serialize)]
   pub struct FieldError {
      pub  message: String,
      pub  description: String,
    }

   pub struct InsertFieldError {
      pub  key: String,
      pub  value: FieldError,
    }

   pub enum InsetWhere {
        Update(usize),
        Push,
    }

    pub struct InsertArrayFieldError {
       pub index: InsetWhere,
       pub value: InsertFieldError
    }

    #[derive(Clone, Serialize)]
    pub struct ObjectError {
        value: HashMap<String, FieldError>
    }

    impl ObjectError {
        pub fn new() -> Self {
            ObjectError {
                value: HashMap::new()
            }
        }
    }

    impl BoundaryHandlers<InsertFieldError, Json<Value>> for ObjectError  {
        fn insert(mut self, params: InsertFieldError) -> Self {

           self.value.insert(params.key, params.value);

            ObjectError {
                value: self.value
            }
        }

        fn send(self, res: Json<Value>) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
            if self.value.len() > 0 {
                Err((StatusCode::UNPROCESSABLE_ENTITY, Json(json!({"detail": self.value}))))
            } else {
                Ok((StatusCode::OK, Json(json!({"data": *res}))))
            }
        }
    }
     pub struct FieldArrayError {
      pub  value: Vec<ObjectError>
    }

    impl FieldArrayError {
      pub  fn new() -> Self {
            FieldArrayError {
                value: Vec::new()
            }
        }
    }

    impl BoundaryHandlers<InsertArrayFieldError, Json<Value>> for FieldArrayError {
        fn insert(mut self, params: InsertArrayFieldError) -> Self {
            // fn create_object_error() {
            //     ObjectError::new();

            // }
            match params.index {
                InsetWhere::Push => {
                    
                    let mut new_error = ObjectError::new();

                    new_error = new_error.insert(InsertFieldError {
                        key: params.value.key,
                        value: params.value.value,
                    });

                    self.value.push(new_error)
                }

                InsetWhere::Update(index) => {
                    let t = &index < &self.value.len();

                    if t {
                        self.value[index] = self.value[index].clone().insert(InsertFieldError {
                            key: params.value.key,
                            value: params.value.value,
                        });
                    } else {
                        
                    }

                    
                }
            }

            FieldArrayError {
                value: self.value
            }
        }

        fn send(self, res: Json<Value>) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
            if self.value.len() > 0 {
                Err((StatusCode::UNPROCESSABLE_ENTITY, Json(json!({"detail": self.value}))))
            } else {
                Ok((StatusCode::OK, Json(json!({"data": *res}))))
            }
        }
    }

    pub struct SimpleError {
        value: String
    }

    impl SimpleError {
       pub fn new() -> Self {
            SimpleError {
                value: String::new()
            }
        }
    }

    impl BoundaryHandlers<String, Json<Value>> for SimpleError {
       fn insert(mut self, params: String) -> Self {
            let new_value = format!("{}{}", self.value, params);
            SimpleError {
                value: new_value
            }
        }

        fn send(self, res: Json<Value>) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
        if self.value.len() > 0 {
            Err((StatusCode::UNPROCESSABLE_ENTITY, Json(json!({"detail": self.value}))))
        } else {
            Ok((StatusCode::OK, Json(json!({"data": *res}))))
        }
    }
    }
}

fn main() {
    let mut err = ErrorBoundary::FieldArrayError::new();

    err = err.insert(ErrorBoundary::InsertArrayFieldError {
        index: ErrorBoundary::InsetWhere::Push,
        value:  ErrorBoundary::InsertFieldError {
            key: String::from("simple_key"),
            value: ErrorBoundary::FieldError {
                message: String::from("test"),
                description: String::from("test")
            }
        }    
    });
}
