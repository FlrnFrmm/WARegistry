mod local_storage_provider;

trait StorageProvider {
    fn save(wa_reference: WebAssemblyReference, data: Vec<u8>) -> Result<()>;
    fn load(wa_reference: WebAssemblyReference) -> Vec<u8>;
}
