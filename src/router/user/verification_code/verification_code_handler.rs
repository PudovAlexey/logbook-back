pub async fn verification_code_handler(
    Path(email): Path<String>,
    State(shared_state): State<SharedStateType>,
) {
    
}
