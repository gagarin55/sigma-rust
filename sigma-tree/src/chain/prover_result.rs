//! ProverResult
use sigma_ser::serializer::SerializationError;
use sigma_ser::serializer::SigmaSerializable;
use sigma_ser::vlq_encode;
use std::io;

use super::context_extension::ContextExtension;
#[cfg(feature = "with-serde")]
use serde::{Deserialize, Serialize};

/// Proof of correctness of tx spending
#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "with-serde", derive(Serialize, Deserialize))]
pub struct ProverResult {
    /// proof that satisfies final sigma proposition
    pub proof: Vec<u8>,
    /// user-defined variables to be put into context
    pub extension: ContextExtension,
}

impl SigmaSerializable for ProverResult {
    fn sigma_serialize<W: vlq_encode::WriteSigmaVlqExt>(&self, w: &mut W) -> Result<(), io::Error> {
        w.put_u16(self.proof.len() as u16)?;
        w.write_all(&self.proof)?;
        self.extension.sigma_serialize(w)?;
        Ok(())
    }
    fn sigma_parse<R: vlq_encode::ReadSigmaVlqExt>(r: &mut R) -> Result<Self, SerializationError> {
        let proof_len = r.get_u16()?;
        let mut proof = vec![0; proof_len as usize];
        r.read_exact(&mut proof)?;
        let extension = ContextExtension::sigma_parse(r)?;
        Ok(ProverResult { proof, extension })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::{collection::vec, prelude::*};
    use sigma_ser::test_helpers::*;

    impl Arbitrary for ProverResult {
        type Parameters = ();

        fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
            (vec(any::<u8>(), 0..100))
                .prop_map(|v| Self {
                    proof: v,
                    extension: ContextExtension::empty(),
                })
                .boxed()
        }

        type Strategy = BoxedStrategy<Self>;
    }
    proptest! {

        #[test]
        fn ser_roundtrip(v in any::<ProverResult>()) {
            prop_assert_eq![sigma_serialize_roundtrip(&v), v];
        }
    }
}
