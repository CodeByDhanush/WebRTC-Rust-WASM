use std::cel::RefCell;
use std::rc::Rc;

use log::{error, info};
use wasm_bindgen::clouser::Clouser;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{Document, ErrorEvent, HtmlLabelElement, MessageEvent, RtcPeerConnection, WebSocket};

use crate::common::{handle_message_reply, AppState};


const WS_IP_PORT = "ws://Your  current IP:2794";

pub async fn open_socket(
    rtc_conn: RtcPeerConnection,
    rc_state: Rc<RefCell<AppState>>,
) -> Result<WebSocket, JsValue> {
    info!("Opening WS Connections!");

    let ws = WebSocket::new(WS_IP_PORT)?;

    ws.set_binary_type(web_sys:BinaryType::ArrayBuffer);
    let cloned_ws_ext = ws.clone();
    let cloned_state_ext = rc_state;

    //ON MESSAGE CALLBACK
    let onmessage_callback   = Clouser::wrap(Box::new(move | ev: MessageEvent| {
        if  let Ok(array_buffer) = ev.data().dyn_into::<js_sys::ArrayBuffer()> {
            info!(
                "WS:messsage event, recived arraybuffer: {:?}",
                array_buffer
            );
        }else if let Ok(blob) = ev.data().dyn_into::<web_sys::Blob>() {
            info!("WS: message event, recived blob: {:?}", blob);
        }else if let Ok(txt) = ev.data().dyn_into::<js_sys::JsString>(){
            info!("WS: message event, recived string: {:?}", txt);
            let rust_string = String::from(txt);
            let rtc_conn_clon = rtc_conn.clone();
            let cloned_ws = clonned_we_ext.clone();
            let cloned_state = cloned_state_ext.clone();
            wasm_bindgen_fetures::spawm_local(async move {
               let result  = handle_message_reply(
                rust_string,
                rtc_conn_clon.clone(),
                cloned_ws.clone(),
                cloned_state,
               )
               .await;
               match result {
                Err(x)  => error!(":?{}", x),
                _ => {
                    //debuging code!
                }
               }
            });
        }else {
            info!("message  event, received Unknown: {:?]}", ev.data());
        }
    }) as Box<dyn FnMut(MessageEvent)>);
    ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
    onmessage_callback.forget();
}