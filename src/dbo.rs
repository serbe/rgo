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

#[derive(Debug, Deserialize, Serialize)]
pub enum DbObject {
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

impl DbObject {
    pub fn name(&self) -> String {
        match self {
            DbObject::Null => String::new(),
            DbObject::Certificate(_) => String::from("Certificate"),
            DbObject::CertificateList(_) => String::from("CertificateList"),
            DbObject::Company(_) => String::from("Company"),
            DbObject::CompanyList(_) => String::from("CompanyList"),
            DbObject::Contact(_) => String::from("Contact"),
            DbObject::ContactList(_) => String::from("ContactList"),
            DbObject::Department(_) => String::from("Department"),
            DbObject::DepartmentList(_) => String::from("DepartmentList"),
            DbObject::Education(_) => String::from("Education"),
            DbObject::EducationList(_) => String::from("EducationList"),
            DbObject::EducationShort(_) => String::from("EducationShort"),
            DbObject::Kind(_) => String::from("Kind"),
            DbObject::KindList(_) => String::from("KindList"),
            DbObject::Post(_) => String::from("Post"),
            DbObject::PostList(_) => String::from("PostList"),
            DbObject::Practice(_) => String::from("Practice"),
            DbObject::PracticeList(_) => String::from("PracticeList"),
            DbObject::PracticeShort(_) => String::from("PracticeShort"),
            DbObject::Rank(_) => String::from("Rank"),
            DbObject::RankList(_) => String::from("RankList"),
            DbObject::Scope(_) => String::from("Scope"),
            DbObject::ScopeList(_) => String::from("ScopeList"),
            DbObject::SelectItem(_) => String::from("SelectItem"),
            DbObject::Siren(_) => String::from("Siren"),
            DbObject::SirenList(_) => String::from("SirenList"),
            DbObject::SirenType(_) => String::from("SirenType"),
            DbObject::SirenTypeList(_) => String::from("SirenTypeList"),
            DbObject::User(_) => String::from("User"),
            DbObject::UserList(_) => String::from("UserList"),
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

pub async fn get_item(item: &Item, client: &Client) -> Result<DbObject, ServiceError> {
    match (item.name.as_str(), item.id) {
        ("Certificate", id) => Ok(DbObject::Certificate(Certificate::get(&client, id).await?)),
        ("Company", id) => Ok(DbObject::Company(Box::new(
            Company::get(&client, id).await?,
        ))),
        ("Contact", id) => Ok(DbObject::Contact(Box::new(
            Contact::get(&client, id).await?,
        ))),
        ("Department", id) => Ok(DbObject::Department(Department::get(&client, id).await?)),
        ("Education", id) => Ok(DbObject::Education(Education::get(&client, id).await?)),
        ("Kind", id) => Ok(DbObject::Kind(Kind::get(&client, id).await?)),
        ("Post", id) => Ok(DbObject::Post(Post::get(&client, id).await?)),
        ("Practice", id) => Ok(DbObject::Practice(Practice::get(&client, id).await?)),
        ("Rank", id) => Ok(DbObject::Rank(Rank::get(&client, id).await?)),
        ("Scope", id) => Ok(DbObject::Scope(Scope::get(&client, id).await?)),
        ("Siren", id) => Ok(DbObject::Siren(Box::new(Siren::get(&client, id).await?))),
        ("SirenType", id) => Ok(DbObject::SirenType(SirenType::get(&client, id).await?)),
        ("User", id) => Ok(DbObject::User(User::get(&client, id).await?)),
        (e, id) => Err(ServiceError::BadRequest(format!(
            "bad item object: {} {}",
            e, id
        ))),
    }
}

pub async fn get_list(name: &str, client: &Client) -> Result<DbObject, ServiceError> {
    match name {
        "CertificateList" => Ok(DbObject::CertificateList(
            CertificateList::get_all(&client).await?,
        )),
        "CompanyList" => Ok(DbObject::CompanyList(CompanyList::get_all(&client).await?)),
        "CompanySelect" => Ok(DbObject::SelectItem(
            SelectItem::company_all(&client).await?,
        )),
        "ContactList" => Ok(DbObject::ContactList(ContactList::get_all(&client).await?)),
        "ContactSelect" => Ok(DbObject::SelectItem(
            SelectItem::contact_all(&client).await?,
        )),
        "DepartmentList" => Ok(DbObject::DepartmentList(
            DepartmentList::get_all(&client).await?,
        )),
        "DepartmentSelect" => Ok(DbObject::SelectItem(
            SelectItem::department_all(&client).await?,
        )),
        "EducationList" => Ok(DbObject::EducationList(
            EducationList::get_all(&client).await?,
        )),
        "EducationNear" => Ok(DbObject::EducationShort(
            EducationShort::get_near(&client).await?,
        )),
        // "EducationShort" =>
        "KindList" => Ok(DbObject::KindList(KindList::get_all(&client).await?)),
        "KindSelect" => Ok(DbObject::SelectItem(SelectItem::kind_all(&client).await?)),
        "PostList" => Ok(DbObject::PostList(PostList::get_all(&client).await?)),
        "PostSelect" => Ok(DbObject::SelectItem(
            SelectItem::post_all(&client, false).await?,
        )),
        "PostGoSelect" => Ok(DbObject::SelectItem(
            SelectItem::post_all(&client, true).await?,
        )),
        "PracticeList" => Ok(DbObject::PracticeList(
            PracticeList::get_all(&client).await?,
        )),
        "PracticeNear" => Ok(DbObject::PracticeShort(
            PracticeShort::get_near(&client).await?,
        )),
        // "PracticeShort" =>
        "RankList" => Ok(DbObject::RankList(RankList::get_all(&client).await?)),
        "RankSelect" => Ok(DbObject::SelectItem(SelectItem::rank_all(&client).await?)),
        "ScopeList" => Ok(DbObject::ScopeList(ScopeList::get_all(&client).await?)),
        "ScopeSelect" => Ok(DbObject::SelectItem(SelectItem::scope_all(&client).await?)),
        // "SelectItem" =>
        "SirenList" => Ok(DbObject::SirenList(SirenList::get_all(&client).await?)),
        "SirenTypeList" => Ok(DbObject::SirenTypeList(
            SirenTypeList::get_all(&client).await?,
        )),
        "SirenTypeSelect" => Ok(DbObject::SelectItem(
            SelectItem::siren_type_all(&client).await?,
        )),
        "UserList" => Ok(DbObject::UserList(UserList::get_all(&client).await?)),
        e => Err(ServiceError::BadRequest(format!("bad list object: {}", e))),
    }
}

pub async fn insert_item(object: DbObject, client: &Client) -> Result<i64, ServiceError> {
    match object {
        DbObject::Certificate(item) => Ok(Certificate::insert(&client, item).await?.id),
        DbObject::Company(item) => Ok(Company::insert(&client, *item).await?.id),
        DbObject::Contact(item) => Ok(Contact::insert(&client, *item).await?.id),
        DbObject::Department(item) => Ok(Department::insert(&client, item).await?.id),
        DbObject::Education(item) => Ok(Education::insert(&client, item).await?.id),
        DbObject::Kind(item) => Ok(Kind::insert(&client, item).await?.id),
        DbObject::Post(item) => Ok(Post::insert(&client, item).await?.id),
        DbObject::Practice(item) => Ok(Practice::insert(&client, item).await?.id),
        DbObject::Rank(item) => Ok(Rank::insert(&client, item).await?.id),
        DbObject::Scope(item) => Ok(Scope::insert(&client, item).await?.id),
        DbObject::Siren(item) => Ok(Siren::insert(&client, *item).await?.id),
        DbObject::SirenType(item) => Ok(SirenType::insert(&client, item).await?.id),
        DbObject::User(item) => Ok(User::insert(&client, item).await?.id),
        _ => Err(ServiceError::BadRequest("bad item object".to_string())),
    }
}

pub async fn update_item(object: DbObject, client: &Client) -> Result<i64, ServiceError> {
    let res = match object {
        DbObject::Certificate(item) => Certificate::update(&client, item).await,
        DbObject::Company(item) => Company::update(&client, *item).await,
        DbObject::Contact(item) => Contact::update(&client, *item).await,
        DbObject::Department(item) => Department::update(&client, item).await,
        DbObject::Education(item) => Education::update(&client, item).await,
        DbObject::Kind(item) => Kind::update(&client, item).await,
        DbObject::Post(item) => Post::update(&client, item).await,
        DbObject::Practice(item) => Practice::update(&client, item).await,
        DbObject::Rank(item) => Rank::update(&client, item).await,
        DbObject::Scope(item) => Scope::update(&client, item).await,
        DbObject::Siren(item) => Siren::update(&client, *item).await,
        DbObject::SirenType(item) => SirenType::update(&client, item).await,
        DbObject::User(item) => User::update(&client, item).await,
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
