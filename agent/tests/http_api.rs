use serde::Deserialize;
use subxt::utils::AccountId32;

const HOST: &str = "http://127.0.0.1:9494";

#[derive(Deserialize)]
pub struct Entity {
    pub account_id: String,
    pub created_at: u32,
}

#[derive(Deserialize)]
pub struct SubEntity {
    pub entity: String,
    pub account_id: String,
    pub location: String,
    pub created_at: u32,
}

#[derive(Deserialize)]
pub struct Spike {
    pub sub_entity: String,
    pub value: String,
    pub created_at: u32,
}

#[derive(Deserialize)]
pub struct TooManySpike {
    pub sub_entity: String,
    pub created_at: u32,
}

#[derive(Deserialize)]
pub struct ReadyCertificate {
    pub sub_entity: String,
    pub created_at: u32,
}

pub async fn request_entities() -> Vec<Entity> {
    reqwest::get(format!("{HOST}/entities")).await.unwrap().json::<Vec<Entity>>().await.unwrap()
}

pub async fn request_sub_entities(entity: &AccountId32) -> Vec<SubEntity> {
    reqwest::get(format!("{HOST}/sub-entities?account_id={entity}"))
        .await
        .unwrap()
        .json::<Vec<SubEntity>>()
        .await
        .unwrap()
}

pub async fn request_spikes(sub_entity: &AccountId32) -> Vec<Spike> {
    reqwest::get(format!("{HOST}/spikes?account_id={sub_entity}"))
        .await
        .unwrap()
        .json::<Vec<Spike>>()
        .await
        .unwrap()
}

pub async fn request_too_many_spikes(sub_entity: &AccountId32) -> Vec<TooManySpike> {
    reqwest::get(format!("{HOST}/too-many-spikes?account_id={sub_entity}"))
        .await
        .unwrap()
        .json::<Vec<TooManySpike>>()
        .await
        .unwrap()
}

pub async fn request_ready_certificates(sub_entity: &AccountId32) -> Vec<ReadyCertificate> {
    reqwest::get(format!("{HOST}/ready-certificates?account_id={sub_entity}"))
        .await
        .unwrap()
        .json::<Vec<ReadyCertificate>>()
        .await
        .unwrap()
}
