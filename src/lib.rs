use uuid::Uuid;

struct RootTishIssue {
    id: u64,
    uuid: Uuid;
    status: Status;

    title: String;
    description: String;

                 comments: Vec<TishIssueComment>;
}

struct TishIssueComment {
    text: String;
          author: TishAuthor;
}

struct TishAuthor{
    name: String;
          email: String;
}
