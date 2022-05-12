use near_jsonrpc_client::methods;

mod utils;

pub fn specify_block_reference() -> std::io::Result<near_primitives::types::BlockReference> {
    println!("=========[Block Reference]=========");
    let block_reference = utils::select(
        || {
            println!(" [1] final        \x1b[38;5;244m(alias: f, fin)\x1b[0m");
            println!(" [2] optimistic   \x1b[38;5;244m(alias: o, opt)\x1b[0m");
            println!(" [3] block hash   \x1b[38;5;244m(alias: s, hash)\x1b[0m");
            println!(" [4] block height \x1b[38;5;244m(alias: h, height)\x1b[0m");
        },
        "\x1b[33m(enter a selection)\x1b[0m> ",
        |selection| match (selection, selection.parse()) {
            ("f" | "fin" | "final", _) | (_, Ok(1)) => {
                Some(near_primitives::types::BlockReference::Finality(
                    near_primitives::types::Finality::Final,
                ))
            }
            ("o" | "opt" | "optimistic", _) | (_, Ok(2)) => {
                Some(near_primitives::types::BlockReference::Finality(
                    near_primitives::types::Finality::None,
                ))
            }
            ("s" | "hash" | "block hash", _) | (_, Ok(3)) => loop {
                match utils::input("What block hash should we query? ")
                    .unwrap()
                    .parse()
                {
                    Ok(block_hash) => {
                        break Some(near_primitives::types::BlockReference::BlockId(
                            near_primitives::types::BlockId::Hash(block_hash),
                        ))
                    }
                    _ => println!("(i) Invalid block hash, please reenter!"),
                }
            },
            ("h" | "height" | "block height", _) | (_, Ok(4)) => loop {
                match utils::input("What block height should we query? ")
                    .unwrap()
                    .parse()
                {
                    Ok(block_height) => {
                        break Some(near_primitives::types::BlockReference::BlockId(
                            near_primitives::types::BlockId::Height(block_height),
                        ))
                    }
                    _ => println!("(i) Invalid block height, please reenter!"),
                }
            },
            _ => None,
        },
    )?;
    println!("===================================");

    Ok(block_reference)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let client = utils::select_network()?;

    // tolerate only 3 retries for a non-failing transaction hash
    'root: for _ in 1..=3 {
        let tx_hash = 'tx_hash: loop {
            // tolerate only 3 retries for a valid transaction hash
            for _ in 1..=3 {
                if let Ok(tx_hash) =
                    utils::input("What transaction hash should we query? ")?.parse()
                {
                    break 'tx_hash tx_hash;
                }
                println!("(i) Invalid transaction hash!");
            }

            break 'root;
        };

        let account_id = 'account_id: loop {
            // tolerate only 3 retries for a valid Account ID
            for _ in 1..=3 {
                if let Ok(account_id) =
                    utils::input("What account signed this transaction? ")?.parse()
                {
                    break 'account_id account_id;
                }
                println!("(i) Invalid Account ID!");
            }

            break 'root;
        };

        match client
            .call(methods::tx::RpcTransactionStatusRequest {
                transaction_info: methods::tx::TransactionInfo::TransactionId {
                    hash: tx_hash,
                    account_id,
                },
            })
            .await
        {
            Ok(block_details) => println!("{:#?}", block_details),
            Err(err) => match err.handler_error() {
                Some(err) => {
                    println!("(i) An error occurred `{:#?}`", err);
                    continue;
                }
                _ => println!("(i) A non-handler error ocurred `{:#?}`", err),
            },
        };
        break;
    }

    Ok(())
}
