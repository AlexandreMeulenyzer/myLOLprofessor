use keyring::Entry;
use thiserror::Error;

const SERVICE: &str = "com.wardstone.app";
const RIOT_API_KEY_ENTRY: &str = "riot_api_key";

#[derive(Debug, Error)]
pub enum SecureStorageError {
    #[error("erreur du trousseau systeme: {0}")]
    Keyring(#[from] keyring::Error),
}

/// Stockage de la cle API Riot personnelle de l'utilisateur dans le
/// trousseau securise du systeme d'exploitation (Windows Credential
/// Manager, Keychain macOS, Secret Service sur Linux). La cle n'est jamais
/// ecrite en clair sur le disque ni journalisee.
pub fn save_riot_api_key(api_key: &str) -> Result<(), SecureStorageError> {
    Entry::new(SERVICE, RIOT_API_KEY_ENTRY)?.set_password(api_key)?;
    Ok(())
}

pub fn get_riot_api_key() -> Result<Option<String>, SecureStorageError> {
    match Entry::new(SERVICE, RIOT_API_KEY_ENTRY)?.get_password() {
        Ok(password) => Ok(Some(password)),
        Err(keyring::Error::NoEntry) => Ok(None),
        Err(err) => Err(err.into()),
    }
}

pub fn delete_riot_api_key() -> Result<(), SecureStorageError> {
    match Entry::new(SERVICE, RIOT_API_KEY_ENTRY)?.delete_credential() {
        Ok(()) | Err(keyring::Error::NoEntry) => Ok(()),
        Err(err) => Err(err.into()),
    }
}
