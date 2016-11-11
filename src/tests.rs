
#![cfg(test)]

use crypto::sha3::Sha3;

use hashable::{ Hashable };
use merkletree::{ MerkleTree };
use merkledigest::{ MerkleDigest };


#[test]
fn test_from_str_vec() {
    let mut digest = Sha3::sha3_256();

    let values = vec![
                        "one",
                        "two",
                        "three",
                        "four"
                    ];

    let hashes = vec![
                        digest.hash_bytes(&values[0].to_bytes()),
                        digest.hash_bytes(&values[1].to_bytes()),
                        digest.hash_bytes(&values[2].to_bytes()),
                        digest.hash_bytes(&values[3].to_bytes())
                    ];

    let count  = values.len();

    let tree   = MerkleTree::from_vec(digest, values);

    let root_hash = Sha3::sha3_256().combine_hashes(
        &Sha3::sha3_256().combine_hashes(&hashes[0], &hashes[1]),
        &Sha3::sha3_256().combine_hashes(&hashes[2], &hashes[3]),
    );

    assert_eq!(tree.count, count);
    assert_eq!(tree.height, 2);
    assert_eq!(tree.root_hash().as_slice(), root_hash.as_slice());
}


#[test]
#[should_panic]
fn test_from_vec_empty() {
    let digest          = Sha3::sha3_256();
    let values: Vec<u8> = vec![];

    MerkleTree::from_vec(digest, values);
}

#[test]
fn test_from_vec1() {
    let digest = Sha3::sha3_256();

    let values = vec!["hello, world".to_string()];
    let tree = MerkleTree::from_vec(digest, values);

    let mut d = Sha3::sha3_256();

    let root_hash = &d.hash_bytes(&"hello, world".to_string().to_bytes());

    assert_eq!(tree.count, 1);
    assert_eq!(tree.height, 1);
    assert_eq!(tree.root_hash().as_slice(), root_hash.as_slice());
}


#[test]
fn test_from_vec3() {
    let digest = Sha3::sha3_256();

    let values = vec![1, 2, 3];
    let tree = MerkleTree::from_vec(digest, values);

    let mut d = Sha3::sha3_256();

    let hashes = vec![
        d.hash_bytes(&1.to_bytes()),
        d.hash_bytes(&2.to_bytes()),
        d.hash_bytes(&3.to_bytes())
    ];

    let h01       = &d.combine_hashes(&hashes[0], &hashes[1]);
    let h2        = &hashes[2];
    let root_hash = &d.combine_hashes(h01, h2);

    assert_eq!(tree.count, 3);
    assert_eq!(tree.height, 2);
    assert_eq!(tree.root_hash().as_slice(), root_hash.as_slice());
}

#[test]
fn test_from_vec9() {
    let digest = Sha3::sha3_256();

    let values = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let count  = values.len();

    let tree = MerkleTree::from_vec(digest, values.clone());

    let mut d = Sha3::sha3_256();

    let hashes = values.iter().map(|v| d.hash_bytes(&v.to_bytes())).collect::<Vec<_>>();

    let h01   = &d.combine_hashes(&hashes[0], &hashes[1]);
    let h23   = &d.combine_hashes(&hashes[2], &hashes[3]);
    let h45   = &d.combine_hashes(&hashes[4], &hashes[5]);
    let h67   = &d.combine_hashes(&hashes[6], &hashes[7]);
    let h8    = &hashes[8];
    let h0123 = &d.combine_hashes(h01, h23);
    let h4567 = &d.combine_hashes(h45, h67);
    let h1to7 = &d.combine_hashes(h0123, h4567);

    let root_hash = &d.combine_hashes(h1to7, h8);

    assert_eq!(tree.count, count);
    assert_eq!(tree.height, 4);
    assert_eq!(tree.root_hash().as_slice(), root_hash.as_slice());
}
