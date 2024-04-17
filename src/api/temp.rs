#[derive(Deserialize, Debug)]
struct BotActionData {
    #[serde(rename = "APPID", default)]
    app_id: Option<String>,
    #[serde(rename = "STATUS", default)]
    status: Option<BotActionStatus>,
    #[serde(rename = "NUM", default)]
    block_number: Option<u64>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
enum BotActionStatus {
    Ack,
    Ready,
}

async fn bot_action(
    State(sessions): State<Sessions>,
    Query(data): Query<BotActionData>,
) -> impl IntoResponse {
    tokio::time::sleep(Duration::from_secs(1)).await;

    match data.status {
        Some(BotActionStatus::Ready) => {
            let commands: Vec<[u8; 3]> = vec![
                BoardAction::StartBlock,
                BoardAction::BlockNumber(1),
                BoardAction::StartDrawing,
                BoardAction::Move(3500, 1200),
                BoardAction::StopDrawing,
            ]
            .into_iter()
            .map(|x| x.serialize())
            .collect();

            let mut data = vec![];

            for i in &commands {
                data.extend_from_slice(i);
            }

            return data;
        }
        _ => return vec![],
    }
}
