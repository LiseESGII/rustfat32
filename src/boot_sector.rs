#![no_std]

// Structure qui représente les champs principaux du Boot Sector FAT32.
pub struct BootSector {
    pub bytes_per_sector: u16,           // Nombre d'octets dans un secteur
    pub sectors_per_cluster: u8,         // Nombre de secteurs dans un cluster
    pub reserved_sectors: u16,           // Nombre de secteurs réservés au début du volume
    pub num_fats: u8,                    // Nombre de tables FAT
    pub sectors_per_fat: u32,            // Taille d'une FAT en secteurs
    pub root_dir_first_cluster: u32,     // Numéro du premier cluster du répertoire racine
}

impl BootSector {
    /// Parse un buffer de 512 octets en BootSector FAT32.
    /// Retourne None si le buffer est trop petit.
    pub fn from_bytes(data: &[u8]) -> Option<Self> {
        // Vérification de la taille du buffer (512 octets attendus pour un secteur)
        if data.len() < 512 {
            return None;
        }
        // D'après la doc FAT32, les offsets sont fixes :
        Some(Self {
            // Offset 11-12 : taille d'un secteur
            bytes_per_sector: u16::from_le_bytes([data[11], data[12]]),
            // Offset 13 : nombre de secteurs par cluster
            sectors_per_cluster: data[13],
            // Offset 14-15 : nombre de secteurs réservés
            reserved_sectors: u16::from_le_bytes([data[14], data[15]]),
            // Offset 16 : nombre de FATs
            num_fats: data[16],
            // Offset 36-39 : taille d'une FAT en secteurs
            sectors_per_fat: u32::from_le_bytes([data[36], data[37], data[38], data[39]]),
            // Offset 44-47 : premier cluster du répertoire racinee
            root_dir_first_cluster: u32::from_le_bytes([data[44], data[45], data[46], data[47]]),
        })
    }
}

/// Fonction de test pour BootSector::from_bytes.
/// Remplit un buffer avec des valeurs connues, puis vérifie que le parsing fonctionne.
pub fn test_boot_sector_parsing() {
    // On crée un buffer de 512 octets initialisé à zéro
    let mut data = [0u8; 512];

    // On place des valeurs arbitraires mais cohérentes dans les bons offsets :
    // 512 octets/secteur
    data[11] = 0x00;
    data[12] = 0x02;

    // 8 secteurs/cluster
    data[13] = 0x08;

    // 32 secteurs réservés
    data[14] = 0x20;
    data[15] = 0x00;

    // 2 FATs
    data[16] = 0x02;

    // 16 secteurs/FAT
    data[36] = 0x10;
    data[37] = 0x00;
    data[38] = 0x00;
    data[39] = 0x00;

    // Premier cluster du répertoire racine = 2
    data[44] = 0x02;
    data[45] = 0x00;
    data[46] = 0x00;
    data[47] = 0x00;

    // On tente de parser le buffer
    let bs = BootSector::from_bytes(&data).expect("Parsing du Boot Sector échoué");

    // Vérifications avec  des assert)
    assert!(bs.bytes_per_sector == 512, "bytes_per_sector incorrect");
    assert!(bs.sectors_per_cluster == 8, "sectors_per_cluster incorrect");
    assert!(bs.reserved_sectors == 32, "reserved_sectors incorrect");
    assert!(bs.num_fats == 2, "num_fats incorrect");
    assert!(bs.sectors_per_fat == 16, "sectors_per_fat incorrect");
    assert!(bs.root_dir_first_cluster == 2, "root_dir_first_cluster incorrect");

    // Si toutes les assertions passent, le parsing fonctionne.
    // Sinon, le kernel va boucler 
}
