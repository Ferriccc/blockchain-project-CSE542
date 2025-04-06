use identity::Keypair;
use libp2p::PeerId;
use libp2p::identity;

#[derive(Debug, Clone)]
pub struct Node {
    pub id: String,
    pub public_key: Vec<u8>,
    pub private_key: Keypair,
}

impl Node {
    /// Creates a new NodeInfo instance with randomly generated Ed25519 keypair
    ///
    /// # Returns
    ///
    /// * `Result<NodeInfo, Box<dyn Error>>` - A Result containing either:
    ///   * `Ok(NodeInfo)` - A new NodeInfo instance with generated ID and keys
    ///   * `Err` - If there was an error encoding the private key
    pub fn new() -> Self {
        let keypair = identity::Keypair::generate_ed25519();
        let id = PeerId::from(keypair.public()).to_string();
        let public_key = keypair.public().encode_protobuf(); // Extract public key
        let private_key = keypair; // The keypair itself acts as the private key

        Self {
            id,
            public_key,
            private_key,
        }
    }
}
