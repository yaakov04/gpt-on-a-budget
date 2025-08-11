// Sigue siendo necesario para mantener la firma de la función
use std::path::PathBuf;

// Nuevas dependencias
use keyring::Entry;

// Constantes para identificar la entrada en el gestor de credenciales
const SERVICE_NAME: &str = "gpt-on-a-budget";
const KEY_USERNAME: &str = "openai_api_key";

/// Guarda la API key en el gestor de credenciales del sistema.
/// La firma de la función es idéntica a la original.
pub fn encrypt_and_save_api_key(
    _app_data_dir: PathBuf, // Se ignora, pero se mantiene por compatibilidad
    api_key: &str,
) -> Result<(), String> {
    let entry = Entry::new(SERVICE_NAME, KEY_USERNAME)
        .map_err(|e| e.to_string())?;

    entry.set_password(api_key)
        .map_err(|e| e.to_string())?;

    Ok(())
}

/// Recupera la API key desde el gestor de credenciales del sistema.
/// La firma de la función es idéntica a la original.
pub fn decrypt_api_key(
    _app_data_dir: PathBuf, // Se ignora, pero se mantiene por compatibilidad
) -> Result<String, String> {
    let entry = Entry::new(SERVICE_NAME, KEY_USERNAME)
        .map_err(|e| e.to_string())?;

    match entry.get_password() {
        Ok(password) => Ok(password),
        Err(keyring::Error::NoEntry) => Err("No se encontró la API key.".to_string()),
        Err(e) => Err(e.to_string()),
    }
}
