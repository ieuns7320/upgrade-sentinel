# UpgradeSentinel

**UpgradeSentinel**은 업그레이드형 스마트컨트랙트를 대상으로 보안 이상 징후를 탐지하기 위한 **하이브리드 분석 도구**이다.  
Rust 기반의 정적 분석과 Go 기반의 온체인 정보 수집을 결합하여, 업그레이드형 컨트랙트에서 자주 발생하는 초기화, 권한 제어, proxy 구성, 위험 호출 관련 문제를 분석하는 것을 목표로 한다.

</br>

## 프로젝트 목표

업그레이드형 스마트컨트랙트는 일반 컨트랙트보다 구조가 복잡하다.  
Proxy와 Implementation의 분리, initializer 기반 초기화, admin/owner/upgrader 권한 관리, `delegatecall` 기반 동작, EIP-1967 slot 사용 등으로 인해 단순한 함수 단위 분석만으로는 보안 상태를 충분히 판단하기 어렵다.

이 프로젝트의 목표는 다음과 같다.

- 업그레이드형 스마트컨트랙트의 보안 이상 징후 탐지
- 정적 분석과 온체인 정보 수집의 결합
- 사람이 읽을 수 있는 보안 리포트 생성
- 업그레이드형 컨트랙트 분석 과정을 자동화할 수 있는 기반 마련

즉, UpgradeSentinel은 단순한 Solidity 파서가 아니라, **업그레이드형 스마트컨트랙트 보안 검토를 보조하는 분석기**를 지향한다.

</br>


## 핵심 아이디어

> UpgradeSentinel은 두 가지 축으로 구성된다.

</br>

#### 1. Rust 기반 정적 분석

> Rust는 분석 코어를 담당한다.

주요 역할은 다음과 같다.

- Solidity AST 또는 빌드 결과 JSON 파싱
- contract / function / modifier / 구조 정보 추출
- 보안 규칙 기반 이상 징후 탐지
- finding 생성
- CLI / JSON / Markdown 형태의 결과 출력

Rust 분석기는 코드 자체를 읽고, 업그레이드형 컨트랙트에서 의미 있는 보안 신호를 정리하는 역할을 맡는다.

</br>

#### 2. Go 기반 온체인 정보 수집

> Go는 온체인 수집기를 담당한다.

주요 역할은 다음과 같다.

- RPC 연결
- 컨트랙트 코드 조회
- EIP-1967 implementation/admin slot 조회
- 대상 주소의 메타데이터 수집
- 정적 분석 결과와 결합 가능한 온체인 정보 제공

Go 수집기는 소스코드만으로는 확인하기 어려운 **실제 배포 상태와 proxy 관련 온체인 정보**를 보완한다.

</br>

## 분석 대상

> 이 프로젝트는 일반 스마트컨트랙트 전반이 아니라, 특히 **업그레이드형 스마트컨트랙트**를 주요 분석 대상으로 한다.

주요 관심 영역은 다음과 같다.

- initializer 노출 여부
- implementation contract 직접 초기화 가능성
- upgrade 경로 접근제어 이상
- `delegatecall` 및 저수준 호출 위험
- `selfdestruct` 등 파괴 가능 코드 경로
- EIP-1967 slot 기반 proxy 구성 확인


</br>

## 구현 목표


**Step 1. 정적 분석 입력 파이프라인 구축**

- AST JSON 입력
- contract / function 구조 파싱
- 기본 분석 대상 식별

</br>

**Step 2. 초기화 함수 관련 규칙 구현**

- `initialize` / `init` 함수 탐지
- visibility 및 modifier 기반 위험도 보정
- initializer exposure 후보 식별

</br>

**Step 3. Implementation lock 관련 규칙 구현**

- constructor 존재 여부 확인
- `_disableInitializers()` 흔적 탐지
- implementation contract 잠금 부재 후보 탐지

</br>

**Step 4. 접근제어 및 위험 호출 규칙 구현**

- upgrade 함수 후보 탐지
- `onlyOwner`, `onlyAdmin` 등 modifier 확인
- `delegatecall`, low-level `call`, `selfdestruct` 사용 여부 분석

</br>

**Step 5. 온체인 정보 수집기 구현**

- RPC를 통한 code / storage 조회
- EIP-1967 slot 분석
- proxy / implementation 상태 정보 수집

</br>

**Step 6. 분석 결과 통합 및 리포트 생성**

- 정적 분석 결과와 온체인 정보 결합
- CLI 요약 출력
- JSON / Markdown 리포트 생성

</br>

## 기대 효과

> 이 프로젝트를 통해 기대하는 효과는 다음과 같다.

</br>

**1. 업그레이드형 컨트랙트 보안 분석 자동화**

초기화, 권한, proxy slot, 위험 호출 등 사람이 반복적으로 확인해야 하는 항목을 일정 부분 자동화할 수 있다.

</br>

**2. 정적 분석과 온체인 분석의 결합**

코드만 보는 분석기의 한계를 넘어, 실제 온체인 배포 상태를 함께 반영하는 분석 흐름을 실험할 수 있다.

</br>

**3. Web3 보안 연구 기반 마련**

업그레이드형 컨트랙트에서 자주 발생하는 보안 실수와, 정적 분석만으로 탐지 가능한 범위 / 온체인 상태 확인이 필요한 범위를 구분하는 연구적 기반이 된다.

</br>

**4. 포트폴리오 및 실전 역량 강화**

이 프로젝트는 단순한 실습 코드가 아니라,
- 정적 분석기 설계
- RPC 수집기 구현
- 보안 규칙 정의
- 리포트 생성
까지 포함하므로, Web3 보안 포트폴리오로도 활용할 수 있다.

</br>

## 프로젝트 구조

```text
upgrade-sentinel/
├─ rust-analyzer/     # Rust 기반 정적 분석 코어
├─ go-collector/      # Go 기반 온체인 정보 수집기
├─ samples/           # 안전/취약 샘플 및 AST 입력 예제
├─ docs/              # 명세 및 설계 문서
├─ schema/            # 결과 JSON 스키마
└─ reports/           # 생성된 분석 리포트
```

</br>

## 출력 형태

최종적으로는 다음과 같은 출력 형식을 목표로 한다.

* CLI Summary
    빠르게 확인할 수 있는 결과 요약
* JSON Report
    자동 처리 및 후속 분석을 위한 구조화된 결과
* Markdown Report
    사람이 읽기 쉬운 형태의 보고서

</br>

## 지향점

UpgradeSentinel은 “모든 취약점을 자동 탐지하는 완전한 솔루션”을 목표로 하지 않는다.
그 대신, 업그레이드형 스마트컨트랙트 보안 검토 과정에서 자주 등장하는 위험 신호를 빠르게 식별하고,
사람이 추가 검토해야 할 포인트를 구조적으로 정리해주는 보안 분석 보조 도구를 지향한다.

**즉, 이 프로젝트의 핵심은 완전한 자동 판정이 아니라 설명 가능한 보안 이상 징후 탐지에 있다.**


</br>

## 한 줄 요약

UpgradeSentinel은 Rust 정적 분석과 Go 온체인 정보 수집을 결합해 업그레이드형 스마트컨트랙트의 보안 이상 징후를 탐지하는 Web3 보안 분석기 프로젝트이다.