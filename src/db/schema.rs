table! {
    attachments (id) {
        id -> Integer,
        uuid -> Text,
        user_id -> Integer,
        project_id -> Nullable<Integer>,
        issue_id -> Nullable<Integer>,
        comment_id -> Nullable<Integer>,
        name -> Text,
        download_count -> Integer,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    comments (id) {
        id -> Integer,
        enum_type -> SmallInt,
        issue_id -> Integer,
        user_id -> Integer,
        content -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    emails (id) {
        id -> Integer,
        user_id -> Integer,
        address -> Text,
        is_primary -> Bool,
        activated_at -> Nullable<Timestamp>,
        token -> Nullable<Text>,
        token_created_at -> Nullable<Timestamp>,
        notification -> SmallInt,
    }
}

table! {
    issues (id) {
        id -> Integer,
        number -> Integer,
        project_id -> Integer,
        user_id -> Integer,
        title -> Text,
        content -> Text,
        num_comments -> Integer,
        is_closed -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    issues_labels (id) {
        id -> Integer,
        label_id -> Integer,
        issue_id -> Integer,
    }
}

table! {
    issues_users (id) {
        id -> Integer,
        user_id -> Integer,
        issue_id -> Integer,
    }
}

table! {
    labels (id) {
        id -> Integer,
        project_id -> Integer,
        name -> Text,
        description -> Nullable<Text>,
        color -> Text,
    }
}

table! {
    project_topics (id) {
        id -> Integer,
        topic_id -> Integer,
        project_id -> Integer,
    }
}

table! {
    projects (id) {
        id -> Integer,
        user_id -> Integer,
        name -> Text,
        description -> Nullable<Text>,
        website -> Nullable<Text>,
        default_branch -> Nullable<Text>,
        num_watches -> Integer,
        num_stars -> Integer,
        num_forks -> Integer,
        num_issues -> Integer,
        num_issues_closed -> Integer,
        num_issues_open -> Integer,
        num_labels -> Integer,
        num_pull_reqs -> Integer,
        num_pull_reqs_closed -> Integer,
        num_pull_reqs_open -> Integer,
        num_milestones -> Integer,
        num_milestones_closed -> Integer,
        num_milestones_open -> Integer,
        num_releases -> Integer,
        is_private -> Bool,
        is_empty -> Bool,
        is_archived -> Bool,
        vcs -> Integer,
        is_fork -> Bool,
        forked_project -> Nullable<Integer>,
        disk_size -> BigInt,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    projects_users (id) {
        id -> Integer,
        project_id -> Integer,
        user_id -> Integer,
    }
}

table! {
    sessions (id) {
        id -> Integer,
        user_id -> Integer,
        token -> Text,
        created_at -> Timestamp,
        expires -> Timestamp,
    }
}

table! {
    topics (id) {
        id -> Integer,
        name -> Text,
    }
}

table! {
    users (id) {
        id -> Integer,
        types -> SmallInt,
        username -> Text,
        full_name -> Nullable<Text>,
        avatar -> Text,
        avatar_email -> Nullable<Text>,
        password -> Text,
        salt -> Text,
        location -> Nullable<Text>,
        website -> Nullable<Text>,
        description -> Nullable<Text>,
        language -> Text,
        must_change_password -> Bool,
        is_email_hidden -> Bool,
        is_admin -> Bool,
        is_organisation -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        last_seen_at -> Timestamp,
    }
}

joinable!(attachments -> comments (comment_id));
joinable!(attachments -> issues (issue_id));
joinable!(attachments -> projects (project_id));
joinable!(attachments -> users (user_id));
joinable!(comments -> issues (issue_id));
joinable!(comments -> users (user_id));
joinable!(emails -> users (user_id));
joinable!(issues -> projects (project_id));
joinable!(issues -> users (user_id));
joinable!(issues_labels -> issues (issue_id));
joinable!(issues_labels -> labels (label_id));
joinable!(issues_users -> issues (issue_id));
joinable!(issues_users -> users (user_id));
joinable!(labels -> projects (project_id));
joinable!(project_topics -> projects (project_id));
joinable!(project_topics -> topics (topic_id));
joinable!(projects -> users (user_id));
joinable!(projects_users -> projects (project_id));
joinable!(projects_users -> users (user_id));
joinable!(sessions -> users (user_id));

allow_tables_to_appear_in_same_query!(
    attachments,
    comments,
    emails,
    issues,
    issues_labels,
    issues_users,
    labels,
    project_topics,
    projects,
    projects_users,
    sessions,
    topics,
    users,
);
