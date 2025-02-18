// src/schnorr.rs
use num_bigint::BigUint;
use rand::Rng;

/// Represents the public parameters for the Schnorr protocol
#[derive(Clone)]
pub struct Parameters {
    /// Generator of the group
    g: BigUint,
    /// Prime modulus defining the field
    p: BigUint,
}

/// Represents a key pair in the Schnorr protocol
pub struct KeyPair {
    /// Secret key (discrete logarithm)
    secret: BigUint,
    /// Public key (y = g^secret mod p)
    public: BigUint,
}

impl Parameters {
    pub fn new(g: BigUint, p: BigUint) -> Self {
        Self { g, p }
    }
}

impl KeyPair {
    /// Generate a new key pair using the given parameters
    pub fn generate(params: &Parameters) -> Self {
        let mut rng = rand::rng();
        
        // Generate random secret key using modular arithmetic
        let secret = BigUint::from_bytes_be(&rng.random::<[u8; 32]>()) % (&params.p - BigUint::from(1u32)) + BigUint::from(1u32);
        
        // Compute public key: y = g^secret mod p
        let public = params.g.modpow(&secret, &params.p);
        
        Self { secret, public }
    }
}

/// Represents the Schnorr protocol instance
pub struct SchnorrProtocol {
    params: Parameters,
}

impl SchnorrProtocol {
    pub fn new(params: Parameters) -> Self {
        Self { params }
    }

    /// Prover's first move: generates commitment a = g^r
    pub fn prover_commit(&self) -> (BigUint, BigUint) {
        let mut rng = rand::rng();
        
        // Generate random nonce r using modular arithmetic
        let r = BigUint::from_bytes_be(&rng.random::<[u8; 32]>()) % (&self.params.p - BigUint::from(1u32)) + BigUint::from(1u32);
        
        // Compute commitment a = g^r mod p
        let a = self.params.g.modpow(&r, &self.params.p);
        
        (r, a)
    }

    /// Prover's response to the challenge: z = r + u*e
    pub fn prover_respond(&self, r: &BigUint, secret: &BigUint, e: &BigUint) -> BigUint {
        // z = r + u*e mod (p-1)
        (r + secret * e) % (&self.params.p - BigUint::from(1u32))
    }

    /// Verifier checks if g^z = a * y^e
    pub fn verify(&self, a: &BigUint, e: &BigUint, z: &BigUint, public_key: &BigUint) -> bool {
        // Compute left side: g^z mod p
        let lhs = self.params.g.modpow(z, &self.params.p);
        
        // Compute right side: a * y^e mod p
        let y_to_e = public_key.modpow(e, &self.params.p);
        let rhs = (a * y_to_e) % &self.params.p;
        
        lhs == rhs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_schnorr_protocol() {
        // Simple test parameters (not secure, just for demonstration)
        let params = Parameters::new(
            BigUint::from(4u32),  // generator
            BigUint::from(23u32), // prime modulus
        );

        let protocol = SchnorrProtocol::new(params.clone());
        let keypair = KeyPair::generate(&params);

        // Prover generates commitment
        let (r, a) = protocol.prover_commit();

        // Verifier generates challenge (in real protocol, this would come from verifier)
        let e = BigUint::from(1u32);

        // Prover generates response
        let z = protocol.prover_respond(&r, &keypair.secret, &e);

        // Verifier checks the proof
        assert!(protocol.verify(&a, &e, &z, &keypair.public));
    }
}