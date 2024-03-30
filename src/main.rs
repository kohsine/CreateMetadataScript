use mpl_token_metadata::{
    accounts::Metadata,
    instructions::{CreateMetadataAccountV3, CreateMetadataAccountV3InstructionArgs},
    types::DataV2,
};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    bs58, message::Message, pubkey::Pubkey, signature::read_keypair_file, signer::Signer,
    transaction::Transaction,
};

const KEYPAIR_FILE: &str = "/path/to/id.json";
const MINT_ID: &str = "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v";
const RPC_URL: &str = "https://api.mainnet-beta.solana.com";

const TOKEN_NAME: &str = "";
const TOKEN_SYMBOL: &str = "";
const TOKEN_URI: &str = "https://something.token.com/api/token/metadata";

const METADATA_IS_MUTABLE: bool = false;

fn main() {
    let mint_id = Pubkey::try_from(bs58::decode(MINT_ID).into_vec().unwrap()).unwrap();
    dbg!(mint_id);

    let keypair = read_keypair_file(KEYPAIR_FILE).unwrap();

    let (metadata_id, _) = Metadata::find_pda(&mint_id);
    dbg!(metadata_id);

    let data = DataV2 {
        name: TOKEN_NAME.to_string(),
        symbol: TOKEN_SYMBOL.to_string(),
        uri: TOKEN_URI.to_string(),
        seller_fee_basis_points: 0,
        creators: None,
        collection: None,
        uses: None,
    };

    let args = CreateMetadataAccountV3InstructionArgs {
        data,
        is_mutable: METADATA_IS_MUTABLE,
        collection_details: None,
    };

    let create_ix = CreateMetadataAccountV3 {
        metadata: metadata_id,
        mint: mint_id,
        mint_authority: keypair.pubkey(),
        payer: keypair.pubkey(),
        update_authority: (keypair.pubkey(), true),
        system_program: solana_sdk::system_program::ID,
        rent: None,
    };
    let create_ix = create_ix.instruction(args);
    let message = Message::new(&[create_ix], Some(&keypair.pubkey()));

    let client = RpcClient::new(RPC_URL);
    let blockhash = client.get_latest_blockhash().unwrap();

    let tx = Transaction::new(&[&keypair], message, blockhash);
    let res = client.send_and_confirm_transaction(&tx).unwrap();
    dbg!(res);
}
