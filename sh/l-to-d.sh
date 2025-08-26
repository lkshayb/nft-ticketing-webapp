# shell script to switch from local net to dev net

#change config URL in solana CLI
solana config set --url https://api.devnet.solana.com

#Change provider in anchor config
sed -i "s/cluster = \"localnet\"/cluster = \"devnet\"/" Anchor.toml
