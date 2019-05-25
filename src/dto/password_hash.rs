
pub struct PasswordHash {
    user_id: i32,
    hash: [u8; 32],
    salt: [u8; 16]
}