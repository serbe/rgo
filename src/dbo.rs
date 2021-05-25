use std::fmt;

use rpel::{
    certificate::{Certificate, CertificateList},
    company::{Company, CompanyList},
    contact::{Contact, ContactList},
    department::{Department, DepartmentList},
    education::{Education, EducationList, EducationShort},
    kind::{Kind, KindList},
    post::{Post, PostList},
    practice::{Practice, PracticeList, PracticeShort},
    rank::{Rank, RankList},
    scope::{Scope, ScopeList},
    select::SelectItem,
    siren::{Siren, SirenList},
    siren_type::{SirenType, SirenTypeList},
    user::{User, UserList},
    RpelPool,
};
use serde::{Deserialize, Serialize};

use crate::error::ServiceError;
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

pub async fn get_item(item: &Item, pool: &RpelPool) -> Result<DBObject, ServiceError> {
    match (item.name.as_str(), item.id) {
        ("Certificate", id) => Ok(DBObject::Certificate(Certificate::get(pool, id).await?)),
        ("Company", id) => Ok(DBObject::Company(Box::new(Company::get(pool, id).await?))),
        ("Contact", id) => Ok(DBObject::Contact(Box::new(Contact::get(pool, id).await?))),
        ("Department", id) => Ok(DBObject::Department(Department::get(pool, id).await?)),
        ("Education", id) => Ok(DBObject::Education(Education::get(pool, id).await?)),
        ("Kind", id) => Ok(DBObject::Kind(Kind::get(pool, id).await?)),
        ("Post", id) => Ok(DBObject::Post(Post::get(pool, id).await?)),
        ("Practice", id) => Ok(DBObject::Practice(Practice::get(pool, id).await?)),
        ("Rank", id) => Ok(DBObject::Rank(Rank::get(pool, id).await?)),
        ("Scope", id) => Ok(DBObject::Scope(Scope::get(pool, id).await?)),
        ("Siren", id) => Ok(DBObject::Siren(Box::new(Siren::get(pool, id).await?))),
        ("SirenType", id) => Ok(DBObject::SirenType(SirenType::get(pool, id).await?)),
        ("User", id) => Ok(DBObject::User(User::get(pool, id).await?)),
        (e, id) => Err(ServiceError::BadRequest(format!(
            "bad item object: {} {}",
            e, id
        ))),
    }
}

pub async fn get_list(name: &str, pool: &RpelPool) -> Result<DBObject, ServiceError> {
    match name {
        "CertificateList" => Ok(DBObject::CertificateList(
            CertificateList::get_all(pool).await?,
        )),
        "CompanyList" => Ok(DBObject::CompanyList(CompanyList::get_all(pool).await?)),
        "CompanySelect" => Ok(DBObject::SelectItem(SelectItem::company_all(pool).await?)),
        "ContactList" => Ok(DBObject::ContactList(ContactList::get_all(pool).await?)),
        "ContactSelect" => Ok(DBObject::SelectItem(SelectItem::contact_all(pool).await?)),
        "DepartmentList" => Ok(DBObject::DepartmentList(
            DepartmentList::get_all(pool).await?,
        )),
        "DepartmentSelect" => Ok(DBObject::SelectItem(
            SelectItem::department_all(pool).await?,
        )),
        "EducationList" => Ok(DBObject::EducationList(EducationList::get_all(pool).await?)),
        "EducationNear" => Ok(DBObject::EducationShort(
            EducationShort::get_near(pool).await?,
        )),
        // "EducationShort" =>
        "KindList" => Ok(DBObject::KindList(KindList::get_all(pool).await?)),
        "KindSelect" => Ok(DBObject::SelectItem(SelectItem::kind_all(pool).await?)),
        "PostList" => Ok(DBObject::PostList(PostList::get_all(pool).await?)),
        "PostSelect" => Ok(DBObject::SelectItem(
            SelectItem::post_all(pool, false).await?,
        )),
        "PostGoSelect" => Ok(DBObject::SelectItem(
            SelectItem::post_all(pool, true).await?,
        )),
        "PracticeList" => Ok(DBObject::PracticeList(PracticeList::get_all(pool).await?)),
        "PracticeNear" => Ok(DBObject::PracticeShort(
            PracticeShort::get_near(pool).await?,
        )),
        // "PracticeShort" =>
        "RankList" => Ok(DBObject::RankList(RankList::get_all(pool).await?)),
        "RankSelect" => Ok(DBObject::SelectItem(SelectItem::rank_all(pool).await?)),
        "ScopeList" => Ok(DBObject::ScopeList(ScopeList::get_all(pool).await?)),
        "ScopeSelect" => Ok(DBObject::SelectItem(SelectItem::scope_all(pool).await?)),
        // "SelectItem" =>
        "SirenList" => Ok(DBObject::SirenList(SirenList::get_all(pool).await?)),
        "SirenTypeList" => Ok(DBObject::SirenTypeList(SirenTypeList::get_all(pool).await?)),
        "SirenTypeSelect" => Ok(DBObject::SelectItem(
            SelectItem::siren_type_all(pool).await?,
        )),
        "UserList" => Ok(DBObject::UserList(UserList::get_all(pool).await?)),
        e => Err(ServiceError::BadRequest(format!("bad list object: {}", e))),
    }
}

pub async fn insert_item(object: DBObject, pool: &RpelPool) -> Result<i64, ServiceError> {
    match object {
        DBObject::Certificate(item) => Ok(Certificate::insert(pool, item).await?.id),
        DBObject::Company(item) => Ok(Company::insert(pool, *item).await?.id),
        DBObject::Contact(item) => Ok(Contact::insert(pool, *item).await?.id),
        DBObject::Department(item) => Ok(Department::insert(pool, item).await?.id),
        DBObject::Education(item) => Ok(Education::insert(pool, item).await?.id),
        DBObject::Kind(item) => Ok(Kind::insert(pool, item).await?.id),
        DBObject::Post(item) => Ok(Post::insert(pool, item).await?.id),
        DBObject::Practice(item) => Ok(Practice::insert(pool, item).await?.id),
        DBObject::Rank(item) => Ok(Rank::insert(pool, item).await?.id),
        DBObject::Scope(item) => Ok(Scope::insert(pool, item).await?.id),
        DBObject::Siren(item) => Ok(Siren::insert(pool, *item).await?.id),
        DBObject::SirenType(item) => Ok(SirenType::insert(pool, item).await?.id),
        DBObject::User(item) => Ok(User::insert(pool, item).await?.id),
        _ => Err(ServiceError::BadRequest("bad item object".to_string())),
    }
}

pub async fn update_item(object: DBObject, pool: &RpelPool) -> Result<i64, ServiceError> {
    let res = match object {
        DBObject::Certificate(item) => Certificate::update(pool, item).await,
        DBObject::Company(item) => Company::update(pool, *item).await,
        DBObject::Contact(item) => Contact::update(pool, *item).await,
        DBObject::Department(item) => Department::update(pool, item).await,
        DBObject::Education(item) => Education::update(pool, item).await,
        DBObject::Kind(item) => Kind::update(pool, item).await,
        DBObject::Post(item) => Post::update(pool, item).await,
        DBObject::Practice(item) => Practice::update(pool, item).await,
        DBObject::Rank(item) => Rank::update(pool, item).await,
        DBObject::Scope(item) => Scope::update(pool, item).await,
        DBObject::Siren(item) => Siren::update(pool, *item).await,
        DBObject::SirenType(item) => SirenType::update(pool, item).await,
        DBObject::User(item) => User::update(pool, item).await,
        _ => return Err(ServiceError::BadRequest("bad item object".to_string())),
    }?;
    Ok(res as i64)
}

pub async fn delete_item(item: &Item, pool: &RpelPool) -> Result<i64, ServiceError> {
    let res = match item.name.as_str() {
        "Certificate" => Certificate::delete(pool, item.id).await,
        "Company" => Company::delete(pool, item.id).await,
        "Contact" => Contact::delete(pool, item.id).await,
        "Department" => Department::delete(pool, item.id).await,
        "Education" => Education::delete(pool, item.id).await,
        "Kind" => Kind::delete(pool, item.id).await,
        "Post" => Post::delete(pool, item.id).await,
        "Practice" => Practice::delete(pool, item.id).await,
        "Rank" => Rank::delete(pool, item.id).await,
        "Scope" => Scope::delete(pool, item.id).await,
        "Siren" => Siren::delete(pool, item.id).await,
        "Siren_type" => SirenType::delete(pool, item.id).await,
        "User" => User::delete(pool, item.id).await,
        _ => {
            return Err(ServiceError::BadRequest(format!(
                "bad path {:?}",
                item.name
            )))
        }
    }?;
    Ok(res as i64)
}
