// @generated automatically by Diesel CLI.

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
    users,
);
