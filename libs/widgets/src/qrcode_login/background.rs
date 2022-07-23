// pub(super) async fn qr_code_handler(client: Arc<Client>, sender: Sender<Input>, temp_path: &Path) {
//     let mut timer = tokio::time::interval(Duration::from_millis(400));
//     let mut qrcode_state = match client.fetch_qrcode().await {
//         Ok(qrcode) => qrcode,
//         Err(err) => {
//             sender.send(Input::Error(err));
//             return;
//         }
//     };

//     let mut qrcode_sign = Option::None;
//     loop {
//         match qrcode_state {
//             ricq::QRCodeState::ImageFetch(ref qrcode) => {
//                 let img = &qrcode.image_data;
//                 tokio::fs::write(temp_path, &img)
//                     .await
//                     .expect("failure to write qrcode file");
//                 qrcode_sign.replace(qrcode.sig.clone());
//                 sender.send(Input::UpdateQrCode)
//             }
//             ricq::QRCodeState::WaitingForScan => {}
//             ricq::QRCodeState::WaitingForConfirm => {}
//             ricq::QRCodeState::Timeout => match client.fetch_qrcode().await {
//                 Ok(qr_state) => {
//                     qrcode_state = qr_state;
//                     continue;
//                 }
//                 Err(err) => {
//                     sender.send(Input::Error(err));
//                     return;
//                 }
//             },
//             ricq::QRCodeState::Confirmed(ref qrcode_confirm) => {
//                 let login_respond = client
//                     .qrcode_login(
//                         &qrcode_confirm.tmp_pwd,
//                         &qrcode_confirm.tmp_no_pic_sig,
//                         &qrcode_confirm.tgt_qr,
//                     )
//                     .await;
//                 match login_respond {
//                     Ok(ok_respond) => sender.send(Input::FollowLogin(ok_respond.into())),
//                     Err(err) => sender.send(Input::Error(err)),
//                 }
//                 return;
//             }
//             ricq::QRCodeState::Canceled => todo!(),
//         }

//         timer.tick().await;
//         let qrcode_sig = Into::<Option<&Bytes>>::into(&qrcode_sign)
//             .map(|byte| -> &[u8] { byte })
//             .unwrap_or(&[]);
//         qrcode_state = match client.query_qrcode_result(qrcode_sig).await {
//             Ok(state) => state,
//             Err(err) => {
//                 sender.send(Input::Error(err));
//                 return;
//             }
//         }
//     }
// }
