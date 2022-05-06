use cosmwasm_std::{Binary, HumanAddr};
use secret_storage_plus::{Item, Map};

const FORWARDING_ADDR: Item<(HumanAddr, String)> = Item::new("forwarding_addr");

pub fn forwarding_addr() -> &'static Item<'static, (HumanAddr, String)> {
    &FORWARDING_ADDR
}

const GENERAL_STORAGE: Map<&[u8], Binary> = Map::new("general_storage");

pub fn general_storage<'a>() -> &'static Map<'static, &'a [u8], Binary> {
    &GENERAL_STORAGE
}
