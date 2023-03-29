use multi_party_ecdsa::communication::sending_messages::SendingMessages;
use multi_party_ecdsa::protocols::multi_party::dmz21::keygen::KeyGenPhase;
use multi_party_ecdsa::protocols::multi_party::dmz21::keygen::Parameters;

fn main() {
    let partyid = "1".to_string();
    let params = Parameters {
        threshold: 1,
        share_count: 3,
    };
    let party_ids = vec!["1".to_string(), "2".to_string(), "3".to_string()];
    let mut keygen = KeyGenPhase::new(partyid, params, &Some(party_ids)).unwrap();

    let sending_msg: SendingMessages = keygen.process_begin().unwrap();

    match sending_msg {
        SendingMessages::BroadcastMessage(msg) => {
            // broadcast the msg to all(including self).
        }
        SendingMessages::P2pMessage(msg) => {
            // send according to the k,v in the msg. k is the index which v will to be sent to.
        }
        SendingMessages::SubsetMessage(msg) => {
            // send according to the k in the party_ids or subset(used in sign phase). k is the index which msg will to be sent to.
        }
        _ => {}
    }

    loop {
        // let (recv_from, recv_msg) = According to the last round of SendingMessages
        let recv_from = "".to_string();
        let recv_msg = vec![0u8];
        let sending_msg = keygen.msg_handler(recv_from, &recv_msg).unwrap();
        match sending_msg {
            SendingMessages::KeyGenSuccessWithResult(msg) => {
                // got the keygen result
                break;
            }
            _ => {
                // other sending messages, ref Step 2.
            }
        }
    }
}
