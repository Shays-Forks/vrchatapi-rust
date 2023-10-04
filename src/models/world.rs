/*
 * VRChat API Documentation
 *
 *
 * Contact: vrchatapi.lpv0t@aries.fyi
 * Generated by: https://openapi-generator.tech
 */

/// World : 



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct World {
    /// A users unique ID, usually in the form of `usr_c1644b5b-3ca4-45b4-97c6-a2a0de70d469`. Legacy players can have old IDs in the form of `8JoV9XEdpo`. The ID can never be changed.
    #[serde(rename = "authorId")]
    pub author_id: String,
    #[serde(rename = "authorName")]
    pub author_name: String,
    #[serde(rename = "capacity")]
    pub capacity: i32,
    #[serde(rename = "recommendedCapacity")]
    pub recommended_capacity: i32,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "favorites", skip_serializing_if = "Option::is_none")]
    pub favorites: Option<i32>,
    #[serde(rename = "featured")]
    pub featured: bool,
    #[serde(rename = "heat")]
    pub heat: i32,
    /// WorldID be \"offline\" on User profiles if you are not friends with that user.
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "imageUrl")]
    pub image_url: String,
    /// Will always be an empty list when unauthenticated.
    #[serde(rename = "instances", skip_serializing_if = "Option::is_none")]
    pub instances: Option<Vec<Vec<serde_json::Value>>>,
    #[serde(rename = "labsPublicationDate")]
    pub labs_publication_date: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "namespace")]
    pub namespace: String,
    /// Will always be `0` when unauthenticated.
    #[serde(rename = "occupants", skip_serializing_if = "Option::is_none")]
    pub occupants: Option<i32>,
    #[serde(rename = "organization")]
    pub organization: String,
    #[serde(rename = "popularity")]
    pub popularity: i32,
    #[serde(rename = "previewYoutubeId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub preview_youtube_id: Option<Option<String>>,
    /// Will always be `0` when unauthenticated.
    #[serde(rename = "privateOccupants", skip_serializing_if = "Option::is_none")]
    pub private_occupants: Option<i32>,
    /// Will always be `0` when unauthenticated.
    #[serde(rename = "publicOccupants", skip_serializing_if = "Option::is_none")]
    pub public_occupants: Option<i32>,
    #[serde(rename = "publicationDate")]
    pub publication_date: String,
    #[serde(rename = "releaseStatus")]
    pub release_status: crate::models::ReleaseStatus,
    ///  
    #[serde(rename = "tags")]
    pub tags: Vec<String>,
    #[serde(rename = "thumbnailImageUrl")]
    pub thumbnail_image_url: String,
    /// Empty if unauthenticated.
    #[serde(rename = "unityPackages", skip_serializing_if = "Option::is_none")]
    pub unity_packages: Option<Vec<crate::models::UnityPackage>>,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    #[serde(rename = "version")]
    pub version: i32,
    #[serde(rename = "visits")]
    pub visits: i32,
}

impl World {
    /// 
    pub fn new(author_id: String, author_name: String, capacity: i32, recommended_capacity: i32, created_at: String, description: String, featured: bool, heat: i32, id: String, image_url: String, labs_publication_date: String, name: String, namespace: String, organization: String, popularity: i32, publication_date: String, release_status: crate::models::ReleaseStatus, tags: Vec<String>, thumbnail_image_url: String, updated_at: String, version: i32, visits: i32) -> World {
        World {
            author_id,
            author_name,
            capacity,
            recommended_capacity,
            created_at,
            description,
            favorites: None,
            featured,
            heat,
            id,
            image_url,
            instances: None,
            labs_publication_date,
            name,
            namespace,
            occupants: None,
            organization,
            popularity,
            preview_youtube_id: None,
            private_occupants: None,
            public_occupants: None,
            publication_date,
            release_status,
            tags,
            thumbnail_image_url,
            unity_packages: None,
            updated_at,
            version,
            visits,
        }
    }
}


