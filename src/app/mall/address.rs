use crate::app::mall::{
    UserAddressDetailResponse, UserAddressListResponse, UserAddresseSaveRequest,
    UserAddresseUpdateRequest,
};
use crate::bootstrap::database::DatabasePool;
use crate::bootstrap::response::Response;
use crate::bootstrap::result;
use crate::middleware::authentication::MallIdentity;
use crate::models::user_address::NewUserAddress;
use crate::services;
use actix_web::web::{Data, Path};
use actix_web::{delete, get, post, put, web};

// 我的收货地址列表
#[get("")]
pub async fn list(pool: web::Data<DatabasePool>, identity: MallIdentity) -> result::Response {
    let conn = &mut pool.get()?;

    let list = services::user_address::list(conn, identity.user.user_id)?;

    let mut response: Vec<UserAddressListResponse> = vec![];

    for user_address in list {
        response.push(UserAddressListResponse {
            address_id: user_address.address_id,
            city_name: user_address.city_name,
            default_flag: user_address.default_flag,
            detail_address: user_address.detail_address,
            province_name: user_address.province_name,
            region_name: user_address.region_name,
            user_id: user_address.user_id,
            user_name: user_address.user_name,
            user_phone: user_address.user_phone,
        })
    }

    Response::success(response)
}

// 添加地址
#[post("")]
pub async fn save(
    pool: web::Data<DatabasePool>,
    web::Json(data): web::Json<UserAddresseSaveRequest>,
    identity: MallIdentity,
) -> result::Response {
    let conn = &mut pool.get()?;

    services::user_address::save(
        conn,
        NewUserAddress {
            user_id: identity.user.user_id,
            city_name: data.city_name,
            default_flag: data.default_flag,
            detail_address: data.detail_address,
            province_name: data.province_name,
            region_name: data.region_name,
            user_name: data.user_name,
            user_phone: data.user_phone,
        },
    )?;

    Response::success(())
}

// 修改地址
#[put("")]
pub async fn update(
    pool: web::Data<DatabasePool>,
    web::Json(data): web::Json<UserAddresseUpdateRequest>,
    identity: MallIdentity,
) -> result::Response {
    let conn = &mut pool.get()?;

    services::user_address::update(conn, identity.user.user_id, data)?;

    Response::success(())
}

// 获取收货地址详情
#[get("/{addressId}")]
pub async fn detail(pool: Data<DatabasePool>, address_id: Path<i64>) -> result::Response {
    let conn = &mut pool.get()?;

    let user_address = services::user_address::find(conn, address_id.into_inner())?;

    Response::success(UserAddressDetailResponse {
        address_id: user_address.address_id,
        city_name: user_address.city_name,
        default_flag: user_address.default_flag,
        detail_address: user_address.detail_address,
        province_name: user_address.province_name,
        region_name: user_address.region_name,
        user_id: user_address.user_id,
        user_name: user_address.user_name,
        user_phone: user_address.user_phone,
    })
}

// 获取默认收货地址
#[get("/default")]
pub async fn default(pool: Data<DatabasePool>, identity: MallIdentity) -> result::Response {
    let conn = &mut pool.get()?;

    match services::user_address::find_default(conn, identity.user.user_id)? {
        None => Response::success(()),
        Some(user_address) => Response::success(UserAddressDetailResponse {
            address_id: user_address.address_id,
            city_name: user_address.city_name,
            default_flag: user_address.default_flag,
            detail_address: user_address.detail_address,
            province_name: user_address.province_name,
            region_name: user_address.region_name,
            user_id: user_address.user_id,
            user_name: user_address.user_name,
            user_phone: user_address.user_phone,
        }),
    }
}

// 删除收货地址
#[delete("/{addressId}")]
pub async fn delete(pool: web::Data<DatabasePool>, address_id: Path<i64>) -> result::Response {
    let conn = &mut pool.get()?;

    services::user_address::delete(conn, address_id.into_inner())?;

    Response::success(())
}
