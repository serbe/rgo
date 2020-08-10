use std::fmt;

use deadpool_postgres::Client;
use serde::{Deserialize, Serialize};

use crate::error::ServiceError;
use crate::rpel::certificate::{Certificate, CertificateList};
use crate::rpel::company::{Company, CompanyList};
use crate::rpel::contact::{Contact, ContactList};
use crate::rpel::department::{Department, DepartmentList};
use crate::rpel::education::{Education, EducationList, EducationShort};
use crate::rpel::kind::{Kind, KindList};
use crate::rpel::post::{Post, PostList};
use crate::rpel::practice::{Practice, PracticeList, PracticeShort};
use crate::rpel::rank::{Rank, RankList};
use crate::rpel::scope::{Scope, ScopeList};
use crate::rpel::select::SelectItem;
use crate::rpel::siren::{Siren, SirenList};
use crate::rpel::siren_type::{SirenType, SirenTypeList};
use crate::rpel::user::{User, UserList};
use crate::services::{Item, Object};

#[derive(Deserialize, Serialize)]
pub enum DBObject {
    Null,
    Certificate(Certificate),
    CertificateList(Vec<CertificateList>),
    Company(Box<Company>),
    CompanyList(Vec<CompanyList>),
    Contact(Box<Contact>),
    ContactList(Vec<ContactList>),
    Department(Department),
    DepartmentList(Vec<DepartmentList>),
    Education(Education),
    EducationList(Vec<EducationList>),
    EducationShort(Vec<EducationShort>),
    Kind(Kind),
    KindList(Vec<KindList>),
    Post(Post),
    PostList(Vec<PostList>),
    Practice(Practice),
    PracticeList(Vec<PracticeList>),
    PracticeShort(Vec<PracticeShort>),
    Rank(Rank),
    RankList(Vec<RankList>),
    Scope(Scope),
    ScopeList(Vec<ScopeList>),
    SelectItem(Vec<SelectItem>),
    Siren(Box<Siren>),
    SirenList(Vec<SirenList>),
    SirenType(SirenType),
    SirenTypeList(Vec<SirenTypeList>),
    User(User),
    UserList(Vec<UserList>),
}

impl DBObject {
    pub fn name(&self) -> String {
        match self {
            DBObject::Null => String::new(),
            DBObject::Certificate(_) => String::from("Certificate"),
            DBObject::CertificateList(_) => String::from("CertificateList"),
            DBObject::Company(_) => String::from("Company"),
            DBObject::CompanyList(_) => String::from("CompanyList"),
            DBObject::Contact(_) => String::from("Contact"),
            DBObject::ContactList(_) => String::from("ContactList"),
            DBObject::Department(_) => String::from("Department"),
            DBObject::DepartmentList(_) => String::from("DepartmentList"),
            DBObject::Education(_) => String::from("Education"),
            DBObject::EducationList(_) => String::from("EducationList"),
            DBObject::EducationShort(_) => String::from("EducationShort"),
            DBObject::Kind(_) => String::from("Kind"),
            DBObject::KindList(_) => String::from("KindList"),
            DBObject::Post(_) => String::from("Post"),
            DBObject::PostList(_) => String::from("PostList"),
            DBObject::Practice(_) => String::from("Practice"),
            DBObject::PracticeList(_) => String::from("PracticeList"),
            DBObject::PracticeShort(_) => String::from("PracticeShort"),
            DBObject::Rank(_) => String::from("Rank"),
            DBObject::RankList(_) => String::from("RankList"),
            DBObject::Scope(_) => String::from("Scope"),
            DBObject::ScopeList(_) => String::from("ScopeList"),
            DBObject::SelectItem(_) => String::from("SelectItem"),
            DBObject::Siren(_) => String::from("Siren"),
            DBObject::SirenList(_) => String::from("SirenList"),
            DBObject::SirenType(_) => String::from("SirenType"),
            DBObject::SirenTypeList(_) => String::from("SirenTypeList"),
            DBObject::User(_) => String::from("User"),
            DBObject::UserList(_) => String::from("UserList"),
        }
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Object::Item(i) => write!(f, "Item {} {}", i.id, i.name),
            Object::List(s) => write!(f, "List {}", s),
        }
    }
}

pub async fn get_item(item: &Item, client: &Client) -> Result<DBObject, ServiceError> {
    match (item.name.as_str(), item.id) {
        ("Certificate", id) => Ok(DBObject::Certificate(Certificate::get(&client, id).await?)),
        ("Company", id) => Ok(DBObject::Company(Box::new(
            Company::get(&client, id).await?,
        ))),
        ("Contact", id) => Ok(DBObject::Contact(Box::new(
            Contact::get(&client, id).await?,
        ))),
        ("Department", id) => Ok(DBObject::Department(Department::get(&client, id).await?)),
        ("Education", id) => Ok(DBObject::Education(Education::get(&client, id).await?)),
        ("Kind", id) => Ok(DBObject::Kind(Kind::get(&client, id).await?)),
        ("Post", id) => Ok(DBObject::Post(Post::get(&client, id).await?)),
        ("Practice", id) => Ok(DBObject::Practice(Practice::get(&client, id).await?)),
        ("Rank", id) => Ok(DBObject::Rank(Rank::get(&client, id).await?)),
        ("Scope", id) => Ok(DBObject::Scope(Scope::get(&client, id).await?)),
        ("Siren", id) => Ok(DBObject::Siren(Box::new(Siren::get(&client, id).await?))),
        ("SirenType", id) => Ok(DBObject::SirenType(SirenType::get(&client, id).await?)),
        ("User", id) => Ok(DBObject::User(User::get(&client, id).await?)),
        (e, id) => Err(ServiceError::BadRequest(format!(
            "bad item object: {} {}",
            e, id
        ))),
    }
}

pub async fn get_list(name: &str, client: &Client) -> Result<DBObject, ServiceError> {
    match name {
        "CertificateList" => Ok(DBObject::CertificateList(
            CertificateList::get_all(&client).await?,
        )),
        "CompanyList" => Ok(DBObject::CompanyList(CompanyList::get_all(&client).await?)),
        "CompanySelect" => Ok(DBObject::SelectItem(
            SelectItem::company_all(&client).await?,
        )),
        "ContactList" => Ok(DBObject::ContactList(ContactList::get_all(&client).await?)),
        "ContactSelect" => Ok(DBObject::SelectItem(
            SelectItem::contact_all(&client).await?,
        )),
        "DepartmentList" => Ok(DBObject::DepartmentList(
            DepartmentList::get_all(&client).await?,
        )),
        "DepartmentSelect" => Ok(DBObject::SelectItem(
            SelectItem::department_all(&client).await?,
        )),
        "EducationList" => Ok(DBObject::EducationList(
            EducationList::get_all(&client).await?,
        )),
        "EducationNear" => Ok(DBObject::EducationShort(
            EducationShort::get_near(&client).await?,
        )),
        // "EducationShort" =>
        "KindList" => Ok(DBObject::KindList(KindList::get_all(&client).await?)),
        "KindSelect" => Ok(DBObject::SelectItem(SelectItem::kind_all(&client).await?)),
        "PostList" => Ok(DBObject::PostList(PostList::get_all(&client).await?)),
        "PostSelect" => Ok(DBObject::SelectItem(
            SelectItem::post_all(&client, false).await?,
        )),
        "PostGoSelect" => Ok(DBObject::SelectItem(
            SelectItem::post_all(&client, true).await?,
        )),
        "PracticeList" => Ok(DBObject::PracticeList(
            PracticeList::get_all(&client).await?,
        )),
        "PracticeNear" => Ok(DBObject::PracticeShort(
            PracticeShort::get_near(&client).await?,
        )),
        // "PracticeShort" =>
        "RankList" => Ok(DBObject::RankList(RankList::get_all(&client).await?)),
        "RankSelect" => Ok(DBObject::SelectItem(SelectItem::rank_all(&client).await?)),
        "ScopeList" => Ok(DBObject::ScopeList(ScopeList::get_all(&client).await?)),
        "ScopeSelect" => Ok(DBObject::SelectItem(SelectItem::scope_all(&client).await?)),
        // "SelectItem" =>
        "SirenList" => Ok(DBObject::SirenList(SirenList::get_all(&client).await?)),
        "SirenTypeList" => Ok(DBObject::SirenTypeList(
            SirenTypeList::get_all(&client).await?,
        )),
        "SirenTypeSelect" => Ok(DBObject::SelectItem(
            SelectItem::siren_type_all(&client).await?,
        )),
        "UserList" => Ok(DBObject::UserList(UserList::get_all(&client).await?)),
        e => Err(ServiceError::BadRequest(format!("bad list object: {}", e))),
    }
}

pub async fn insert_item(object: DBObject, client: &Client) -> Result<i64, ServiceError> {
    match object {
        DBObject::Certificate(item) => Ok(Certificate::insert(&client, item).await?.id),
        DBObject::Company(item) => Ok(Company::insert(&client, *item).await?.id),
        DBObject::Contact(item) => Ok(Contact::insert(&client, *item).await?.id),
        DBObject::Department(item) => Ok(Department::insert(&client, item).await?.id),
        DBObject::Education(item) => Ok(Education::insert(&client, item).await?.id),
        DBObject::Kind(item) => Ok(Kind::insert(&client, item).await?.id),
        DBObject::Post(item) => Ok(Post::insert(&client, item).await?.id),
        DBObject::Practice(item) => Ok(Practice::insert(&client, item).await?.id),
        DBObject::Rank(item) => Ok(Rank::insert(&client, item).await?.id),
        DBObject::Scope(item) => Ok(Scope::insert(&client, item).await?.id),
        DBObject::Siren(item) => Ok(Siren::insert(&client, *item).await?.id),
        DBObject::SirenType(item) => Ok(SirenType::insert(&client, item).await?.id),
        DBObject::User(item) => Ok(User::insert(&client, item).await?.id),
        _ => Err(ServiceError::BadRequest("bad item object".to_string())),
    }
}

pub async fn update_item(object: DBObject, client: &Client) -> Result<i64, ServiceError> {
    let res = match object {
        DBObject::Certificate(item) => Certificate::update(&client, item).await,
        DBObject::Company(item) => Company::update(&client, *item).await,
        DBObject::Contact(item) => Contact::update(&client, *item).await,
        DBObject::Department(item) => Department::update(&client, item).await,
        DBObject::Education(item) => Education::update(&client, item).await,
        DBObject::Kind(item) => Kind::update(&client, item).await,
        DBObject::Post(item) => Post::update(&client, item).await,
        DBObject::Practice(item) => Practice::update(&client, item).await,
        DBObject::Rank(item) => Rank::update(&client, item).await,
        DBObject::Scope(item) => Scope::update(&client, item).await,
        DBObject::Siren(item) => Siren::update(&client, *item).await,
        DBObject::SirenType(item) => SirenType::update(&client, item).await,
        DBObject::User(item) => User::update(&client, item).await,
        _ => return Err(ServiceError::BadRequest("bad item object".to_string())),
    }?;
    Ok(res as i64)
}

pub async fn delete_item(item: &Item, client: &Client) -> Result<i64, ServiceError> {
    let res = match item.name.as_str() {
        "Certificate" => Certificate::delete(client, item.id).await,
        "Company" => Company::delete(client, item.id).await,
        "Contact" => Contact::delete(client, item.id).await,
        "Department" => Department::delete(client, item.id).await,
        "Education" => Education::delete(client, item.id).await,
        "Kind" => Kind::delete(client, item.id).await,
        "Post" => Post::delete(client, item.id).await,
        "Practice" => Practice::delete(client, item.id).await,
        "Rank" => Rank::delete(client, item.id).await,
        "Scope" => Scope::delete(client, item.id).await,
        "Siren" => Siren::delete(client, item.id).await,
        "Siren_type" => SirenType::delete(client, item.id).await,
        "User" => User::delete(client, item.id).await,
        _ => {
            return Err(ServiceError::BadRequest(format!(
                "bad path {:?}",
                item.name
            )))
        }
    }?;
    Ok(res as i64)
}
