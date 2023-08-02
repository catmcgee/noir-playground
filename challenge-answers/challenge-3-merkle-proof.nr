use dep::std;

fn main(
    root: pub Field,          // the root of the Merkle tree
    account_id: pub Field,    // the unique ID of the account
    index: Field,             // The index path of the account in the tree
    hash_path_1: Field, 
    hash_path_2: Field,
    commitment: Field         // the commitment of this account in the tree
) -> pub Field {

    // Compute the id_commitment with the hash of the account_id
    let id_commitment = std::hash::pedersen([account_id])[0];

    // Assert the id_commitment is equal to the provided commitment
    assert(id_commitment == commitment);
    let hash_path = [hash_path_1, hash_path_2];
    // Prove the commitment is included in the liabilities Merkle tree
    let computed_root = std::merkle::compute_merkle_root(commitment, index, hash_path);
    assert(root == computed_root);

    computed_root
}

#[test]
fn test_build_merkle_tree() {
    let user1_id = 1;
    let user2_id = 2;
    let user3_id = 3;
    let user4_id = 4;

    // Calculate each user's id_commitment
    let commitment1 =  std::hash::pedersen([user1_id])[0];
    let commitment2 =  std::hash::pedersen([user2_id])[0];
    let commitment3 =  std::hash::pedersen([user3_id])[0];
    let commitment4 =  std::hash::pedersen([user4_id])[0];
  
    // Combined commitments for the next layer of the Merkle tree
    let left_branch = std::hash::pedersen([commitment1, commitment2])[0];
    let right_branch = std::hash::pedersen([commitment3, commitment4])[0];

    // The root of our Merkle tree
    let root = std::hash::pedersen([left_branch, right_branch])[0];

    std::println("Merkle Tree:");
    std::println([root]);
    std::println([left_branch, right_branch]);
    std::println([commitment1, commitment2, commitment3, commitment4]);
}
