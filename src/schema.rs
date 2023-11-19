// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "priority_type"))]
    pub struct PriorityType;

    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "status_type"))]
    pub struct StatusType;
}

diesel::table! {
    categories (id) {
        id -> Uuid,
        #[max_length = 256]
        en_string -> Varchar,
        #[max_length = 256]
        fr_string -> Varchar,
        en_description -> Nullable<Text>,
        fr_description -> Nullable<Text>,
    }
}

diesel::table! {
    documents (id) {
        id -> Uuid,
        template_id -> Uuid,
        title_text_id -> Uuid,
        purpose_text_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        #[max_length = 64]
        security_classification -> Varchar,
        published -> Bool,
        created_by_id -> Uuid,
    }
}

diesel::table! {
    email_verification_code (id) {
        id -> Uuid,
        #[max_length = 128]
        email_address -> Varchar,
        #[max_length = 5]
        activation_code -> Varchar,
        expires_on -> Timestamp,
    }
}

diesel::table! {
    keywords (id) {
        id -> Uuid,
        #[max_length = 256]
        en_string -> Varchar,
        #[max_length = 256]
        fr_string -> Varchar,
        en_description -> Nullable<Text>,
        fr_description -> Nullable<Text>,
    }
}

diesel::table! {
    metadata (id) {
        id -> Uuid,
        #[max_length = 256]
        searchable_title_en -> Varchar,
        #[max_length = 256]
        searchable_title_fr -> Varchar,
        document_id -> Uuid,
        author_id -> Uuid,
        subject_id -> Nullable<Uuid>,
        category_id -> Nullable<Uuid>,
        summary_text_en -> Text,
        summary_text_fr -> Text,
        keyword_ids -> Nullable<Array<Nullable<Uuid>>>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    password_reset_token (id) {
        id -> Uuid,
        #[max_length = 128]
        email_address -> Varchar,
        #[max_length = 36]
        reset_token -> Varchar,
        expires_on -> Timestamp,
    }
}

diesel::table! {
    sections (id) {
        id -> Uuid,
        document_id -> Uuid,
        template_section_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        created_by_id -> Uuid,
    }
}

diesel::table! {
    subjects (id) {
        id -> Uuid,
        #[max_length = 256]
        en_string -> Varchar,
        #[max_length = 256]
        fr_string -> Varchar,
        en_description -> Nullable<Text>,
        fr_description -> Nullable<Text>,
    }
}

diesel::table! {
    template_sections (id) {
        id -> Uuid,
        template_id -> Uuid,
        header_text_id -> Uuid,
        order_number -> Int4,
        help_text_id -> Uuid,
        character_limit -> Nullable<Int4>,
    }
}

diesel::table! {
    templates (id) {
        id -> Uuid,
        name_text_id -> Uuid,
        purpose_text_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        #[max_length = 64]
        slug -> Varchar,
        active -> Bool,
    }
}

diesel::table! {
    texts (id, lang) {
        id -> Uuid,
        section_id -> Nullable<Uuid>,
        #[max_length = 2]
        lang -> Varchar,
        content -> Array<Nullable<Text>>,
        keywords -> Nullable<Jsonb>,
        translated -> Array<Nullable<Bool>>,
        machine_translation -> Array<Nullable<Bool>>,
        created_at -> Array<Nullable<Timestamp>>,
        created_by_id -> Array<Nullable<Uuid>>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::PriorityType;
    use super::sql_types::StatusType;

    todos (id) {
        id -> Uuid,
        list_id -> Uuid,
        #[max_length = 255]
        title -> Varchar,
        description -> Nullable<Text>,
        priority -> PriorityType,
        status -> StatusType,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    todos_list (id) {
        id -> Uuid,
        user_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        hash -> Bytea,
        #[max_length = 255]
        salt -> Varchar,
        #[max_length = 128]
        email -> Varchar,
        #[max_length = 32]
        user_name -> Varchar,
        #[max_length = 32]
        slug -> Varchar,
        created_at -> Timestamp,
        #[max_length = 32]
        role -> Varchar,
        validated -> Bool,
    }
}

diesel::joinable!(documents -> templates (template_id));
diesel::joinable!(metadata -> documents (document_id));
diesel::joinable!(sections -> documents (document_id));
diesel::joinable!(sections -> template_sections (template_section_id));
diesel::joinable!(template_sections -> templates (template_id));
diesel::joinable!(texts -> sections (section_id));
diesel::joinable!(todos -> todos_list (list_id));
diesel::joinable!(todos_list -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    categories,
    documents,
    email_verification_code,
    keywords,
    metadata,
    password_reset_token,
    sections,
    subjects,
    template_sections,
    templates,
    texts,
    todos,
    todos_list,
    users,
);
