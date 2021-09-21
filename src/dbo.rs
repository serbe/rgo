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
use crate::messages::Item;

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

pub async fn get_item(item: &Item, pool: &RpelPool) -> Result<DbObject, ServiceError> {
    match (item.name.as_str(), item.id) {
        ("Certificate", id) => Ok(DbObject::Certificate(Certificate::get(pool, id).await?)),
        ("Company", id) => Ok(DbObject::Company(Box::new(Company::get(pool, id).await?))),
        ("Contact", id) => Ok(DbObject::Contact(Box::new(Contact::get(pool, id).await?))),
        ("Department", id) => Ok(DbObject::Department(Department::get(pool, id).await?)),
        ("Education", id) => Ok(DbObject::Education(Education::get(pool, id).await?)),
        ("Kind", id) => Ok(DbObject::Kind(Kind::get(pool, id).await?)),
        ("Post", id) => Ok(DbObject::Post(Post::get(pool, id).await?)),
        ("Practice", id) => Ok(DbObject::Practice(Practice::get(pool, id).await?)),
        ("Rank", id) => Ok(DbObject::Rank(Rank::get(pool, id).await?)),
        ("Scope", id) => Ok(DbObject::Scope(Scope::get(pool, id).await?)),
        ("Siren", id) => Ok(DbObject::Siren(Box::new(Siren::get(pool, id).await?))),
        ("SirenType", id) => Ok(DbObject::SirenType(SirenType::get(pool, id).await?)),
        ("User", id) => Ok(DbObject::User(User::get(pool, id).await?)),
        (e, id) => Err(ServiceError::BadRequest(format!(
            "bad item object: {} {}",
            e, id
        ))),
    }
}

pub async fn get_list(name: &str, pool: &RpelPool) -> Result<DbObject, ServiceError> {
    match name {
        "CertificateList" => Ok(DbObject::CertificateList(
            CertificateList::get_all(pool).await?,
        )),
        "CompanyList" => Ok(DbObject::CompanyList(CompanyList::get_all(pool).await?)),
        "CompanySelect" => Ok(DbObject::SelectItem(SelectItem::company_all(pool).await?)),
        "ContactList" => Ok(DbObject::ContactList(ContactList::get_all(pool).await?)),
        "ContactSelect" => Ok(DbObject::SelectItem(SelectItem::contact_all(pool).await?)),
        "DepartmentList" => Ok(DbObject::DepartmentList(
            DepartmentList::get_all(pool).await?,
        )),
        "DepartmentSelect" => Ok(DbObject::SelectItem(
            SelectItem::department_all(pool).await?,
        )),
        "EducationList" => Ok(DbObject::EducationList(EducationList::get_all(pool).await?)),
        "EducationNear" => Ok(DbObject::EducationShort(
            EducationShort::get_near(pool).await?,
        )),
        // "EducationShort" =>
        "KindList" => Ok(DbObject::KindList(KindList::get_all(pool).await?)),
        "KindSelect" => Ok(DbObject::SelectItem(SelectItem::kind_all(pool).await?)),
        "PostList" => Ok(DbObject::PostList(PostList::get_all(pool).await?)),
        "PostSelect" => Ok(DbObject::SelectItem(
            SelectItem::post_all(pool, false).await?,
        )),
        "PostGoSelect" => Ok(DbObject::SelectItem(
            SelectItem::post_all(pool, true).await?,
        )),
        "PracticeList" => Ok(DbObject::PracticeList(PracticeList::get_all(pool).await?)),
        "PracticeNear" => Ok(DbObject::PracticeShort(
            PracticeShort::get_near(pool).await?,
        )),
        // "PracticeShort" =>
        "RankList" => Ok(DbObject::RankList(RankList::get_all(pool).await?)),
        "RankSelect" => Ok(DbObject::SelectItem(SelectItem::rank_all(pool).await?)),
        "ScopeList" => Ok(DbObject::ScopeList(ScopeList::get_all(pool).await?)),
        "ScopeSelect" => Ok(DbObject::SelectItem(SelectItem::scope_all(pool).await?)),
        // "SelectItem" =>
        "SirenList" => Ok(DbObject::SirenList(SirenList::get_all(pool).await?)),
        "SirenTypeList" => Ok(DbObject::SirenTypeList(SirenTypeList::get_all(pool).await?)),
        "SirenTypeSelect" => Ok(DbObject::SelectItem(
            SelectItem::siren_type_all(pool).await?,
        )),
        "UserList" => Ok(DbObject::UserList(UserList::get_all(pool).await?)),
        e => Err(ServiceError::BadRequest(format!("bad list object: {}", e))),
    }
}

pub async fn insert_item(object: DbObject, pool: &RpelPool) -> Result<i64, ServiceError> {
    match object {
        DbObject::Certificate(item) => Ok(Certificate::insert(pool, item).await?.id),
        DbObject::Company(item) => Ok(Company::insert(pool, *item).await?.id),
        DbObject::Contact(item) => Ok(Contact::insert(pool, *item).await?.id),
        DbObject::Department(item) => Ok(Department::insert(pool, item).await?.id),
        DbObject::Education(item) => Ok(Education::insert(pool, item).await?.id),
        DbObject::Kind(item) => Ok(Kind::insert(pool, item).await?.id),
        DbObject::Post(item) => Ok(Post::insert(pool, item).await?.id),
        DbObject::Practice(item) => Ok(Practice::insert(pool, item).await?.id),
        DbObject::Rank(item) => Ok(Rank::insert(pool, item).await?.id),
        DbObject::Scope(item) => Ok(Scope::insert(pool, item).await?.id),
        DbObject::Siren(item) => Ok(Siren::insert(pool, *item).await?.id),
        DbObject::SirenType(item) => Ok(SirenType::insert(pool, item).await?.id),
        DbObject::User(item) => Ok(User::insert(pool, item).await?.id),
        _ => Err(ServiceError::BadRequest("bad item object".to_string())),
    }
}

pub async fn update_item(object: DbObject, pool: &RpelPool) -> Result<i64, ServiceError> {
    let res = match object {
        DbObject::Certificate(item) => Certificate::update(pool, item).await,
        DbObject::Company(item) => Company::update(pool, *item).await,
        DbObject::Contact(item) => Contact::update(pool, *item).await,
        DbObject::Department(item) => Department::update(pool, item).await,
        DbObject::Education(item) => Education::update(pool, item).await,
        DbObject::Kind(item) => Kind::update(pool, item).await,
        DbObject::Post(item) => Post::update(pool, item).await,
        DbObject::Practice(item) => Practice::update(pool, item).await,
        DbObject::Rank(item) => Rank::update(pool, item).await,
        DbObject::Scope(item) => Scope::update(pool, item).await,
        DbObject::Siren(item) => Siren::update(pool, *item).await,
        DbObject::SirenType(item) => SirenType::update(pool, item).await,
        DbObject::User(item) => User::update(pool, item).await,
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
