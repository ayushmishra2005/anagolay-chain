//! Tests for the module.

#![cfg(test)]
use super::mock::*;
use super::*;
use frame_support::{assert_noop, assert_ok};

const PERCEPTUAL_HASH: &[u8] = b"0x30303030303030303031313030303030303030303030303030303031313130303031313131313030313131313131313031313131313131313131313131313130303031313130303030303030303030303131313131313130303030303030303031313131313131313130303030303030313131313131313131313131313030303131313131313131313131313031313131313131313131313130313030313130313131303030303030303130303030303030303030303031303030303030303031313131313131313131313131313131313131313131313130303030303030303131313130303030303030303030303031313131303030303030303030303030";

pub fn build_default_proof(rule_id: Vec<u8>) -> Proof {
    let mut proof = Proof::default();
    proof.data.rule_id = rule_id;
    proof
}

#[test]
fn proof_create_default() {
    ExtBuilder::build().execute_with(|| {
        // todo create default rule, figure out better way
        // T::ExternalRulesStorage::put_rule_in_storage(4);
        // todo create default rule, figure out better way

        let r_id = b"bafk".to_vec();
        let pd = build_default_proof(r_id);
        let res = TestPoe::create_proof(Origin::signed(1), pd.clone());
        assert_ok!(res);
    });
}
#[test]
fn proof_error_on_duplicate() {
    ExtBuilder::build().execute_with(|| {
        // // todo create default rule, figure out better way
        // let r = create_default_rule();
        // let res = TestPoe::create_rule(Origin::signed(1), r.clone());
        // assert_ok!(res);
        // // todo create default rule, figure out better way

        // create the proof
        let r_id = b"bafk".to_vec();
        let pd = build_default_proof(r_id);
        let res1 = TestPoe::create_proof(Origin::signed(1), pd.clone());

        assert_ok!(res1);

        // create the proof AGAIN
        let res2 = TestPoe::create_proof(Origin::signed(1), pd.clone());

        assert_noop!(res2, Error::<Test>::ProofAlreadyClaimed);
    });
}

// #[test]
// fn proof_error_on_no_rule() {
//     ExtBuilder::build().execute_with(|| {
//         let rule_id = b"dummy-text-never-created".to_vec();
//         let pd = build_default_proof(rule_id);
//         let res = TestPoe::create_proof(Origin::signed(1), pd.clone());
//         assert_noop!(res, Error::<Test>::NoSuchRule);
//     });
// }
// #[test]
// fn test_test_phash() {
//   ExtBuilder::build().execute_with(|| {
//     // todo create default rule, figure out better way
//     let mut r = Rule::default();
//       //     r.for_what = ForWhat::Generic;
//     // r.ops = vec![
//     //   Operation {
//     //     op: b"meta_copyright".to_vec(),
//     //     name: b"".to_vec(),
//     //     desc: b"".to_vec(),
//     //     hash_algo: b"blake2b".to_vec(),
//     //     hash_bits: 256,
//     //     enc: b"hex".to_vec(),
//     //     prefix: b"0x".to_vec(),
//     //     ops: vec![],
//     //   },
//     //   Operation {
//     //     op: PERCEPTUAL_HASH_NAME.to_vec(),
//     //     name: b"".to_vec(),
//     //     desc: b"binary encoded, like 01110111".to_vec(),
//     //     hash_algo: b"blake2b".to_vec(),
//     //     hash_bits: 256,
//     //     enc: b"hex".to_vec(),
//     //     prefix: b"0x".to_vec(),
//     //     ops: vec![],
//     //   },
//     // ];
//     let res = TestPoe::create_rule(Origin::signed(1), r.clone());
//     assert_ok!(res);
//     let p_hash_name = b"perceptual_hash".to_vec();
//     // todo create default rule, figure out better way
//     // let phash_op = r.ops.iter().filter(|x| x.op = "perceptual_hash")
//     for (pos, e) in r.ops.iter().enumerate() {
//       if p_hash_name == e.op {
//         println!("Element at position {}: {:?}", pos, e.op);
//       }
//     }
//   });
// }
#[test]
fn phash_save_phash() {
    ExtBuilder::build().execute_with(|| {
        // // todo create default rule, figure out better way
        // let r = create_default_rule();
        // let res = TestPoe::create_rule(Origin::signed(1), r.clone());
        // assert_ok!(res);
        // // todo create default rule, figure out better way
        let r_id = b"bafk".to_vec();
        let pd = build_default_proof(r_id);
        let res = TestPoe::create_proof(Origin::signed(1), pd.clone());
        assert_ok!(res);

        let phash = PERCEPTUAL_HASH.clone().to_vec();
        let p_hash_payload = PhashInfo {
            p_hash: phash.clone(),
            proof_id: pd.id.clone(),
        };

        let res = TestPoe::save_phash(Origin::signed(1), p_hash_payload);
        assert_ok!(res);
    });
}
#[test]
fn phash_save_phash_error_on_duplicate() {
    ExtBuilder::build().execute_with(|| {
        // // todo create default rule, figure out better way
        // let r = create_default_rule();
        // let res = TestPoe::create_rule(Origin::signed(1), r.clone());
        // assert_ok!(res);
        // // todo create default rule, figure out better way
        let r_id = b"bafk".to_vec();
        let pd = build_default_proof(r_id);
        let res = TestPoe::create_proof(Origin::signed(1), pd.clone());
        assert_ok!(res);

        let phash = PERCEPTUAL_HASH.clone().to_vec();

        let p_hash_payload = PhashInfo {
            p_hash: phash.clone(),
            proof_id: pd.id.clone(),
        };

        let res = TestPoe::save_phash(Origin::signed(1), p_hash_payload.clone());
        assert_ok!(res);

        let res2 = TestPoe::save_phash(Origin::signed(1), p_hash_payload.clone());
        assert_noop!(res2, Error::<Test>::PHashAndProofIdComboAlreadyExist);
    });
}
#[test]
fn phash_save_phash_error_no_proof() {
    ExtBuilder::build().execute_with(|| {
  let phash = PERCEPTUAL_HASH.clone().to_vec();

  let proof_id = b"0x6261666b32627a616365616d6c6e766678726c717175743274686f6b6c6a76726b68726f7a787562696a78746f3476743566646f776c6162747733686177".to_vec();

  let p_hash_payload = PhashInfo {
    p_hash: phash.clone(),
    proof_id: proof_id.clone(),
  };

  let res = TestPoe::save_phash(Origin::signed(1), p_hash_payload);
  assert_noop!(res, Error::<Test>::NoSuchProof);
});
}
// #[test]
// fn test_cid() {
//   ExtBuilder::build().execute_with(|| {
//     // Creating multihash from the struct in substrate
//     // mainly testing the mh and cid variants
//     // interesting, here to_string works but in the sn_cid it doesn't
//     let op = Operation::default();
//     let s = op.encode();
//     let h = Blake2b256::digest(s.as_slice());

//     let cid = Cid::new(Version::V1, Codec::Raw, h).unwrap();
//     assert_eq!(
//       cid.to_string(),
//       "bafk2bzaceb3uveahln5rqipt55vraca7t4obmxwk7ewfklrri2ygep5dtxwnk"
//     )
//   });
// }
#[test]
fn test_template() {
    ExtBuilder::build().execute_with(|| {});
}
