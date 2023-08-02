use std::sync::Arc;

pub struct Challenge {
    pub id: u32,
    pub description: String,
    pub test_cases: Vec<String>,
}

pub fn get_challenges() -> Arc<Vec<Challenge>> {
    Arc::new(vec![
        Challenge {
            id: 1,
            description: "<p style=\"font-size: 20px; font-weight:bold;\">Assert</p>

            <p>Welcome to Noir Playground! In this first challenge, we will write a program that asserts that two integers are not equal.</p>
        
            <p style=\"font-size: 18px; font-weight:bold;\">Example 1:</p>
            <p>Input: x=1, y=2</p>
            <ul>
                <li>Program passes and proof is generated</li>
                <li>x and y are not equal</li>
            </ul>
        
            <p style=\"font-size: 18px; font-weight:bold;\">Example 2:</p>
            <p>Input: x=5, y=5</p>
            <ul>
                <li>Program does not pass and proof is not generated</li>
                <li>x and y are equal</li>
            </ul>
            
            <p>On the right you will see some code already:</p>
            <pre>
                <code>
                fn main() {}
                </code>
            </pre>
        
            <p>Please type your code into this main function.</p>
            
            <p>Optional: Click the `run` button to run any tests you have written. If you have not written any tests, the response will be empty. PS don’t worry if you don’t know about testing yet - we will learn more later.</p>
            
            <p>The prover inputs are the values you would like to input to generate a proof of program execution. In the first example, the prover inputs are x:1 and y:2.</p>
        
            <p>When you are happy with your code and prover inputs, click the `Submit` button to check that your code works and to run the prover and verifier. See the results in the Response box.</p>
        
            <p style=\"font-size: 18px; font-weight:bold;\">Hint:</p>
            <p>If you're stuck, check out the Intro to Noir Syntax</p>".into(),
            test_cases: vec![
                "#[test]
                fn test_main_fromserver() {
                    main(1,2);
                    main(0,5);
                }".into(),
            ],
        },
        Challenge {
            id: 2,
            description: " <p style=\"font-size: 20px; font-weight:bold;\">Functions and Variables</p>
            <p>This challenge is to create two functions: <code>main</code> and <code>add</code> in Noir lang. </p>
        
            <ul>
                <li><b>add function:</b> It should take two numbers as inputs, add them together, and then return the result.</li>
                <li><b>main function:</b> This should take two numbers as inputs, call the <code>add</code> function with these inputs, and check if the returned result is the same as the sum of the inputs.
            </ul>
            <p>Your task is to implement the two functions. Here's an example of what your inputs might look like:</p>
        
            <p style=\"font-size: 18px; font-weight:bold;\">Hint:</p>
            <p>Use <code>let</code> to declare variables.</p>
            <p>When you are happy with your code, click `Check` to get the prover inputs. Then fill them out and click the `Submit` button. Enjoy coding!</p>".into(),
            test_cases: vec![
                "#[test]
                fn test_main_fromserver() {
                    main(1, 2);
                    main(0,1);
                    add(1,2);
                }".into(),
            ],
        },
        Challenge {
            id: 3,
            description: "<p style=\"font-size: 20px; font-weight:bold;\">A Merkle Tree</p>
            <p>
               is a data structure used to verify the integrity of data. Every piece of information, 
               or a leaf, is hashed. These hashes are then paired, hashed, paired again, and hashed again 
               until you get a single hash, known as the root. This process forms a tree-like diagram of 
               hashes, hence the name, Merkle Tree.
            </p>
        
            <p>
                In this task, we are going to take a famous example of Merkle Trees: ICONOMI’s proof of solvency.
                ICONOMI created a merkle tree of all of their commitments, i.e user accounts and owed loans,
                and allowed users to check that their account was included in the Merkle tree without giving them
                access to other user’s sensitive data. You can read more about that 
                <a href=\"https://medium.com/iconominet/proof-of-solvency-technical-overview-d1d0e8a8a0b8\">here</a>.
            </p>
        
            <p style=\"font-size: 18px; font-weight:bold;\">Task</p>
            <p>
                You will write a Noir program that allows the user to check that their account is included in a given
                Merkle tree. These are the inputs that will be used to test and prove the function so please take them 
                in this order:
            </p>
            
            <ul>
                <li><code>root</code>: The root of the Merkle tree.</li>
                <li><code>account_id</code>: The unique ID of the account.</li>
                <li><code>index</code>: The index path of the account in the tree.</li>
                <li><code>hash_path_1</code>: First part of the hash path.</li>
                <li><code>hash_path_2</code>: Second part of the hash path.</li>
                <li><code>commitment</code>: The commitment of this account in the tree.</li>
            </ul>
        
            <p>
                You should compute a Merkle root using this information and check that it matches the existing 
                Merkle root given to the user by the company.
            </p>
        
            <p style=\"font-size: 18px; font-weight:bold;\">Hint</p>
            <p>
                You can write tests as helper functions, for example generating a tree and then testing that your program
                generates the same hashes.
            </p>
        
            <p style=\"font-size: 18px; font-weight:bold;\">Helpful Functions</p>
        
            <pre>
                <code>
                    std::hash::pedersen()
        
                    std::merkle::compute_merkle_root
                </code>
            </pre>
        
            <p>Good luck!</p>".into(),
            test_cases: vec![
                "#[test]
                fn test_main_fromserver() {
                    let root = 0x29fd5ee89e33f559a7b32ac39f57400aa5a6c77492e28c088f9eb511b0c73e78; // Root from built Merkle Tree
                    let account_id = 1;
                    let index = 0;
                
                    // Hash path should include sibling hashes from commitment to root.
                    let hash_path_1 = 0x2d961d9814298c04a4639a56c5c95030d704340ab6d13c135a326da5e515559d; // Sibling of commitment1 -> commitment2 (sibling at level 1)
                    let hash_path_2 = 0x1501e80783ee5c988327f46f5fcdce388cb97aa7e959ad345c1e2cbaa0b42b83; // Sibling of left_branch -> right_branch (sibling at level 2)
                
                
                    let id_commitment = std::hash::pedersen([account_id])[0];
                    std::println(id_commitment);
                    let computed_root = main(root, account_id, index, hash_path_1, hash_path_2, id_commitment); 
                
                    assert(root == computed_root);
                }".into(),
            ],
        },
    ])
}
