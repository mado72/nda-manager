# NDA Backend - Diagramas de Fluxo

Este documento contém diagramas Mermaid que ilustram os fluxos de informação e processos do sistema NDA Backend.

## Índice

- [NDA Backend - Diagramas de Fluxo](#nda-backend---diagramas-de-fluxo)
  - [Índice](#índice)
  - [Fluxo Completo do Sistema](#fluxo-completo-do-sistema)
  - [Fluxo de Registro de Usuário](#fluxo-de-registro-de-usuário)
  - [Fluxo de Autenticação](#fluxo-de-autenticação)
    - [Login com Senha](#login-com-senha)
    - [Auto-Login](#auto-login)
  - [Fluxo de Criação de Processo](#fluxo-de-criação-de-processo)
  - [Fluxo de Compartilhamento (Blockchain)](#fluxo-de-compartilhamento-blockchain)
  - [Fluxo de Acesso ao Conteúdo](#fluxo-de-acesso-ao-conteúdo)
  - [Arquitetura em Camadas](#arquitetura-em-camadas)
  - [Fluxo de Dados entre Componentes](#fluxo-de-dados-entre-componentes)
  - [Ciclo de Vida de um Processo](#ciclo-de-vida-de-um-processo)
  - [Fluxo de Segurança e Verificações](#fluxo-de-segurança-e-verificações)
  - [Fluxo de Notificações e Auditoria](#fluxo-de-notificações-e-auditoria)
  - [Modelo de Dados e Relacionamentos](#modelo-de-dados-e-relacionamentos)
  - [Fluxo de Criptografia Detalhado](#fluxo-de-criptografia-detalhado)
  - [Integração Stellar Blockchain](#integração-stellar-blockchain)
  - [Resumo dos Fluxos Principais](#resumo-dos-fluxos-principais)
    - [1. Registro → Blockchain Account](#1-registro--blockchain-account)
    - [2. Criar Processo → Criptografia](#2-criar-processo--criptografia)
    - [3. Compartilhar → Blockchain Transaction](#3-compartilhar--blockchain-transaction)
    - [4. Acessar → Verificação + Descriptografia](#4-acessar--verificação--descriptografia)
    - [5. Notificações → Auditoria](#5-notificações--auditoria)
  - [Segurança em Múltiplas Camadas](#segurança-em-múltiplas-camadas)
  - [Conclusão](#conclusão)

---

## Fluxo Completo do Sistema

```mermaid
graph TB
    Start([Início]) --> ClientReg[Cliente se Registra]
    Start --> PartnerReg[Parceiro se Registra]
    
    ClientReg --> CreateProc[Cliente Cria Processo NDA]
    CreateProc --> Encrypt[Sistema Criptografa Conteúdo<br/>AES-256-GCM]
    Encrypt --> StoreDB[(Armazena no Banco)]
    
    StoreDB --> ShareProc[Cliente Compartilha Processo]
    ShareProc --> BlockchainTx[Cria Transação Stellar]
    BlockchainTx --> RecordShare[(Registra Compartilhamento)]
    
    PartnerReg --> AccessProc[Parceiro Acessa Processo]
    RecordShare --> AccessProc
    
    AccessProc --> VerifyShare{Compartilhamento<br/>Verificado?}
    VerifyShare -->|Sim| Decrypt[Descriptografa Conteúdo]
    VerifyShare -->|Não| Denied[Acesso Negado]
    
    Decrypt --> LogAccess[(Registra Acesso)]
    LogAccess --> ReturnContent[Retorna Conteúdo]
    
    ReturnContent --> ClientNotif[Cliente Visualiza Notificações]
    
    style Start fill:#e1f5ff
    style ClientReg fill:#c8e6c9
    style PartnerReg fill:#ffecb3
    style Encrypt fill:#f48fb1
    style BlockchainTx fill:#ce93d8
    style Decrypt fill:#f48fb1
    style Denied fill:#ef5350
    style ReturnContent fill:#81c784
```

---

## Fluxo de Registro de Usuário

```mermaid
sequenceDiagram
    participant Client as Cliente/Parceiro
    participant API as API Handler
    participant Auth as Auth Module
    participant Stellar as Stellar Client
    participant DB as Database
    participant TestNet as Stellar Testnet

    Client->>API: POST /api/users/register<br/>{username, name, password, roles}
    
    API->>DB: Verificar se username existe
    
    alt Username já existe
        DB-->>API: Usuário encontrado
        API-->>Client: 409 Conflict
    else Username disponível
        DB-->>API: Username disponível
        
        API->>Stellar: Gerar keypair Ed25519
        Stellar-->>API: {public_key, secret_key}
        
        API->>TestNet: Financiar conta testnet
        TestNet-->>API: Conta financiada (10,000 XLM)
        
        API->>Auth: Hash password (bcrypt)
        Auth-->>API: password_hash
        
        API->>DB: Criar usuário<br/>(username, stellar keys, password_hash, roles)
        DB-->>API: Usuário criado
        
        API-->>Client: 200 OK<br/>{id, username, stellar_public_key, roles}
    end
```

---

## Fluxo de Autenticação

### Login com Senha

```mermaid
sequenceDiagram
    participant Client as Cliente
    participant API as API Handler
    participant Auth as Auth Module
    participant DB as Database

    Client->>API: POST /api/users/login<br/>{username, password}
    
    API->>DB: Buscar usuário por username
    
    alt Usuário não encontrado
        DB-->>API: null
        API-->>Client: 401 Unauthorized
    else Usuário encontrado
        DB-->>API: {id, username, password_hash, ...}
        
        API->>Auth: Verificar senha<br/>bcrypt::verify(password, password_hash)
        
        alt Senha inválida
            Auth-->>API: false
            API-->>Client: 401 Unauthorized
        else Senha válida
            Auth-->>API: true
            API-->>Client: 200 OK<br/>{id, username, stellar_public_key, roles}
        end
    end
```

### Auto-Login

```mermaid
sequenceDiagram
    participant Client as Cliente (localStorage)
    participant API as API Handler
    participant DB as Database

    Client->>API: POST /api/users/auto-login<br/>{user_name, user_id}
    
    API->>DB: Buscar usuário por user_id
    
    alt Usuário não encontrado
        DB-->>API: null
        API-->>Client: 401 Unauthorized
    else Usuário encontrado
        DB-->>API: {id, username, ...}
        
        alt Username não corresponde
            API-->>Client: 401 Unauthorized
        else Username corresponde
            API-->>Client: 200 OK<br/>{id, username, stellar_public_key, roles}
        end
    end
```

---

## Fluxo de Criação de Processo

```mermaid
sequenceDiagram
    participant Client as Cliente
    participant API as API Handler
    participant Crypto as Crypto Module
    participant DB as Database

    Client->>API: POST /api/processes<br/>{client_id, title, description, confidential_content}
    
    API->>DB: Buscar cliente por ID
    
    alt Cliente não encontrado
        DB-->>API: null
        API-->>Client: 422 Unprocessable Entity
    else Cliente encontrado
        DB-->>API: {id, roles, ...}
        
        alt Não tem role "client"
            API-->>Client: 403 Forbidden
        else Tem role "client"
            API->>Crypto: Gerar chave de criptografia<br/>generate_key()
            Crypto-->>API: encryption_key (32 bytes)
            
            API->>Crypto: Criptografar conteúdo<br/>encrypt_content(content, key)
            Crypto-->>API: encrypted_content<br/>(nonce + ciphertext + tag)
            
            API->>DB: Criar processo<br/>(client_id, title, description,<br/>encrypted_content, encryption_key)
            DB-->>API: Process criado
            
            API-->>Client: 200 OK<br/>{id, client_id, title, description, status}
        end
    end

    Note over Crypto: AES-256-GCM<br/>Chave única por processo<br/>Nonce aleatório
```

---

## Fluxo de Compartilhamento (Blockchain)

```mermaid
sequenceDiagram
    participant Client as Cliente
    participant API as API Handler
    participant Stellar as Stellar Client
    participant Horizon as Stellar Horizon API
    participant Blockchain as Stellar Blockchain
    participant DB as Database

    Client->>API: POST /api/processes/share<br/>{client_username, process_id, partner_public_key}
    
    API->>DB: Buscar processo por ID
    alt Processo não encontrado
        DB-->>API: null
        API-->>Client: 404 Not Found
    else Processo encontrado
        DB-->>API: Process
        
        API->>DB: Buscar cliente por username
        alt Cliente não encontrado
            DB-->>API: null
            API-->>Client: 404 Not Found
        else Cliente encontrado
            DB-->>API: {id, stellar_secret_key, ...}
            
            API->>Stellar: Preparar transação de compartilhamento
            Note over Stellar: Operação: Payment<br/>Valor: 1 XLM<br/>Memo: "NDA_SHARE:{process_id}"
            
            Stellar->>Horizon: Buscar conta do cliente
            Horizon-->>Stellar: Account details
            
            Stellar->>Stellar: Assinar transação<br/>(com stellar_secret_key)
            
            Stellar->>Horizon: Submeter transação
            Horizon->>Blockchain: Registrar na blockchain
            Blockchain-->>Horizon: Transaction confirmed
            Horizon-->>Stellar: {hash, success}
            
            Stellar-->>API: {stellar_transaction_hash}
            
            API->>DB: Registrar compartilhamento<br/>(process_id, partner_public_key,<br/>stellar_transaction_hash)
            DB-->>API: ProcessShare criado
            
            API-->>Client: 200 OK<br/>{id, process_id, partner_public_key,<br/>stellar_transaction_hash, shared_at}
        end
    end

    Note over Blockchain: Registro imutável<br/>Prova criptográfica<br/>Verificável independentemente
```

---

## Fluxo de Acesso ao Conteúdo

```mermaid
sequenceDiagram
    participant Partner as Parceiro
    participant API as API Handler
    participant DB as Database
    participant Crypto as Crypto Module

    Partner->>API: POST /api/processes/access<br/>{process_id, partner_username, partner_public_key}
    
    API->>DB: Buscar processo por ID
    alt Processo não encontrado
        DB-->>API: null
        API-->>Partner: 404 Not Found
    else Processo encontrado
        DB-->>API: {id, encrypted_content, encryption_key, ...}
        
        API->>DB: Buscar parceiro por username
        alt Parceiro não encontrado
            DB-->>API: null
            API-->>Partner: 404 Not Found
        else Parceiro encontrado
            DB-->>API: {id, roles, ...}
            
            alt Não tem role "partner"
                API-->>Partner: 403 Forbidden
            else Tem role "partner"
                API->>DB: Verificar compartilhamento<br/>WHERE process_id AND partner_public_key
                
                alt Compartilhamento não existe
                    DB-->>API: null
                    API-->>Partner: 403 Forbidden<br/>"Processo não compartilhado"
                else Compartilhamento existe
                    DB-->>API: ProcessShare found
                    
                    API->>Crypto: Descriptografar conteúdo<br/>decrypt_content(encrypted_content, encryption_key)
                    Crypto-->>API: decrypted_content
                    
                    API->>DB: Registrar acesso<br/>(process_id, partner_id, accessed_at)
                    DB-->>API: ProcessAccess criado
                    
                    API-->>Partner: 200 OK<br/>{process_id, title, description,<br/>content: decrypted_content, accessed_at}
                end
            end
        end
    end

    Note over Crypto: Descriptografia em memória<br/>Não armazenada<br/>Log de acesso para auditoria
```

---

## Arquitetura em Camadas

```mermaid
graph TB
    subgraph "Camada de Apresentação"
        HTTP[HTTP/HTTPS Requests]
        CORS[CORS Middleware]
    end
    
    subgraph "Camada de Roteamento"
        Router[Axum Router]
        Routes[Route Handlers]
    end
    
    subgraph "Camada de Handlers"
        HealthH[health_check]
        UserH[User Handlers<br/>register, login, auto_login]
        ProcessH[Process Handlers<br/>create, list]
        ShareH[Share Handlers<br/>share, access]
        NotifH[Notification Handler<br/>get_notifications]
    end
    
    subgraph "Camada de Lógica de Negócio"
        Auth[Auth Module<br/>Password Hashing/Verify]
        Crypto[Crypto Module<br/>AES-256-GCM]
        Stellar[Stellar Client<br/>Blockchain Integration]
    end
    
    subgraph "Camada de Dados"
        Queries[Database Queries]
        Pool[SQLite Connection Pool]
        DB[(SQLite Database)]
    end
    
    subgraph "Serviços Externos"
        StellarNet[Stellar Testnet/Mainnet<br/>Horizon API]
    end
    
    HTTP --> CORS
    CORS --> Router
    Router --> Routes
    Routes --> HealthH
    Routes --> UserH
    Routes --> ProcessH
    Routes --> ShareH
    Routes --> NotifH
    
    UserH --> Auth
    UserH --> Stellar
    UserH --> Queries
    
    ProcessH --> Crypto
    ProcessH --> Queries
    
    ShareH --> Stellar
    ShareH --> Queries
    
    ShareH --> Crypto
    
    NotifH --> Queries
    
    Queries --> Pool
    Pool --> DB
    
    Stellar --> StellarNet
    
    style HTTP fill:#e3f2fd
    style Router fill:#c5cae9
    style UserH fill:#b2dfdb
    style ProcessH fill:#b2dfdb
    style ShareH fill:#b2dfdb
    style NotifH fill:#b2dfdb
    style Auth fill:#f8bbd0
    style Crypto fill:#f8bbd0
    style Stellar fill:#ce93d8
    style DB fill:#ffccbc
    style StellarNet fill:#ce93d8
```

---

## Fluxo de Dados entre Componentes

```mermaid
graph LR
    subgraph "Frontend (Angular)"
        UI[Interface do Usuário]
        Services[Services]
    end
    
    subgraph "Backend (Rust/Axum)"
        API[API Handlers]
        Models[Models]
        Auth[Auth]
        Crypto[Crypto]
        Stellar[Stellar Client]
        DB_Queries[Database Queries]
    end
    
    subgraph "Armazenamento"
        SQLite[(SQLite DB)]
    end
    
    subgraph "Blockchain"
        StellarNet[Stellar Network]
    end
    
    UI -->|HTTP/JSON| Services
    Services -->|POST/GET| API
    
    API -->|Validate| Models
    API -->|Hash/Verify| Auth
    API -->|Encrypt/Decrypt| Crypto
    API -->|Blockchain Ops| Stellar
    API -->|CRUD| DB_Queries
    
    DB_Queries -->|SQL| SQLite
    SQLite -->|Results| DB_Queries
    
    Stellar -->|Transactions| StellarNet
    StellarNet -->|Confirmation| Stellar
    
    DB_Queries -->|Data| API
    API -->|JSON Response| Services
    Services -->|Display| UI
    
    style UI fill:#e1f5ff
    style API fill:#c8e6c9
    style Auth fill:#f48fb1
    style Crypto fill:#f48fb1
    style Stellar fill:#ce93d8
    style SQLite fill:#ffccbc
    style StellarNet fill:#ce93d8
```

---

## Ciclo de Vida de um Processo

```mermaid
stateDiagram-v2
    [*] --> Created: Cliente cria processo
    
    Created --> Encrypted: Sistema criptografa<br/>(AES-256-GCM)
    
    Encrypted --> Stored: Armazena no banco<br/>com chave única
    
    Stored --> Shared: Cliente compartilha<br/>via blockchain
    
    Shared --> BlockchainRecorded: Transação Stellar<br/>registrada
    
    BlockchainRecorded --> AwaitingAccess: Aguardando acesso<br/>do parceiro
    
    AwaitingAccess --> Accessed: Parceiro acessa<br/>e descriptografa
    
    Accessed --> Logged: Acesso registrado<br/>para auditoria
    
    Logged --> Active: Processo ativo<br/>pode ser acessado novamente
    
    Active --> Accessed: Novos acessos
    
    Active --> Completed: Processo concluído
    
    Completed --> [*]
    
    note right of Encrypted
        Chave única por processo
        Conteúdo não pode ser lido
        sem a chave
    end note
    
    note right of BlockchainRecorded
        Registro imutável
        Prova criptográfica
        Verificável por terceiros
    end note
    
    note right of Logged
        Timestamp
        Partner ID
        Trilha de auditoria
    end note
```

---

## Fluxo de Segurança e Verificações

```mermaid
graph TB
    Start([Request Recebido]) --> Auth{Autenticação<br/>Necessária?}
    
    Auth -->|Sim| CheckAuth{Credenciais<br/>Válidas?}
    Auth -->|Não| RBAC
    
    CheckAuth -->|Não| Unauthorized[401 Unauthorized]
    CheckAuth -->|Sim| RBAC{Verificar<br/>Roles}
    
    RBAC -->|Role Inválido| Forbidden[403 Forbidden]
    RBAC -->|Role Válido| Resource{Recurso<br/>Existe?}
    
    Resource -->|Não| NotFound[404 Not Found]
    Resource -->|Sim| Permission{Tem<br/>Permissão?}
    
    Permission -->|Não| Forbidden2[403 Forbidden]
    Permission -->|Sim| Blockchain{Verificação<br/>Blockchain?}
    
    Blockchain -->|Necessária| CheckShare{Compartilhamento<br/>Verificado?}
    Blockchain -->|Não necessária| Process
    
    CheckShare -->|Não| Forbidden3[403 Forbidden]
    CheckShare -->|Sim| Process[Processar Request]
    
    Process --> Encrypt{Criptografia<br/>Necessária?}
    
    Encrypt -->|Sim| DoCrypto[Criptografar/<br/>Descriptografar]
    Encrypt -->|Não| Log
    
    DoCrypto --> Log[Registrar<br/>Ação]
    
    Log --> Success[200 OK]
    
    Unauthorized --> End([Fim])
    Forbidden --> End
    Forbidden2 --> End
    Forbidden3 --> End
    NotFound --> End
    Success --> End
    
    style Start fill:#e1f5ff
    style Success fill:#81c784
    style Unauthorized fill:#ef5350
    style Forbidden fill:#ef5350
    style Forbidden2 fill:#ef5350
    style Forbidden3 fill:#ef5350
    style NotFound fill:#ff9800
    style DoCrypto fill:#f48fb1
    style Log fill:#ce93d8
```

---

## Fluxo de Notificações e Auditoria

```mermaid
sequenceDiagram
    participant Client as Cliente
    participant API as API Handler
    participant DB as Database

    Client->>API: GET /api/notifications?client_id={id}
    
    API->>DB: Buscar cliente por ID
    
    alt Cliente não encontrado
        DB-->>API: null
        API-->>Client: 404 Not Found
    else Cliente encontrado
        DB-->>API: Client
        
        API->>DB: Buscar acessos com detalhes<br/>JOIN processes, users<br/>WHERE client_id
        
        Note over DB: SELECT pa.*, p.title, p.description,<br/>p.status, u.username<br/>FROM process_accesses pa<br/>JOIN processes p ON pa.process_id = p.id<br/>JOIN users u ON pa.partner_id = u.id<br/>WHERE p.client_id = client_id<br/>ORDER BY pa.accessed_at DESC
        
        DB-->>API: Lista de ProcessAccessWithDetails[]
        
        API-->>Client: 200 OK<br/>[{<br/>  id, process_id, partner_id,<br/>  accessed_at, process_title,<br/>  process_description, process_status,<br/>  partner_username<br/>}, ...]
    end

    Note over Client,DB: Dados desnormalizados para<br/>dashboard rico com contexto completo
```

---

## Modelo de Dados e Relacionamentos

```mermaid
erDiagram
    USERS ||--o{ PROCESSES : "cria (client_id)"
    USERS ||--o{ PROCESS_ACCESSES : "acessa (partner_id)"
    PROCESSES ||--o{ PROCESS_SHARES : "compartilhado"
    PROCESSES ||--o{ PROCESS_ACCESSES : "acessado"
    
    USERS {
        string id PK
        string username UK
        string name
        string stellar_public_key
        string stellar_secret_key
        string password_hash
        string roles "JSON array"
        datetime created_at
    }
    
    PROCESSES {
        string id PK
        string client_id FK
        string title
        string description
        string encrypted_content
        string encryption_key
        string status
        datetime created_at
    }
    
    PROCESS_SHARES {
        string id PK
        string process_id FK
        string partner_public_key
        string stellar_transaction_hash
        datetime shared_at
    }
    
    PROCESS_ACCESSES {
        string id PK
        string process_id FK
        string partner_id FK
        datetime accessed_at
    }
```

---

## Fluxo de Criptografia Detalhado

```mermaid
graph TB
    subgraph "Criação de Processo - Criptografia"
        Plain[Conteúdo Confidencial<br/>Texto Plano]
        GenKey[Gerar Chave AES-256<br/>32 bytes aleatórios]
        GenNonce[Gerar Nonce<br/>12 bytes aleatórios]
        Encrypt[Criptografar com AES-256-GCM]
        Combine[Combinar:<br/>nonce + ciphertext + tag]
        B64Enc[Codificar Base64]
        StoreEnc[(Armazenar:<br/>encrypted_content + encryption_key)]
    end
    
    subgraph "Acesso ao Processo - Descriptografia"
        RetrieveEnc[(Recuperar:<br/>encrypted_content + encryption_key)]
        B64Dec[Decodificar Base64]
        Split[Separar:<br/>nonce | ciphertext + tag]
        Decrypt[Descriptografar com AES-256-GCM]
        Verify[Verificar Tag de Autenticação]
        PlainOut[Conteúdo Confidencial<br/>Descriptografado]
    end
    
    Plain --> GenKey
    Plain --> GenNonce
    GenKey --> Encrypt
    GenNonce --> Encrypt
    Plain --> Encrypt
    Encrypt --> Combine
    Combine --> B64Enc
    B64Enc --> StoreEnc
    
    StoreEnc -.->|Quando acessado| RetrieveEnc
    RetrieveEnc --> B64Dec
    B64Dec --> Split
    Split --> Decrypt
    Decrypt --> Verify
    Verify -->|Tag válida| PlainOut
    Verify -->|Tag inválida| Error[Erro: Conteúdo<br/>foi alterado]
    
    style Plain fill:#e3f2fd
    style GenKey fill:#f48fb1
    style Encrypt fill:#f48fb1
    style StoreEnc fill:#ffccbc
    style PlainOut fill:#81c784
    style Error fill:#ef5350
```

---

## Integração Stellar Blockchain

```mermaid
graph TB
    subgraph "Sistema NDA Backend"
        App[Aplicação Rust]
        StellarClient[Stellar Client Module]
    end
    
    subgraph "Stellar Network"
        Horizon[Horizon API Server<br/>REST Interface]
        Core[Stellar Core<br/>Blockchain Node]
        Network[Stellar Network<br/>Distributed Ledger]
    end
    
    App -->|1. Gerar Keypair| StellarClient
    StellarClient -->|2. Solicitar Funding| Horizon
    Horizon -->|3. Criar Conta| Network
    
    App -->|4. Share Process| StellarClient
    StellarClient -->|5. Criar Transaction<br/>Payment + Memo| StellarClient
    StellarClient -->|6. Assinar com Secret Key| StellarClient
    StellarClient -->|7. Submeter Transaction| Horizon
    Horizon -->|8. Validar| Core
    Core -->|9. Registrar na Blockchain| Network
    Network -->|10. Confirmação| Core
    Core -->|11. Transaction Hash| Horizon
    Horizon -->|12. Resultado| StellarClient
    StellarClient -->|13. Transaction Hash| App
    
    App -->|14. Armazenar Hash no DB| DB[(SQLite)]
    
    style App fill:#c8e6c9
    style StellarClient fill:#ce93d8
    style Horizon fill:#b39ddb
    style Core fill:#9575cd
    style Network fill:#7e57c2
    style DB fill:#ffccbc
```

---

## Resumo dos Fluxos Principais

### 1. Registro → Blockchain Account
```
Cliente/Parceiro → API → Gerar Keypair → Financiar Testnet → Hash Password → Salvar DB
```

### 2. Criar Processo → Criptografia
```
Cliente → API → Gerar Chave AES → Criptografar Conteúdo → Salvar DB (encrypted)
```

### 3. Compartilhar → Blockchain Transaction
```
Cliente → API → Stellar Transaction → Blockchain → Salvar Hash no DB
```

### 4. Acessar → Verificação + Descriptografia
```
Parceiro → API → Verificar Compartilhamento → Descriptografar → Registrar Acesso → Retornar Conteúdo
```

### 5. Notificações → Auditoria
```
Cliente → API → Buscar Acessos com JOIN → Retornar Lista com Detalhes
```

---

## Segurança em Múltiplas Camadas

```mermaid
graph TB
    Request[HTTP Request] --> Layer1{Camada 1:<br/>Autenticação}
    
    Layer1 -->|Pass| Layer2{Camada 2:<br/>RBAC}
    Layer1 -->|Fail| Block1[❌ 401 Unauthorized]
    
    Layer2 -->|Pass| Layer3{Camada 3:<br/>Verificação de Recursos}
    Layer2 -->|Fail| Block2[❌ 403 Forbidden]
    
    Layer3 -->|Pass| Layer4{Camada 4:<br/>Blockchain Verification}
    Layer3 -->|Fail| Block3[❌ 404 Not Found]
    
    Layer4 -->|Pass| Layer5{Camada 5:<br/>Criptografia}
    Layer4 -->|Fail| Block4[❌ 403 Forbidden - Not Shared]
    
    Layer5 -->|Pass| Layer6[Camada 6:<br/>Auditoria]
    Layer5 -->|Fail| Block5[❌ 500 Decryption Error]
    
    Layer6 --> Success[✅ 200 OK]
    
    style Request fill:#e1f5ff
    style Success fill:#81c784
    style Block1 fill:#ef5350
    style Block2 fill:#ef5350
    style Block3 fill:#ff9800
    style Block4 fill:#ef5350
    style Block5 fill:#ef5350
    style Layer1 fill:#fff9c4
    style Layer2 fill:#fff59d
    style Layer3 fill:#fff176
    style Layer4 fill:#ffee58
    style Layer5 fill:#ffeb3b
    style Layer6 fill:#fdd835
```

---

## Conclusão

Estes diagramas ilustram os principais fluxos de informação do sistema NDA Backend:

- **Autenticação e Autorização**: Sistema robusto com RBAC
- **Criptografia**: AES-256-GCM com chaves únicas por processo
- **Blockchain**: Integração com Stellar para registros imutáveis
- **Auditoria**: Trilha completa de todos os acessos
- **Segurança em Camadas**: Múltiplos níveis de proteção

Para mais detalhes, consulte:
- [API Reference](API_REFERENCE.md)
- [Architecture Guide](ARCHITECTURE.md)
- [Quick Start](QUICKSTART.md)
