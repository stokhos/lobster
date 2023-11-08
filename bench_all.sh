cargo clean

git switch master
cargo +stable bench -- --save-baseline master

git switch btree_tinyvec
cargo +stable bench -- --save-baseline btree_tinyvec

git switch btree_tinyvec_ahash
cargo +stable bench -- --save-baseline btree_tinyvec_ahash

git switch qp_trie
cargo +stable bench -- --save-baseline qp_trie

git switch qp_trie_tinyvec_2
cargo +stable bench -- --save-baseline qp_trie_tinyvec

git switch qp_trie_tinyvec_ahash
cargo +stable bench -- --save-baseline qp_trie_ahash

cargo +stable bench
cargo bench -- --load-baseline new --baseline master
cargo bench -- --load-baseline new --baseline btree_tinyve
cargo bench -- --load-baseline new --baseline btree_tinyvec_ahash
cargo bench -- --load-baseline new --baseline qp_trie
cargo bench -- --load-baseline new --baseline qp_trie_tinyvec
