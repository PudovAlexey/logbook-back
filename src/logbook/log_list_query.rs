use chrono::NaiveDateTime;

pub struct LogListParams {
    pub search_value: Option<String>,
    pub user_id: uuid::Uuid,
    pub start_date: Option<NaiveDateTime>,
    pub end_date: Option<NaiveDateTime>,
    pub offset: i64,
    pub limit: i64,
}

pub fn log_list_query(
    LogListParams {
        search_value,
        user_id,
        start_date,
        end_date,
        offset,
        limit,
    }: LogListParams,
) -> String {
    let title_similarity = &search_value;
    let description_similarity = &search_value;
    let search_similarity = &search_value;
    let order_similarity = &search_value;

    let title_similarity = if title_similarity.is_some() {
        format!(
            ", similarity(title, '{}') as similarity_title",
            title_similarity.clone().unwrap()
        )
    } else {
        String::new()
    };
    let description_similarity = if description_similarity.is_some() {
        format!(
            ", similarity(description, '{}') as similarity_descr",
            description_similarity.clone().unwrap()
        )
    } else {
        String::new()
    };
    let search_similarity = if search_similarity.is_some() {
        format!(
            "AND word_similarity(title, '{}') > 0.1 OR word_similarity(description, '{}') > 0.1",
            search_similarity.clone().unwrap().as_mut(),
            search_similarity.clone().unwrap()
        )
    } else {
        String::new()
    };

    let date_search = if start_date.is_some() && end_date.is_some() {
        format!(
            "AND '{}' <= start_datetime AND '{}' >= start_datetime",
            start_date.unwrap().to_string(),
            end_date.unwrap().to_string()
        )
    } else {
        String::new()
    };

    let order_similarity = if order_similarity.is_some() {
        format!("ORDER BY  similarity_title DESC, similarity_descr DESC")
    } else {
        String::new()
    };

    format!(
        "SELECT id, title, description, start_datetime, image_id {} {} FROM loginfo
    WHERE user_id = '{}' {} {}
    {}
    OFFSET {}
    LIMIT {};",
        title_similarity,
        description_similarity,
        user_id,
        date_search,
        search_similarity,
        order_similarity,
        offset,
        limit,
    )
}

// let result = if !search_value.is_empty() { format!("similarity({}, '{}')", title, search_value) } else { String::new() };

// 2024-05-1T14:50:17.770Z
