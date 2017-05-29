

// fn download(request: Request) ->



// fn connect(url: Url) -> UrlState<'static> {
//     let (tx, rx) = channel();
//     let tx_ = tx.clone();
//     let url_ = url.clone();
//
//     thread::spawn(move || {
//         let client = Client::new();
//         let response;
//         {
//             let url_str = url.as_str();
//             response = client.get(url_str).send();
//         }
//
//
//         let _ = tx.send(
//             match response {
//                 Ok(mut response) => {
//                     if let StatusCode::Ok = response.status {
//                         let mut buffer = vec![];
//                         match response.read_to_end(&mut buffer){
//                             Ok(_) => {
//                                 UrlState::Accessible(url, buffer)
//                             },
//                             Err(err) => {
//                                 UrlState::ReadError(url, err)
//                             },
//                         }
//                     } else {
//                         // TODO: allow redirects unless they're circular
//                         UrlState::BadStatus(url, response.status)
//                     }
//                 }
//                 Err(_) => UrlState::ConnectionFailed(url),
//             }
//         );
//     });
//
//     thread::spawn(move || {
//         thread::sleep(Duration::from_secs(TIMEOUT_SECS));
//         let _ = tx_.send(UrlState::TimedOut(url_));
//     });
//
//     rx.recv().unwrap()
// }
