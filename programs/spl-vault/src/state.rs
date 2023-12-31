use anchor_lang::prelude::*;

#[account]
pub struct Vault {
    pub name: String,

    pub authority: Pubkey,

    pub last_fee_collection: u64,

    pub fee: u64,

    pub fee_assets: Vec<Asset>,

    pub bump: u8,
}

impl Vault {
    pub const LEN: usize = std::mem::size_of::<Vault>() + std::mem::size_of::<Asset>() * 50;
}

#[account]
pub struct User {
    pub vault: Pubkey,

    pub key: Pubkey,

    pub assets: Vec<Asset>,

    pub bump: u8,
}

impl User {
    pub const LEN: usize = std::mem::size_of::<User>() + std::mem::size_of::<Asset>() * 50;
}

#[derive(Debug, AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, Copy)]
pub struct Asset {
    pub mint: Pubkey,

    pub amount: u64,
}
