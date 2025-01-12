impl SettlementTypeMetadata {
    pub fn is_empty(&self) -> bool {
        // Add your empty check logic here
        self.instrument_index.is_none()
    }
} 