use ring::{
    aead::{Nonce, NonceSequence},
    rand::{SecureRandom, SystemRandom},
};
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};

#[derive(Debug, Deserialize, Serialize)]
pub struct EncryptedData {
    pub data: Vec<u8>,
    pub tag: [u8; 16],
    pub nonce: NonceGen,
}

#[derive(Debug, Default, Clone, Copy, Deserialize, Serialize)]
pub struct NonceGen([u8; 12]);

impl NonceGen {
    pub fn new() -> Self {
        let r = SystemRandom::new();
        let mut nonce = [0u8; 12];
        r.fill(&mut nonce).unwrap();
        NonceGen(nonce)
    }
}

impl NonceSequence for NonceGen {
    fn advance(&mut self) -> Result<Nonce, ring::error::Unspecified> {
        Nonce::try_assume_unique_for_key(&self.0)
    }
}

pub struct MerkleTree {
	pub MT: [String; 400001],
	pub indexToNode: [i32; 100001]
}

impl MerkleTree {
	pub fn initTree(&mut self, l: i32, r: i32, node: i32){ //initialize:a alltid med l=0, r=1e5, node=1
		if l == r {
			self.indexToNode[l as usize] = node;
			return;
		}
		let mid = (l+r)>>1;
		self.initTree(l, mid, 2*node);
		self.initTree(mid+1, r, 2*node+1);
	}
	
	fn hash_two_strings(s1: String, s2: String) -> String {
	    let concatenated = format!("{}{}", s1, s2);
	    let mut hasher = Sha256::new();
	    hasher.update(concatenated);
	    let result = hasher.finalize();
	    
	    format!("{:x}", result)
	}
	
	pub fn update(&mut self, index: i32, hashed_data: String){
		let mut node = self.indexToNode[index as usize];
		self.MT[node as usize] = hashed_data;
		node = node >> 1;
		while node > 0 {
			let a = 2*node;
			let b = 2*node+1;
			self.MT[node as usize] = Self::hash_two_strings(self.MT[a as usize].clone(), self.MT[b as usize].clone());
			node = node >> 1;
		}
	}
	
	pub fn top_hash(&self) -> String {
		return self.MT[1].clone();
	}
	
}