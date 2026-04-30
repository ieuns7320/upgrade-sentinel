use serde::Serialize;

// 탐지 규칙이 발견한 보안 이상 징후를 CLI/JSON 출력에 공통으로 사용하는 구조체이다.
#[derive(Debug, Serialize)]
pub struct Finding {
    // 규칙 식별자. 예: UPG-001
    pub id: String,
    // 사람이 우선순위를 판단할 수 있는 심각도 값이다.
    pub severity: String,
    // finding의 짧은 제목이다.
    pub title: String,
    // 왜 검토가 필요한지 설명하는 상세 문장이다.
    pub description: String,
    // finding이 발생한 컨트랙트 이름이다.
    pub contract_name: String,
    // 특정 함수와 연결되는 finding이면 함수명을 담고, 아니면 None을 사용한다.
    pub function_name: Option<String>,
    // 판정에 사용된 근거들을 문자열 목록으로 보관한다.
    pub evidence: Vec<String>,
}
