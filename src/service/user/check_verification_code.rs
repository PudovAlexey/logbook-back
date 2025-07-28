use crate::{
    common::redis::SetExpireItem, error::AppError, service::user::get_user_by_id::get_user_by,
    SharedStateType,
};

pub async fn check_verification_code(
    shared_state: SharedStateType,
    email: String,
) -> AppError<String> {
    let can_try_again = shared_state
        .redis
        .get_item(String::from("verification_handler_expire"))
        .unwrap();

    let user = get_user_by(shared_state, email).unwrap();

    let mut rng = rand::thread_rng();
    let random_number: u32 = rng.gen_range(100000..999999);

    let expires_token = shared_state.redis.set_expire_item(SetExpireItem {
        key: format!("change_password={}", { &email }),
        value: random_number,
        expires: 3600,
    });

    if expires_token.status == "success" {
        let mailer = Mailer::new(Mailer {
            header: ContentType::TEXT_HTML,
            to: email,
            subject: String::from("enter these code to reset password"),
            body: format!("your code is <span>{}</span>", { random_number }),
        });
    }

    todo!()
}
