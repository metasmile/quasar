use candid::{CandidType, Deserialize};
use ic_cdk::api::management_canister::http_request::TransformFunc;

use ic_cdk::api::management_canister::http_request::{
    http_request, CanisterHttpRequestArgument, HttpHeader, HttpResponse, HttpMethod, TransformContext,
};
use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager, VirtualMemory},
    DefaultMemoryImpl, StableCell,
};
use serde::Serialize;
use std::cell::RefCell;

type Memory = VirtualMemory<DefaultMemoryImpl>;

// 메모리 관리 및 상태 저장
thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    // API 키 저장 (MemoryId 0)
    static API_KEY: RefCell<StableCell<String, Memory>> = RefCell::new(
        StableCell::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))),
            "".to_string() // 초기값은 빈 문자열
        ).unwrap()
    );

    // 마지막 API 응답 저장 (MemoryId 1)
    static LAST_RESPONSE: RefCell<StableCell<String, Memory>> = RefCell::new(
        StableCell::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1))),
            "".to_string()
        ).unwrap()
    );
}

// API 응답 구조체
#[derive(Serialize, Deserialize, CandidType, Debug)]
struct SupplyDistribution {
    date: String,
    supply: f64,
    percentage: f64,
}

// API 키 설정
#[ic_cdk::update]
fn set_api_key(key: String) {
    API_KEY.with(|cell| {
        cell.borrow_mut().set(key).expect("Failed to set API key");
    });
}

// API 키 조회
#[ic_cdk::query]
fn get_api_key() -> String {
    API_KEY.with(|cell| cell.borrow().get().to_string())
}

// HTTP 응답 변환 함수
#[ic_cdk::query]
fn transform_response(raw: (Vec<u8>, HttpResponse)) -> HttpResponse {
    // Destructure the tuple to get the response
    let (_context, response) = raw;
    response
}

// ResearchBitcoin API 호출
#[ic_cdk::update]
async fn fetch_supply_data(date: String) -> Result<SupplyDistribution, String> {
    let api_key = get_api_key();
    if api_key.is_empty() {
        return Err("API key not set".to_string());
    }

    let request = CanisterHttpRequestArgument {
        url: format!("https://api..."),
        method: HttpMethod::GET,
        headers: vec![HttpHeader {
            name: "Accept".to_string(),
            value: "application/json".to_string(),
        }],
        max_response_bytes: None,
        body: None,
        transform: Some(TransformContext {
            function: TransformFunc::new(
                ic_cdk::api::id(),
                "transform_response".to_string(),
            ),
            context: vec![], // Empty context since we're not using it
        }),
    };
    
    match http_request(request, 30_000_000_000).await {
        Ok((response,)) => {
            let response_str = String::from_utf8(response.body)
                .map_err(|e| format!("Failed to parse response: {}", e))?;
            
            // 응답 저장
            LAST_RESPONSE.with(|cell| {
                cell.borrow_mut()
                    .set(response_str.clone())
                    .expect("Failed to store response");
            });

            serde_json::from_str::<SupplyDistribution>(&response_str)
                .map_err(|e| format!("Failed to parse JSON: {}", e))
        }
        Err((code, msg)) => Err(format!("HTTP request failed: {:?} - {}", code, msg)),
    }
}

// 마지막 응답 조회
#[ic_cdk::query]
fn get_last_response() -> String {
    LAST_RESPONSE.with(|cell| cell.borrow().get().to_string())
}

// 인사말 함수
#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

// Candid 인터페이스 정의
ic_cdk::export_candid!();