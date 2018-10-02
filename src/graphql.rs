use juniper::*;

pub struct QueryRoot;

#[derive(GraphQLObject)]
/// User
pub struct User {
    pub id: i32,
    pub email: String,
}


#[derive(GraphQLObject)]
/// Card object
pub struct Card {
    id: i32,
    title: String,
    content: String,
    author: User,
}

graphql_object!(QueryRoot: () |&self| {
    field card(&executor, id: i32) -> FieldResult<Card> {
        Ok(Card {
            id,
            title: "Example".to_string(),
            content: "Conten".to_string(),
            author: User {
                id: 123,
                email: "foo@bar".to_string(),
            }
        })
    }
});

pub struct MutationRoot;

#[derive(GraphQLInputObject)]
pub struct NewCard {
    pub title: String,
    pub content: String,
}

graphql_object!(MutationRoot: () |&self| {
    field createCard(&executor, new_card: NewCard) -> FieldResult<Card> {
        Ok(Card {
            id: 123,
            title: new_card.title.clone(),
            content: new_card.content.clone(),
            author: User {
                id: 123,
                email: "foo@bar".to_string(),
            }
        })
    }
});

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot, MutationRoot)
}
