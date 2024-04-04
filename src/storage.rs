use hard_xml::{XmlRead, XmlWrite};
use std::borrow::Cow;

#[derive(XmlWrite, XmlRead, PartialEq, Eq, Debug, Hash, Clone)]
#[xml(tag = "pool")]
pub struct StoragePool<'a> {
    #[xml(attr = "type")]
    r#type: Cow<'a, str>,
    #[xml(flatten_text = "name")]
    name: Option<Cow<'a, str>>,
    #[xml(flatten_text = "uuid")]
    uuid: Option<uuid::Uuid>,
    #[xml(child = "allocation")]
    allocation: Option<StoragePoolSize<'a>>,
    #[xml(child = "capacity")]
    capacity: Option<StoragePoolSize<'a>>,
    #[xml(child = "available")]
    available: Option<StoragePoolSize<'a>>,
    #[xml(child = "features")]
    features: Option<StoragePoolFeatures<'a>>,
    #[xml(child = "target")]
    target: Option<StoragePoolTarget<'a>>,
    #[xml(child = "source")]
    source: Option<StoragePoolSource<'a>>,
    #[xml(child = "refresh")]
    refresh : Option<StoragePoolRefresh<'a>>,
}

#[derive(XmlWrite, XmlRead, PartialEq, Eq, Debug, Hash, Clone)]
#[xml(tag = "")]
pub struct StoragePoolSize<'a> {
    #[xml(attr = "unit")]
    unit: Option<Cow<'a, str>>,
    #[xml(text)]
    value: u64,
}

#[derive(XmlWrite, XmlRead, PartialEq, Eq, Debug, Hash, Clone)]
#[xml(tag = "features")]
pub struct StoragePoolFeatures<'a> {
    #[xml(child = "cow")]
    cow: Option<StoragePoolFeatureCow<'a>>,
}

#[derive(XmlWrite, XmlRead, PartialEq, Eq, Debug, Hash, Clone)]
#[xml(tag = "cow")]
pub struct StoragePoolFeatureCow<'a> {
    #[xml(attr = "state")]
    state: Cow<'a, str>,
}

#[derive(XmlWrite, XmlRead, PartialEq, Eq, Debug, Hash, Clone)]
#[xml(tag = "target")]
pub struct StoragePoolTarget<'a> {
    #[xml(flatten_text = "path")]
    path: Cow<'a, str>,
    #[xml(child = "permissions")]
    permissions: Option<StoragePoolTargetPermissions<'a>>,
}

#[derive(XmlWrite, XmlRead, PartialEq, Eq, Debug, Hash, Clone)]
#[xml(tag = "permissions")]
pub struct StoragePoolTargetPermissions<'a> {
    #[xml(flatten_text = "owner")]
    owner: Option<u16>,
    #[xml(flatten_text = "group")]
    group: Option<u16>,
    #[xml(flatten_text = "mode")]
    mode: Option<u8>,
    #[xml(flatten_text = "label")]
    label: Option<Cow<'a, str>>,
}

#[derive(XmlWrite, XmlRead, PartialEq, Eq, Debug, Hash, Clone)]
#[xml(tag = "source")]
pub struct StoragePoolSource<'a> {
    #[xml(flatten_text = "name")]
    name: Option<Cow<'a, str>>,
    #[xml(child = "dir")]
    dir: Option<StoragePoolSourceDir<'a>>,
    #[xml(child = "host")]
    host: Option<StoragePoolSourceHost<'a>>,
    #[xml(child = "device")]
    device: Option<StoragePoolSourceDevice<'a>>,
    #[xml(child = "auth")]
    auth: Option<StoragePoolSourceAuth<'a>>,
    #[xml(child = "vendor")]
    vendor: Option<StoragePoolSourceVendor<'a>>,
    #[xml(child = "product")]
    product: Option<StoragePoolSourceProduct<'a>>,
    #[xml(child = "format")]
    format: Option<StoragePoolSourceFormat<'a>>,
    #[xml(child = "protocol")]
    protocol: Option<StoragePoolSourceProtocol<'a>>,
    #[xml(child = "adapter")]
    adapter: Option<StoragePoolSourceAdapter<'a>>,
    #[xml(child = "initiator")]
    initiator: Option<StoragePoolSourceInitiator<'a>>,
}

#[derive(XmlWrite, XmlRead, PartialEq, Eq, Debug, Hash, Clone)]
#[xml(tag = "dir")]
pub struct StoragePoolSourceDir<'a> {
    #[xml(attr = "path")]
    path: Cow<'a, str>,
}

#[derive(XmlWrite, XmlRead, PartialEq, Eq, Debug, Hash, Clone)]
#[xml(tag = "host")]
pub struct StoragePoolSourceHost<'a> {
    #[xml(attr = "name")]
    name: Cow<'a, str>,
    #[xml(attr = "port")]
    port: Option<u16>,
}

#[derive(XmlWrite, XmlRead, PartialEq, Eq, Debug, Hash, Clone)]
#[xml(tag = "device")]
pub struct StoragePoolSourceDevice<'a> {
    #[xml(attr = "path")]
    path: Cow<'a, str>,
    #[xml(attr = "part_separator")]
    part_separator: Option<Cow<'a, str>>,
    #[xml(child = "freeExtents")]
    free_extent: Vec<StoragePoolSourceDeviceFreeExtent>,
}

#[derive(XmlWrite, XmlRead, PartialEq, Eq, Debug, Hash, Clone)]
#[xml(tag = "freeExtent")]
pub struct StoragePoolSourceDeviceFreeExtent {
    #[xml(attr = "start")]
    start: u64,
    #[xml(attr = "end")]
    end: u64,
}

#[derive(XmlWrite, XmlRead, PartialEq, Eq, Debug, Hash, Clone)]
#[xml(tag = "auth")]
pub struct StoragePoolSourceAuth<'a> {
    #[xml(attr = "type")]
    r#type: Cow<'a, str>,
    #[xml(attr = "username")]
    username: Cow<'a, str>,
    #[xml(child = "secret")]
    secret: StoragePoolSourceAuthSecret<'a>,
}

#[derive(XmlWrite, XmlRead, PartialEq, Eq, Debug, Hash, Clone)]
#[xml(tag = "secret")]
pub struct StoragePoolSourceAuthSecret<'a> {
    #[xml(attr = "usage")]
    usage: Option<Cow<'a, str>>,
    #[xml(attr = "uuid")]
    uuid: Option<uuid::Uuid>,
}

#[derive(XmlWrite, XmlRead, PartialEq, Eq, Debug, Hash, Clone)]
#[xml(tag = "vendor")]
pub struct StoragePoolSourceVendor<'a> {
    #[xml(attr = "name")]
    name: Cow<'a, str>,
}

#[derive(XmlWrite, XmlRead, PartialEq, Eq, Debug, Hash, Clone)]
#[xml(tag = "product")]
pub struct StoragePoolSourceProduct<'a> {
    #[xml(attr = "name")]
    name: Cow<'a, str>,
}

#[derive(XmlWrite, XmlRead, PartialEq, Eq, Debug, Hash, Clone)]
#[xml(tag = "format")]
pub struct StoragePoolSourceFormat<'a> {
    #[xml(attr = "type")]
    r#type: Cow<'a, str>,
}

#[derive(XmlWrite, XmlRead, PartialEq, Eq, Debug, Hash, Clone)]
#[xml(tag = "protocol")]
pub struct StoragePoolSourceProtocol<'a> {
    #[xml(attr = "ver")]
    version: Cow<'a, str>,
}

#[derive(XmlWrite, XmlRead, PartialEq, Eq, Debug, Hash, Clone)]
#[xml(tag = "adapter")]
pub struct StoragePoolSourceAdapter<'a> {
    #[xml(attr = "type")]
    r#type: Option<Cow<'a, str>>,
    #[xml(attr = "name")]
    name: Option<Cow<'a, str>>,
    #[xml(attr = "parent")]
    parent: Option<Cow<'a, str>>,
    #[xml(attr = "managed")]
    managed: Option<Cow<'a, str>>,
    #[xml(attr = "wwnn")]
    wwnn: Option<Cow<'a, str>>,
    #[xml(attr = "wwpn")]
    wwpn: Option<Cow<'a, str>>,
    #[xml(child = "parentaddr")]
    parent_addr: Option<StoragePoolSourceAdapterParentAddr>,
}
#[derive(XmlWrite, XmlRead, PartialEq, Eq, Debug, Hash, Clone)]
#[xml(tag = "parentaddr")]
pub struct StoragePoolSourceAdapterParentAddr {
    #[xml(attr = "unique_id")]
    unique_id: u64,
    #[xml(child = "address")]
    address: Option<StoragePoolSourceAdapterParentAddrPCIAddress>,
}
#[derive(XmlWrite, XmlRead, PartialEq, Eq, Debug, Hash, Clone)]
#[xml(tag = "address")]
pub struct StoragePoolSourceAdapterParentAddrPCIAddress {
    #[xml(attr = "domain")]
    domain: Option<u16>,
    #[xml(attr = "bus")]
    bus: Option<u8>,
    #[xml(attr = "slot")]
    slot: Option<u8>,
    #[xml(attr = "function")]
    function: Option<u8>,
}

#[derive(XmlWrite, XmlRead, PartialEq, Eq, Debug, Hash, Clone)]
#[xml(tag = "initiator")]
pub struct StoragePoolSourceInitiator<'a> {
    #[xml(child = "iqn")]
    iqn: StoragePoolSourceInitiatorIQN<'a>,
}

#[derive(XmlWrite, XmlRead, PartialEq, Eq, Debug, Hash, Clone)]
#[xml(tag = "iqn")]
pub struct StoragePoolSourceInitiatorIQN<'a> {
    #[xml(attr = "name")]
    name: Option<Cow<'a, str>>,
}

#[derive(XmlWrite, XmlRead, PartialEq, Eq, Debug, Hash, Clone)]
#[xml(tag = "refresh")]
pub struct StoragePoolRefresh<'a> {
    #[xml(child="volume")]
    volume : StoragePoolRefreshVol<'a>,
}

#[derive(XmlWrite, XmlRead, PartialEq, Eq, Debug, Hash, Clone)]
#[xml(tag="volume")]
pub struct StoragePoolRefreshVol<'a> {
    #[xml(attr="allocation")]
    allocation : Cow<'a, str>,
}


