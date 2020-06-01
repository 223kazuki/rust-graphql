use juniper::FieldResult;

/// GraphQLの型の素になるPhoto構造体
#[derive(Clone, Debug)]
pub struct Photo {
    pub id: String,
    pub name: String,
    pub description: String,
}

/// 実際にGraphQLとしての型になるのは以下アトリビュートがついているこちら
#[juniper::object]
#[graphql(description = "A Project returns struct")]
impl Photo {
    pub fn id(&self) -> String {
        self.id.clone()
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn description(&self) -> String {
        self.description.clone()
    }

    pub fn url(&self) -> String {
        format!("http://hogehoge/{}", self.id)
    }
}

pub struct Query;
pub struct Mutation;

/// 各GraphQLのリゾルバの引数に与えられるコンテキスト
/// 今は仮実装で起動時に与えられたPhotoのベクタだけ持つ
#[derive(Clone, Debug)]
pub struct Context {
    pub photos: Vec<Photo>,
}
impl juniper::Context for Context {}

/// GraphQLのクエリ系リゾルバ
#[juniper::object(Context = Context)]
impl Query {
    fn all_photos(&self, context: &Context) -> FieldResult<Vec<Photo>> {
        Ok(context.photos.clone())
    }
}

/// GraphQLのミューテーション系(更新系)リゾルバ
#[juniper::object(Context = Context)]
impl Mutation {}

pub type Schema = juniper::RootNode<'static, Query, Mutation>;

pub fn create_schema() -> Schema {
    Schema::new(Query {}, Mutation {})
}
