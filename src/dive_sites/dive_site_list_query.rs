pub struct DiveSiteParams {
    pub search_value: Option<String>,
    pub offset: i64,
    pub limit: i64,
}

pub fn dive_site_list_query(
    DiveSiteParams {
        search_value,
        offset,
        limit,
    }: DiveSiteParams,
) -> String {
    let title_similarity = &search_value;
    let description_similarity = &search_value;
    let search_similarity = &search_value;
    let order_similarity = &search_value;

    let title_similarity = if title_similarity.is_some() {
        format!(
            "similarity(title, '{}') as similarity_title",
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
            "word_similarity(title, '{}') > 0.1 OR word_similarity(description, '{}') > 0.1",
            search_similarity.clone().unwrap().as_mut(),
            search_similarity.clone().unwrap()
        )
    } else {
        String::new()
    };

    let order_similarity = if order_similarity.is_some() {
        format!("ORDER BY  similarity_title DESC, similarity_descr DESC")
    } else {
        String::new()
    };

    let query = format!(
        "SELECT id, title, description, image_id, {} {} FROM dive_site
  WHERE {} 
  {}
  OFFSET {}
  LIMIT {};",
        title_similarity,
        description_similarity,
        search_similarity,
        order_similarity,
        offset,
        limit,
    );

    query
}
